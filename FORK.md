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

Patches 1-3 are on `master`; patch 4 is superseded and unmerged. `master` is not
pushed under its own name: `origin/master` is 84 commits ahead, so the fork's line
of development is published as the `fork/master` branch instead. See
[Base decision](#base-decision).

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

**This patch alone did not fix #55.** It is necessary and was not sufficient; see
patch 3. Installing it and declaring the bug fixed, on the strength of 443 passing
tests and a code-reading argument, was wrong — a live test disproved it in about
thirty seconds. The lesson is recorded on #55 and is the reason patch 3 was
developed against a reproduction instead.

- **Merged:** yes.
- **Installed:** yes, 2026-09-15, in `0.9.0+fork.1`.
- **Upstream:** no, and it should not go. For a human-facing multiplexer, staying
  quiet about a pane the human is watching is a defensible product choice. It is
  wrong for us because our consumer is a program that is never watching.

### 3. Completion erased by a focused client — Quest Log #55

`65a92729` · `src/server/headless.rs`

The other half of #55, and the half that actually made the symptom. Patch 2
latches the completion correctly and `PaneAgentStatusChanged` really is emitted
carrying `Done` — that much was confirmed by instrumenting a running server. But
`sync_foreground_client_state` then called `mark_active_tab_seen()` on **every**
client-state sync while the outer terminal reported focus. That is an ambient
condition, not a user action, and it sets `pane.seen = true` — the same bit that
makes `AgentStatus::Done`. The completion was erased moments after being latched.

This is why event-level tests passed while the bug was live: **a hook does not
read status from the event payload.** The plugin context is built by `pane_info`
from current pane state, as are `session.snapshot` and `session_list`. Every one
of them re-derives, so every one of them read `idle`.

Acknowledgement now follows an actual user action. `handle_pane_focus` and
`focus_agent_target` still call `mark_active_tab_seen`, so navigating to a pane
still clears it; merely having a focused client attached no longer does.

- **Merged:** yes.
- **Installed:** yes, 2026-09-15, in `0.9.0+fork.1`.
- **Upstream:** no. Same reasoning as patch 2.
- **Verified against a reproduction, not by reading.** An isolated named herdr
  session driven with no human: pty-attached client, `ESC[I` injected as outer
  focus, `working`/`idle` driven through `pane.report_agent`. Before: focused
  yields `idle`, unfocused yields `done`, repeatably. After: `done` in all three
  conditions. The regression test fails with the fix reverted and passes with it —
  a negative control that patch 2's tests never had.
- **Known consequence:** a human sitting on a focused pane now sees the done marker
  persist until they navigate, rather than it clearing under them.
- **Not verified on the fleet:** **outstanding, and #55 stays open until it is.**
  The harness proves this against an isolated server. The acceptance test is the
  real thing: attach a shell, focus a pane, send a session a one-line question, and
  the done event should reach Ace within about ten seconds.

### 4. Completion suppressed when no client attached — superseded

Branch `fix/done-suppressed-when-no-client-attached` (`6a46ebed`) · **not merged,
and the recommendation is not to merge it.**

The earlier, narrower fix for #55: `outer_terminal_focus` defaults to `None` and
`None != Some(false)`, so with nothing attached Herdr concluded a human was
watching. It adds an `outer_terminal_attached` bool to the predicate.

Patches 2 and 3 supersede it. It removed the predicate from the completion path
altogether, which covers the detached case *and* the focused case the narrower fix
never addressed. What remains of `6a46ebed` is a behavioural change to sound and
toast state while detached — real, since the predicate is still live at
`actions.rs:2021`, `actions.rs:2108` and `headless.rs:3187`, but no longer fixing
anything we have observed, and never exercised against a running server.

It conflicts with patch 2 in two places, both exactly where patch 2 deleted the
lines it edits. Resolving means taking our side in both, which leaves only the
untested part. The branch is kept as the record of that investigation.

## Installed

Currently installed: **`0.9.0+fork.1`** — patches 1, 2 and 3 — built from
`1ad17cb6`, installed 2026-09-15.

Verify with **exactly** one of these two commands — the digest you get depends on
which, and they are not comparable:

```sh
herdr --version                    # 0.9.0+fork.1   <- since fork.1, this is enough
shasum -a 256 ~/.local/bin/herdr   # 6cb85acdaa3c28b2d6ff187a22dd5374500f45edc84fa83eba64cc00270110e6
shasum -a 1   ~/.local/bin/herdr   # fccfb29ae4e088712a9718b23c0d0eea9ccb7906
```

> **`shasum` with no `-a` is SHA-1, not SHA-256.** Both digests above are correct
> for the same file: 40 hex characters is SHA-1, 64 is SHA-256. If what you get is
> the wrong length to match, you ran the other algorithm — that is not a wrong
> binary. Check the length before concluding anything. This bit someone on
> 2026-09-15, on this page, which is the page written to prevent it.

Digests are written in full throughout. A truncated hash is not comparable and
invites the same mistake this section is about.

**`~/.local/bin/herdr`** — `0.9.0+fork.1`, patches 1-3, built from `1ad17cb6`:

- SHA-256 `6cb85acdaa3c28b2d6ff187a22dd5374500f45edc84fa83eba64cc00270110e6`
- SHA-1 &nbsp;&nbsp;`fccfb29ae4e088712a9718b23c0d0eea9ccb7906`

Reproducible: a full `cargo clean -p herdr` and rebuild produced a byte-identical
binary, which is how this artefact was reconciled against the instrumented and
candidate builds that had been sitting in `target/release/` during the #55
investigation.

**`~/.local/bin/herdr.bak-0.9.0-fork0-20260915b`** — 0.9.0 + patches 1 and 2, the
unlabelled build that ran 2026-09-15 19:36 to 21:05:

- SHA-256 `c7e140b218e8d35b77b035a17b688777846f716143b90b5467e1b3b693877d80`
- SHA-1 &nbsp;&nbsp;`19c16da4d2ca586061f00bba8c042374db3e2536`

**`~/.local/bin/herdr.bak-0.9.0-altscreen-20260915`** — 0.9.0 + patch 1, the image
that ran 2026-09-11 to 2026-09-15:

- SHA-256 `95ab73bf07d57b95e98302a4ce61da087b18a1b59df5aa39313958e39bb2afcb`
- SHA-1 &nbsp;&nbsp;`96855d935c66b2a9393fbe451a021336c78c2f03`

**`~/.local/bin/herdr.bak-0.9.0-stock-20260910`** — stock 0.9.0:

- SHA-256 `32b53df09872628059c789a69f02a6b8e29e14ddf26711421f3463f70c1aef17`
- SHA-1 &nbsp;&nbsp;`88bc05f68abe28d65536414ce5844b110679bc7f`

Everything from `fork.1` onward identifies itself: `herdr --version` and the API
`version` field both report `0.9.0+fork.1`. **The three older binaries above all
report a bare `0.9.0` and cannot be told apart except by hashing.**

**An install is not live until the server restarts.** The running process keeps
serving the old image; install and restart are separate events and only the second
changes behaviour.

## Version label

Our builds report **`0.9.0+fork.N`** — upstream's version, then semver build
metadata naming ours. Shipped in `fork.1`, 2026-09-15.

**Bump `FORK_BUILD` in `src/build_info.rs` once per INSTALLED build**, in the same
commit that records the new digest above. Builds that are never installed do not
get a number; the counter tracks what has actually run, which is the question this
file exists to answer. It is a plain counter rather than a git SHA because the SHA
is already recorded against the checksum, and a counter is short enough to read
aloud.

Where it shows up: `herdr --version`, and the API `version` field — so
`session_list` now distinguishes our build from stock without hashing anything.

### Where the label lives, and why not in Cargo.toml

`FORK_BUILD` is a constant in `src/build_info.rs`. It must **not** move into
`Cargo.toml`. `update::Version::parse` splits `CARGO_PKG_VERSION` on `.` and
requires exactly three integer parts, and `Version::current()` calls `.expect()`
on the result — so a `0.9.0+fork.1` there panics the update checker at runtime.
Keeping `BASE_VERSION` a clean `0.9.0` means every comparison is untouched and
only the display and API strings carry the label. `build_info` has a test
asserting this invariant; if it fails, do not "fix" it by loosening the assert.

`+` is build metadata, which semver defines as **ignored for precedence**, so
`0.9.0+fork.1` compares equal to `0.9.0` and a genuine upstream release still
reads as newer.

### What was checked before shipping it

The constraint this file recorded was that the label must not break anything
parsing the version. All four were checked:

- **Update checker** — uses `BASE_VERSION`, not `version()`. Unaffected.
- **Protocol handshake** — carries `server_version`, but only logs it. No equality
  test anywhere in the tree; compatibility gates on `PROTOCOL_VERSION` and codecs.
- **Handoff** — `server/handoff.rs` compares `expected_version` against our own
  `version()`. Both sides are the same binary, so it stays self-consistent.
- **Earshot** — reports the live version but never compares it, and
  `PROBED_HERDR_VERSION` is Earshot's own hardcoded constant, unrelated to what
  the server reports.

**One real breakage was found and fixed rather than shipped.** Release notes and
product announcements are keyed by version string, and a "seen" marker stored for
`0.9.0` would never match `0.9.0+fork.1` — so the notes would have reappeared on
every startup, forever. They now use `build_info::release_version()`, which is
`version()` without our label. 18 tests caught it. If you add another
version-keyed store, key it on `release_version()`, not `version()`.

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

# 3. record BOTH digests here. Bump FORK_BUILD in src/build_info.rs in the
#    same commit -- the counter tracks installed builds, not every build.
shasum -a 256 ~/.local/bin/herdr
shasum -a 1   ~/.local/bin/herdr
herdr --version

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
