# Fork patches

This is a fork of [herdrdev/herdr](https://github.com/herdrdev/herdr), maintained
for Earshot's needs. Upstream is a human-facing terminal multiplexer; we drive it
programmatically, and some of its defaults are wrong for that. We carry our own
patches rather than trying to land them upstream.

Upstream does not accept unsolicited pull requests (`CONTRIBUTING.md`); our GitHub
account is not in `.github/APPROVED_CONTRIBUTORS`. That is settled and is not a
thing to retry per patch.

**Closes Quest Log #86.** Keep this file current in the same commit as any patch
change — that is the whole point of it.

## Why a file date is not evidence

On 2026-09-15 a patch was reported as not running because the installed binary's
date predated the commit. It was running: it had been installed three days before
it was committed. A date tells you when a binary was built, not what is in it.

**Record checksums. `shasum -a 256 ~/.local/bin/herdr` is the only honest answer
to "what is running".**

## Base

| | |
|---|---|
| Our base | `58271459` — `feat: support conditional sidebar token hiding (#3925)` |
| Upstream version | 0.9.0 (released 2026-09-07) |
| `origin/master` | `32503f81` (#4148) — **84 commits ahead of our base** |

`origin/master` on our own fork is a current mirror of upstream and has moved on;
our patches sit on the older `58271459`. Staying there is a deliberate decision —
see [Base decision](#base-decision).

Note `upstream/master` as a remote-tracking ref in a local clone goes stale. Run
`git fetch upstream` before believing it.

## Check upstream before writing a patch

**Standing policy.** Kyle, 2026-09-15: *"if we run into any issues with things
like idle states, we can know to check upstream for fixes before we roll our
own."*

We are 84 commits behind upstream by choice, so any bug we hit may already be
fixed in commits we have not taken. That makes "search upstream" the **first**
step of a Herdr debugging session, not a postscript after the patch is written.
It applies most to agent state detection — idle, done, blocked, focus, hook
authority — which is where our patches already cluster and where upstream is most
active.

```sh
git fetch upstream
git log --oneline 58271459..upstream/master -- src/app src/detect src/terminal
git log --oneline 58271459..upstream/master --grep=idle --grep=agent --grep=detect -i
```

Finding a fix upstream does not mean adopting the base. Cherry-picking one commit
is usually cheaper than taking all 84, and either way it beats writing a second
fix for a solved problem. Record what you searched in the issue, including a
negative result — "checked upstream, nothing" is worth writing down, because the
next person will otherwise wonder.

## Patches

All three are on `master`. None are pushed: `origin/master` is 84 commits ahead,
so publishing `master` is blocked behind the base decision below.

### 1. Alternate-screen read truncation — Quest Log #14

`c149faad`, `05d2b867` · `src/server/alt_screen_read.rs`

A `pane read` / `agent read` that asked for more history than fits on screen could
silently return *less* than a smaller request would, labelled complete. The
alternate-screen scroll capture (added upstream in 0.8.0) did not honour the
`truncated: true` contract (added in 0.6.4) on any of its fallback paths.

Three parts: `complete_fallback` rewrites the response with `truncated: true`
instead of forwarding the pre-scroll passive snapshot verbatim; an `Unaligned`
harvest frame keeps the rows already merged from aligned frames rather than
discarding the whole read; and `next_harvest_events` shrinks the wheel batch once
a batch scrolls close to a full page, preserving enough overlap for
`merge_scrolled_up` to align successive frames.

- **Merged:** yes.
- **Installed:** yes, since 2026-09-11. See [Installed](#installed).
- **Upstream:** no. #14 was cancelled as an upstream *report*, not as a fix.
- **Not verified live:** the adaptive batch size against a real Claude Code pane.
  It has run since 2026-09-11 without complaint, which is not a test.

`c149faad` (removing `AGENTS.md` and `.agents/skills`) is carried with it. It is a
harness cleanup, not part of the fix, but it is what made the fix committable —
`AGENTS.md` forbade AI co-author trailers, which had blocked the commit for four
days.

### 2. Completion event suppressed by pane focus — Quest Log #55

`29bab18e` · `src/app/actions.rs`

A turn that ended while its pane was the focused active tab reported `idle`
instead of `done`, so no API event and no plugin hook ever learned the session had
finished. Reproduced on demand: done event in ~11s unfocused, absent after 2.5
minutes focused. Affected every harness.

`pane.seen == false` is the only thing that turns `AgentState::Idle` into
`AgentStatus::Done`, and it was being set from a predicate that guesses whether a
human is looking at the pane. Completion is now latched unconditionally. Sound and
toast suppression is untouched and still focus-driven — it is computed separately
in `record_or_deliver_agent_notification`, so a human watching a pane still gets
no beep and no toast.

- **Merged:** yes.
- **Installed:** yes, 2026-09-15. See [Installed](#installed).
- **Upstream:** no, and it should not go. For a human-facing multiplexer, staying
  quiet about a pane the human is watching is a defensible product choice. It is
  wrong for us because our consumer is a program that is never watching.
- **Not verified live:** **outstanding.** Installed and running since 2026-09-15,
  but nobody has yet observed a focused-pane completion emitting a `done` event on
  it. Until someone runs that test, this patch is verified only by unit tests and
  code reading. The test is the #55 repro in reverse: prompt a session with its
  pane focused, stay on it, and expect the event in about ten seconds rather than
  never.

### 3. Completion suppressed when no client attached — superseded

Branch `fix/done-suppressed-when-no-client-attached` (`6a46ebed`) · **not merged,
and the recommendation is not to merge it.**

The earlier, narrower fix for #55: `outer_terminal_focus` defaults to `None` and
`None != Some(false)`, so with nothing attached Herdr concluded a human was
watching. It adds an `outer_terminal_attached` bool to the predicate.

Patch 2 supersedes it. It removed the predicate from the completion path
altogether, which covers the detached case *and* the focused case the narrower fix
never addressed. What remains of `6a46ebed` is a behavioural change to sound and
toast state while detached — real, since the predicate is still live at
`actions.rs:2021`, `actions.rs:2108` and `headless.rs:3187`, but no longer fixing
anything we have observed, and never exercised against a running server.

It conflicts with patch 2 in two places, both exactly where patch 2 deleted the
lines it edits. Resolving means taking our side in both, which leaves only the
untested part. The branch is kept as the record of that investigation.

## Installed

Currently installed and running: **0.9.0 + patch 1 + patch 2**, built from
`a09edf99`, installed and restarted 2026-09-15.

Verify with **exactly** one of these two commands — the digest you get depends on
which, and they are not comparable:

```sh
shasum -a 256 ~/.local/bin/herdr   # c7e140b218e8d35b77b035a17b688777846f716143b90b5467e1b3b693877d80
shasum -a 1   ~/.local/bin/herdr   # 19c16da4d2ca586061f00bba8c042374db3e2536
```

> **`shasum` with no `-a` is SHA-1, not SHA-256.** Both digests above are correct
> for the same file: 40 hex characters is SHA-1, 64 is SHA-256. If what you get is
> the wrong length to match, you ran the other algorithm — that is not a wrong
> binary. Check the length before concluding anything. This bit someone on
> 2026-09-15, on this page, which is the page written to prevent it.

Digests are written in full throughout. A truncated hash is not comparable and
invites the same mistake this section is about.

**`~/.local/bin/herdr`** — 0.9.0 + patch 1 + patch 2, built from `a09edf99`:

- SHA-256 `c7e140b218e8d35b77b035a17b688777846f716143b90b5467e1b3b693877d80`
- SHA-1 &nbsp;&nbsp;`19c16da4d2ca586061f00bba8c042374db3e2536`

**`~/.local/bin/herdr.bak-0.9.0-altscreen-20260915`** — 0.9.0 + patch 1, the image
that ran 2026-09-11 to 2026-09-15:

- SHA-256 `95ab73bf07d57b95e98302a4ce61da087b18a1b59df5aa39313958e39bb2afcb`
- SHA-1 &nbsp;&nbsp;`96855d935c66b2a9393fbe451a021336c78c2f03`

**`~/.local/bin/herdr.bak-0.9.0-stock-20260910`** — stock 0.9.0:

- SHA-256 `32b53df09872628059c789a69f02a6b8e29e14ddf26711421f3463f70c1aef17`
- SHA-1 &nbsp;&nbsp;`88bc05f68abe28d65536414ce5844b110679bc7f`

`herdr --version` reports `0.9.0` for every one of these. **It does not
distinguish our builds from stock, or from each other.** Hash the file — and see
[Version suffix](#version-suffix-planned) for the plan to fix that.

**An install is not live until the server restarts.** The running process keeps
serving the old image; install and restart are separate events and only the second
changes behaviour.

## Version suffix (planned)

**Do this on the next build.** Not retrofitted to the current one — changing the
version string means another build, install and restart, which is not worth it for
a label.

The problem is above: `herdr --version` says `0.9.0` for stock and for every build
we make, so a checksum is currently the only honest answer to "what is this". It
is also why `session_list`'s `probedVersion` carries no information today.

Proposed form: **`0.9.0+fork.N`** — upstream's version, then a `+` build-metadata
suffix with a counter we bump per installed build. So tonight's binary would have
been `0.9.0+fork.1` and the next is `0.9.0+fork.2`.

Why this form:

- `+`-prefixed build metadata is the semver-designated place for exactly this, and
  semver says it is **ignored for precedence** — `0.9.0+fork.2` compares equal to
  `0.9.0`. Anything doing a version comparison, including upstream's own update
  check, keeps working and will not think we are ahead of or behind a real release.
- It never collides with an upstream version, so adopting a new base is just
  changing the part in front of the `+`.
- A plain counter, not a git SHA: it stays short, it is readable aloud, and the SHA
  is already recorded here against the checksum. Bump it in the same commit that
  records the new digest.

**Constraint: it must not break anything that parses the version.** Before
shipping it, check the update checker, any `probedVersion` consumer in Earshot, and
the CLI's own version handling for exact-string comparisons against `0.9.0` or for
parsers that reject a `+` suffix. A label that breaks the update check is worse
than no label.

## Installing

> **Installing requires restarting the herdr server, and how you restart it
> decides whether every pane dies.** This is Kyle's deliberate act, not a step an
> agent performs.
>
> - `groundctl stop herdr` then `groundctl start herdr` — **panes survive.** Done
>   on 2026-09-15: the server came back as a new pid and all eleven panes,
>   including the agent pane driving the install, were still there.
> - A `bootout`-style restart **SIGKILLs the process group and takes every pane
>   with it.** This is what went wrong in the fleet-wide apply earlier on
>   2026-09-15. Do not reach for it.
>
> The other failure seen on 2026-09-15 was a restart that could not take the socket
> back because an orphaned server still held it. If that happens, stop the orphan
> first, confirm the socket cleared, then start.

Build only, safe at any time:

```sh
cargo build --release          # needs Zig 0.16 for the vendored libghostty-vt
cargo test --bins
cargo fmt --check
cargo clippy --all-targets
```

To install, by hand:

```sh
# 1. back up what is running, and record what it was
cp -p ~/.local/bin/herdr ~/.local/bin/herdr.bak-$(date +%Y%m%d)
shasum -a 256 ~/.local/bin/herdr.bak-$(date +%Y%m%d)

# 2. swap atomically
cp target/release/herdr ~/.local/bin/herdr.new
chmod +x ~/.local/bin/herdr.new
mv ~/.local/bin/herdr.new ~/.local/bin/herdr

# 3. record BOTH digests here, and bump the version suffix in the same commit
shasum -a 256 ~/.local/bin/herdr
shasum -a 1   ~/.local/bin/herdr

# 4. stop, confirm the socket is clear, then start -- panes survive this.
#    Do NOT bootout: that SIGKILLs the process group and kills every pane.
groundctl stop herdr
groundctl start herdr
```

Rollback is the same swap with the backup, and another stop/start.

Leave `~/.config/herdr/` alone. Manifest auto-update is deliberately disabled and
the manifests are pinned; see Quest Log #22.

## Base decision

**Decided 2026-09-15: stay on `58271459`.** Kyle: *"let's stay put. Everything
works, so let's keep it that way."*

The alternative was rebasing onto `origin/master` (`32503f81`, #4148), which was
probed and applies cleanly. It was rejected because installing is the expensive
operation, not rebasing: every install costs a restart of the component that
watches every session, and a rebase would make that restart deliver our patches
*and* 84 unexercised upstream commits at once. If something then misbehaved there
would be no way to attribute it. Keep the risky
operation carrying one variable.

Consequence: `origin/master` stays ahead of what we build, so `master` cannot be
pushed under its own name without rewriting published history. Publish additively
instead — the fork's line of development is on the `fork/master` branch.

This is not permanent. Revisit after an upstream major release, or when
[Check upstream before writing a patch](#check-upstream-before-writing-a-patch)
turns up something we want. When it happens it should be its own job: rebase, full
suite, install, verify, and only then land anything new on top.
