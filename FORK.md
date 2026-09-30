# Fork patches

This is a fork of [herdrdev/herdr](https://github.com/herdrdev/herdr), maintained
for Earshot's needs. Upstream is a human-facing terminal multiplexer; we drive it
programmatically, and some of its defaults are wrong for that. We carry our own
patches rather than trying to land them upstream.

Upstream does not accept unsolicited pull requests (`CONTRIBUTING.md`); our GitHub
account is not in `.github/APPROVED_CONTRIBUTORS`. That is settled and is not a
thing to retry per patch. "Should go upstream" below means *file a bug report*,
never *open a PR*.

**Closes Quest Log #86.** Keep this file current in the same commit as any patch
change — that is the whole point of it.

Last audited **2026-09-26**, against `codex/rebase-upstream-2026-09-21` at
`9136cc19`, on the VM (`Kyles-Virtual-Machine`).

## At a glance

| # | Patch | Quest Log | On build branch | Installed on VM | Should go upstream |
|---|---|---|---|---|---|
| 1 | Alt-screen read truncation | #14 | yes, `618ef5c3` | yes | bug report, maybe |
| 2 | Completion latched regardless of focus | #55 | yes, `489e17b2` | yes | no |
| 3 | Focused client no longer erases a completion | #55 | yes, `1673030a` | yes | no |
| 4 | Completion when no client attached | #55 | **no** (superseded) | no | no |
| 5 | agy permission prompt + trust dialog | #104 | yes, `bbee2342`, `a84932a7` | yes, live | bug report |
| 6 | agy prompt box idle rule | #104 | yes, `90095251` | yes, live | bug report |
| 7 | `+fork.N` version label | #86 | yes, `c345b7c0` + bumps | yes | no |
| 8 | Synthetic blocker-gate engine tests | #104 | yes, `9136cc19` | yes (tests only) | no |
| 9 | Codex 0.158 startup update chooser | #201 | branch `fix/codex-startup-update-201` | **no** (and shadowed by the VM override) | bug report |
| 10 | Codex queued questions during Working | #239 | no, `fix/codex-queued-question` | no | bug report |

Ten entries: eight behavioural fixes (1, 2, 3, 5, 6, 9, 10, and the superseded 4),
one fork-identity feature (7), one test-only commit (8). **Seven are on the build
branch; patches 4, 9 and 10 are not.**

## Why a file date is not evidence

On 2026-09-15 a patch was reported as not running because the installed binary's
date predated the commit. It was running: it had been installed three days before
it was committed. The same day, a patched binary was called "stock" on the
strength of its file date. A date tells you when a binary was built, not what is
in it.

**Record checksums. `shasum -a 256 ~/.local/bin/herdr` is the only honest answer
to "what is running".** And since the rebase, `herdr --version` is not enough
either — see [Version label](#version-label).

## Base

| | |
|---|---|
| Build branch | `codex/rebase-upstream-2026-09-21` (on `origin`), tip `9136cc19` |
| Our base | `5a649142` — `chore: approve ain3sh contributor accounts`, upstream `master` of 2026-09-21 |
| Upstream version at base | `0.9.1` in `Cargo.toml` (`956f23ff release: synchronize metadata for v0.9.1`) |
| Upstream since base | `upstream/master` is 31 commits ahead (`d11c0c34`, #4566, fetched 2026-09-26) |
| Previous base | `58271459` (#3925), upstream 0.9.0 — everything up to `fork.4` on the M6 host |
| `origin/master` | `32503f81` (#4148) — **stale mirror, now 58 commits *behind* our base** |

Upstream's `v0.9.1` tag (`065ef9d6`) is not an ancestor of our base: upstream cuts
stable tags off-branch and merges the release metadata back. Our base is upstream
`master` *after* that merge-back, so it is 0.9.1 plus 37 later master commits
(including `fix: preserve session layouts across shutdown and restore failures
(#4400)` and the navigator perf work). Describe it as "upstream master
2026-09-21", not "0.9.1".

`upstream/master` as a remote-tracking ref in a local clone goes stale. Run
`git fetch upstream` before believing it.

### The 2026-09-21 rebase

The fork was rebased from `58271459` onto `5a649142` on 2026-09-21 (branch named
`codex/…`, so presumably a Codex session). No written decision record exists in
the repo; the 2026-09-15 "stay put" decision in [Base decision](#base-decision)
was superseded in practice by this rebase. Pre-rebase state is preserved on
`origin` as `backup/pre-rebase-2026-09-21` (`8d0446a3`), identical to
`m1-local-master`.

What the rebase changed, found by `git cherry` / `git range-diff` against the
backup:

- **Clean replays:** patches 1, 2, 3, 7, and patch 5's permission-prompt half
  and patch 6 (new SHAs, same diff content).
- **`c149faad` (remove `AGENTS.md` and `.agents/skills`) was dropped.** Both are
  back in the tree. The fork no longer carries that cleanup; upstream's
  `AGENTS.md` is present again.
- **The agy manifest tests were dropped.** Patch 5's trust-dialog test, the
  permission-prompt tests, and patch 6's six targeted tests no longer exist in
  `src/`. That matches upstream's current rule in `CLAUDE.md` ("do not add
  tests that classify captured or invented CLI screens against bundled agent
  rules"). Patch 8 replaced them with synthetic engine tests. Consequence: the
  "revert-and-fail" proofs described under patches 5 and 6 were true for the
  old base and **cannot be rerun on the build branch**; the rules are now
  guarded only by live smoke tests.
- **`FORK.md` itself** was replayed as-is, so until this audit it still
  described the old base.

## Check upstream before writing a patch

**Standing policy.** Kyle, 2026-09-15: *"if we run into any issues with things
like idle states, we can know to check upstream for fixes before we roll our
own."*

Any bug we hit may already be fixed in commits we have not taken. That makes
"search upstream" the **first** step of a Herdr debugging session, not a
postscript after the patch is written. It applies most to agent state
detection — idle, done, blocked, focus, hook authority — which is where our
patches cluster and where upstream is most active.

```sh
git fetch upstream
git log --oneline 5a649142..upstream/master -- src/app src/detect src/terminal src/server
git log --oneline 5a649142..upstream/master --grep=idle --grep=agent --grep=detect -i
```

Two upstream commits since our base touch exactly the files our patches live in,
and must be read before the next rebase:

- `28360107` `fix: distinguish agent completion from startup and session changes
  (#4457)` — 364 lines in `src/app/actions.rs`, 49 in `src/server/headless.rs`.
  That is patches 2 and 3's territory. It may fix, overlap, or conflict with #55.
- `9c96f7dd` `fix: avoid inferring codex idle from terminal output (#4563)` — the
  engine change the VM's codex override (below) is waiting for.

Finding a fix upstream does not mean adopting the base. Cherry-picking one commit
is usually cheaper than a full rebase. Record what you searched in the issue,
including a negative result.

## Patches

SHAs are on the build branch. The pre-rebase SHA (on `backup/pre-rebase-2026-09-21`
and `fork/master`) follows in brackets, because older notes and Quest Log comments
cite those.

### 1. Alternate-screen read truncation — Quest Log #14

`618ef5c3` [was `05d2b867`] · `src/server/alt_screen_read.rs` · branch
`fix/alt-screen-read-truncated-fallback` (`88dc2ea7`, same patch-id, old base)

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

- **On build branch:** yes.
- **Upstream base it applies to:** `5a649142`. Upstream has not touched
  `alt_screen_read.rs` since.
- **Installed:** yes (VM, and M6 host since 2026-09-11 per the old record).
- **Upstream:** a bug report would be legitimate — the `truncated` contract is
  upstream's own. #14 was cancelled as an upstream *report*, not as a fix.
- **Not verified live:** the adaptive batch size against a real Claude Code pane.

### 2. Completion event suppressed by pane focus — Quest Log #55

`489e17b2` [was `29bab18e`] · `src/app/actions.rs` · branch
`fix/emit-done-regardless-of-pane-focus` (`29bab18e`, merged)

A turn that ended while its pane was the focused active tab reported `idle`
instead of `done`, so no API event and no plugin hook ever learned the session had
finished. Reproduced on demand: done event in ~11s unfocused, absent after 2.5
minutes focused. Affected every harness.

`pane.seen == false` is the only thing that turns `AgentState::Idle` into
`AgentStatus::Done`, and it was being set from a predicate that guesses whether a
human is looking at the pane. Completion is now latched unconditionally. Sound and
toast suppression is untouched and still focus-driven.

**This patch alone did not fix #55.** It was necessary but not sufficient; see
patch 3. Installing it and declaring the bug fixed, on the strength of passing
tests and a code-reading argument, was wrong — a live test disproved it in about
thirty seconds.

- **On build branch:** yes.
- **Upstream base it applies to:** `5a649142`. **Next rebase will collide with
  upstream `28360107` (#4457).**
- **Installed:** yes.
- **Upstream:** no. For a human-facing multiplexer, staying quiet about a pane
  the human is watching is a defensible product choice. It is wrong for us
  because our consumer is a program that is never watching.

### 3. Completion erased by a focused client — Quest Log #55

`1673030a` [was `65a92729`] · `src/server/headless.rs` · branch
`fix/completion-latch-erased-by-focused-client` (`65a92729`, merged)

The other half of #55, and the half that actually made the symptom.
`sync_foreground_client_state` called `mark_active_tab_seen()` on **every**
client-state sync while the outer terminal reported focus. That is an ambient
condition, not a user action, and it sets `pane.seen = true` — erasing the
completion moments after patch 2 latched it.

Event-level tests passed while the bug was live because **a hook does not read
status from the event payload.** The plugin context, `session.snapshot` and
`session_list` all re-derive from current pane state, so all of them read `idle`.

Acknowledgement now follows an actual user action (`handle_pane_focus`,
`focus_agent_target`); merely having a focused client attached no longer does.

- **On build branch:** yes, with its regression tests
  (`focused_client_sync_does_not_erase_a_completion` and friends in
  `src/server/headless/tests/mod.rs`).
- **Upstream base it applies to:** `5a649142`. Same #4457 collision warning as
  patch 2.
- **Installed:** yes.
- **Upstream:** no. Same reasoning as patch 2.
- **Known consequence:** a human sitting on a focused pane sees the done marker
  persist until they navigate.
- **Not verified on the fleet:** the acceptance test is still outstanding —
  attach a shell, focus a pane, send a session a one-line question, and the done
  event should reach Ace within about ten seconds. #55 stays open until it is.

### 4. Completion suppressed when no client attached — superseded

Branch `fix/done-suppressed-when-no-client-attached` (`6a46ebed`, on `origin`,
based on the old `58271459`) · **not merged, and the recommendation is not to
merge it.**

The earlier, narrower fix for #55: `outer_terminal_focus` defaults to `None` and
`None != Some(false)`, so with nothing attached Herdr concluded a human was
watching. It adds an `outer_terminal_attached` bool to the predicate.

Patches 2 and 3 supersede it by removing the predicate from the completion path
altogether. What remains is a behavioural change to sound and toast state while
detached, never exercised against a running server, and it conflicts with patch
2 exactly where patch 2 deleted the lines it edits. Kept as the record of that
investigation. It has not been rebased and should not be.

- **Upstream:** no.

### 5. Antigravity reports a false state on its two modals — Quest Log #104

`bbee2342` [was `832d3ebc`], `a84932a7` [was `998e7823`] ·
`src/detect/manifests/antigravity.toml`

Manifest-only: detection rules, no engine change.

**Permission dialog.** The bundled `permission_prompt` rule required the literal
`edit command`; agy 1.2.5 renders `ctrl+g edit/expand command`, and `contains` is
an AND, so the rule never matched and a pane sitting on an unanswered question
reported `idle` / `done`. Replaced with `tab amend` plus a bounded same-line regex
`(?i)\bedit\b[^\n]{0,32}\bcommand\b`.

**Startup trust dialog.** *"Do you trust the contents of this project?"* had no
rule, so it fell through to `default_known_agent_idle_fallback`. Added
`folder_trust_dialog`: `state = "blocked"`, `visible_blocker = true`, gated on
the question and then on the option list, same shape as
`qwen:folder_trust_dialog` and `codex:trust_directory`.

The manifest `version` is bumped on each change, which is load-bearing: a cached
remote manifest wins unless it is strictly older than the bundled one.

- **On build branch:** yes (tests dropped in the rebase; see above).
- **Upstream base it applies to:** `5a649142`. Upstream has not changed
  `antigravity.toml` since.
- **Installed and live on the VM:** `herdr server agent-manifests --json` reports
  `agy bundled 2026.09.17.3`; the cached remote `agy.toml` is `2026.06.24.1`, older,
  so ours wins.
- **Upstream:** worth a bug report — manifest wording drift is upstream's to
  track, and neither rule is fork-specific.
- **Verified live on the M6 host** on 2026-09-17 against a natural dialog
  (`blocked` / `permission_prompt` / `visible_blocker=true`, Earshot agreeing).

### 6. Antigravity prompt box falls through to unknown idle — Quest Log #104

`90095251` [was `454adb1d`] · `src/detect/manifests/antigravity.toml`

agy's empty prompt box (`>` framed by two rules) had no idle rule, so it fell
through to the fallback — `visible_idle: false`, `matched_rule: null` — and
automation that refuses to type into an unconfirmed idle (Earshot) was stuck.
Added `live_prompt_box`: priority 50, `bottom_non_empty_lines(10)`, an anchored
`\z` regex requiring the empty prompt framed by both rules at the foot of the
region, and not-gates vetoing spinners, background-task lines, the permission
dialog and the trust dialog. Manifest `2026.09.17.3`.

- **On build branch:** yes (its six tests dropped in the rebase).
- **Upstream base it applies to:** `5a649142`.
- **Installed and live on the VM** (same manifest check as patch 5).
- **Upstream:** worth a bug report, same reasoning as patch 5.
- **Not verified live:** no record of a real agy prompt box read through a
  server serving this rule. Do that before calling #104's idle half done.

### 7. `+fork.N` version label — Quest Log #86

`c345b7c0` [was `1ad17cb6`] · `src/build_info.rs`, `src/release_notes.rs`,
`src/product_announcements.rs` · plus one `chore: bump fork build to fork.N`
commit per installed build (`dc7c46d5` fork.2, `c90926fa` fork.3, `76a4996d`
fork.4).

See [Version label](#version-label). **Upstream:** no.

### 8. Synthetic blocker-gate tests — Quest Log #104

`9136cc19` · `src/detect/manifest/tests.rs` (+155 lines, tests only)

Covers composed blocker gates (AND/OR/NOT) with synthetic manifests, in place of
the agy screen tests the rebase dropped. Phase 0 of #104. No behaviour change;
it is in the running binary only in the sense that the binary was built from
this commit. **Upstream:** no (would need to be a solicited contribution).

### 9. Codex 0.158 startup update chooser reads as fallback idle — Quest Log #201

Branch `fix/codex-startup-update-201`, off `76a0da21` · `src/detect/manifests/codex.toml`,
`distribution/agent-detection/codex.toml`, `src/detect/manifest/tests.rs`

Codex 0.158 reworded its startup update chooser: `Update available! A -> B` became
`Update available · A → B`, and `Press enter to continue` became
`enter continue · esc skip`. `startup_update` required both old literals, so the
new chooser matched nothing and fell back to `default_known_agent_idle_fallback`.
The rule now accepts either header and either footer; `Update now`,
`Skip until next version` and a footer that ends the region are still required.
Manifest `2026.09.28.1`. Checked offline with `agent explain --file` (isolated
`XDG_*` dirs) against the exact 0.157.1 → 0.158.0 screen, both old layouts, and
negatives. The new synthetic test covers the conjunctive `regex` list with a final
footer alternative.

- **Not installed.** The VM's `codex.toml` override (see
  [VM codex override](#loose-ends)) still has the old rule and shadows the bundled
  manifest, so installing a build is not enough: the override's `startup_update`
  needs the same edit, followed by `herdr server reload-agent-manifests`.
- **Upstream:** bug report (upstream's `startup_update` has the same literals).

### 10. Codex queued questions during Working — Quest Log #239

Branch `fix/codex-queued-question`, based on the fetched build branch
`origin/codex/rebase-upstream-2026-09-21` at `76a0da21` (2026-09-30).
Bundled and published Codex manifests move together to `2026.09.30.1`.

Codex 0.159.0 marks its OSC title `Action Required` whenever an async question
is unanswered, even while its screen still shows `Working (1m 13s • esc to interrupt)` with
`Queued follow-up inputs`, `? 1 question`, and `shift+← to answer`. The existing
OSC blocker outranks all screen-working rules. This emits a false blocked state
to Earshot and produces an unnecessary Pocket Ace session card.

A higher-priority screen rule requires a live elapsed-time status followed by
the queued-input section and a collapsed question count, with no later response
marker. It permits remapped or absent answer/interrupt hints. Real permission
and synchronous question controls veto the exception (including the approval
strings `would you like to`, `do you want to`, `[y/n]`, `yes (y)`, `yes, proceed`,
`esc to cancel`, `enter to select`, `enter to submit`); an unanswered question
without that live working evidence retains normal blocked detection.

Rendering checked against the installed CLI's source tag `rust-v0.159.0`, commit
`687a119f0fcaace47e1f1abcc77cec6c813fd6da`: `bottom_pane/questions.rs` renders
the collapsed count independently of task status; `terminal_title_requires_action`
in `bottom_pane/mod.rs` includes every unanswered async question. Turn completion
and failure recover drafts and clear pending questions (`chatwidget/protocol.rs`,
`bottom_pane/async_questions/state.rs`), so a normal completed turn does not leave
an idle unanswered async-question queue. Synchronous request-user-input dialogs
remain blockers.

The requested fork fixture lives in `src/detect/fixtures/` and tests both manifest
copies against empty, static, spinner, and both blinking Action Required titles.
It checks CRLF, plural counts/countdowns, remapped/absent hints, dynamic activity
labels, stale status, and genuine blockers. This is the explicit #239 exception
to upstream's synthetic-only screen-test convention. No live question was issued.

Validation: the fixture failed before the change with `Blocked` for the Action
Required title, then passed with the new rules. All 97 detection tests, 13
manifest-validator tests, catalog validation, formatting, and Clippy passed.
`just maintenance-test` also passed (144 Python tests and 5 Bun tests).
`just check` stopped at the existing `ping_over_socket_returns_version` assertion:
it expects the package version `0.9.1`, while this fork reports `0.9.1+fork.4`.
Neither the ping test nor version code changed in this patch; the full suite
and later check stages were not completed.

- **On build branch:** no; local fix branch only, not pushed or installed.
- **Upstream searched:** fetched `upstream/master` at `331775c3`; reviewed Codex
  manifest and detection history since `5a649142`, including #4495 and #4563.
  Neither fixes the queued-question OSC title precedence.
- **Deployment:** integrate the local commit into the build branch, build with
  Zig 0.16 (`cargo build --release --locked`), and follow Installing below.
  Bump the next installed fork build and record its source/digests separately.
  This VM's existing local `codex.toml` override still shadows bundled/remote
  rules; Kyle must deliberately update it to this manifest or retire it before
  the change can take effect. It was left untouched by this patch.
- **Restart:** Kyle performs `groundctl stop herdr` then `groundctl start herdr`;
  replacing the binary alone does not change the running server.
- **Upstream:** a reproducible bug report may be appropriate; no upstream PR.

## Installed

### This VM — `Kyles-Virtual-Machine`, arm64, audited 2026-09-26

| | |
|---|---|
| Running process | pid 598, `/Users/kyle/.local/bin/herdr server`, started 2026-09-24 16:42 |
| Process image | `lsof` shows the text segment is inode 422447, which is the current `~/.local/bin/herdr` (not a replaced file) |
| `herdr --version` | `herdr 0.9.1+fork.4` |
| SHA-256 | `a056c207580511a4fcc81b34f8f63b55ff871a9989d008a2f4be87f70dd0f704` |
| SHA-1 | `82576e4ab8de60193ab437ed919a6cfe8d90a3c6` |
| Built from | **`9136cc196bdbd2cf859d4b84f00a9c7506dd2222`** (`codex/rebase-upstream-2026-09-21`) — **established, not inferred** |

How the "built from" was established:

1. `/Users/kyle/Repos/herdr/target/release/herdr` has the same SHA-256 and the
   same mtime (2026-09-24 13:33) as the installed binary.
2. That clone's reflog has exactly one entry: cloned from `Skylled/herdr` at
   2026-09-24 13:31 at `9136cc19`. Nothing else has ever been checked out there,
   and the working tree is clean.
3. **A fresh `cargo build --release --locked` of `9136cc19` into an empty target
   directory on 2026-09-26 produced a byte-identical binary** (same SHA-256).
   That is the proof; 1 and 2 are corroboration.

So the running server has patches 1, 2, 3, 5, 6, 7 and 8, on upstream master
`5a649142`, and not patch 4.

There are **no** `herdr.bak*` files on this VM. There is no rollback binary here;
the digests recorded for the M6 host below do not describe files on this machine.

Detection state on this VM that is *not* in the binary, and changes behaviour:

- `~/.config/herdr/agent-detection/codex.toml` — a local override (created
  2026-09-25): remote codex `2026.09.23.1` plus `osc_title_idle` from the bundled
  manifest. It shadows both bundled and remote codex, including across upgrades,
  until someone deletes it. Drop it once the build contains upstream `9c96f7dd`
  (#4563) or later.
- Most other agents (claude, opencode, …) run from cached **remote** manifests in
  `~/.local/state/herdr/agent-detection/remote/`, downloaded 2026-09-24 before
  `manifest_check = false` was set on 2026-09-25. The binary's bundled manifests
  are not what classifies those panes. `herdr server agent-manifests --json`
  shows the real source per agent.

### The M6 host — not visible from here

Kyle's M6 host has its own herdr. This audit cannot see it. To record it, run
there and paste the output into this section:

```sh
p=$(ps -axo command | awk '/[h]erdr server/{print $1; exit}'); echo "$p"; "$p" --version; shasum -a 256 "$p"; shasum -a 256 ~/.local/bin/herdr*
```

Then compare the SHA-256 against the table below (and against the VM's
`a056c207…` above — if it matches, the host is on the same build).

### Historical record (M6 host, before the rebase)

Kept because these digests are the only way to identify those binaries. All built
from the old base `58271459` and report `0.9.0[+fork.N]`. **None of these files
exist on the VM.** Whether they still exist on the host is unknown.

| Build | Built from | Patches | SHA-256 |
|---|---|---|---|
| `0.9.0+fork.4` | `6b6d29b2` | 1-3, 5, 6, 7 | `5907fb4c6d2092956f38012215f0068be45052c880ff95de7e9cc78104cefdc5` |
| `0.9.0+fork.3` | `8664b312` | 1-3, 5, 7 | `36b809a0e5ba382572119b7842e6f0d48d9fbd618d7fff62ccac49388e90485f` |
| `0.9.0+fork.2` | `85c3edac` | 1-3, 5 (matcher), 7 | `875e733ee9b43e95469a9d0767ca4528be9e9c04ddf4ec90d9d4e38e0545a52f` |
| `0.9.0+fork.1` | `1ad17cb6` | 1-3, 7 | `6cb85acdaa3c28b2d6ff187a22dd5374500f45edc84fa83eba64cc00270110e6` |
| `0.9.0` (unlabelled) | uncommitted at the time | 1, 2 | `c7e140b218e8d35b77b035a17b688777846f716143b90b5467e1b3b693877d80` |
| `0.9.0` (unlabelled) | uncommitted for 3 days | 1 | `95ab73bf07d57b95e98302a4ce61da087b18a1b59df5aa39313958e39bb2afcb` |
| `0.9.0` stock | upstream `v0.9.0` | none | `32b53df09872628059c789a69f02a6b8e29e14ddf26711421f3463f70c1aef17` |

SHA-1 digests for these are in this file's history (`git show 5f9d5c71:FORK.md`).

> **`shasum` with no `-a` is SHA-1, not SHA-256.** 40 hex characters is SHA-1,
> 64 is SHA-256. If what you get is the wrong length to match, you ran the other
> algorithm — that is not a wrong binary. Write digests in full; a truncated hash
> invites the same mistake.

**An install is not live until the server restarts.** The running process keeps
serving the old image. On this VM that is currently moot: the process started
after the binary was written and maps the current inode.

## Version label

Our builds report **`<upstream>+fork.N`** — upstream's `Cargo.toml` version, then
semver build metadata naming ours. `FORK_BUILD` is a constant in
`src/build_info.rs`; it is currently `4`.

### How fork.N is cut

1. Land the patch commit(s) on the build branch.
2. Build, test, and install (see [Installing](#installing)).
3. In a separate commit, `chore: bump fork build to fork.N`, increment
   `FORK_BUILD` — **only for a build that is actually installed**. Builds that
   are never installed do not get a number.
4. In a `docs: record fork.N as installed` commit, record both digests and the
   source SHA here.

In practice steps 2 and 3 have been done in the other order (bump, build that
commit, install), which is fine as long as the recorded SHA is the bump commit.

### The label no longer identifies a build on its own

**`0.9.1+fork.4` (VM) and `0.9.0+fork.4` (M6 host) are different binaries** with
the same N: the rebase changed the upstream half of the label but did not bump
`FORK_BUILD`, and the VM build was installed without a bump commit or a digest
record. Two consequences:

- `herdr --version` distinguishes 0.9.0 from 0.9.1, but within a base N was
  meant to be unique, and nothing stops a future 0.9.1 build with different
  patches also reporting `fork.4`.
- The VM install broke the "one bump per installed build" rule. This audit is
  its digest record, after the fact.

**Recommendation:** make N monotonic across rebases — the next installed build is
`fork.5` regardless of base — and treat "installed without a bump" as the thing
to avoid. Not changed here: this is a docs-only commit.

### Where the label lives, and why not in Cargo.toml

`FORK_BUILD` must **not** move into `Cargo.toml`. `update::Version::parse` splits
`CARGO_PKG_VERSION` on `.` and requires exactly three integer parts, and
`Version::current()` calls `.expect()` on the result — so a `+fork.N` there panics
the update checker. Keeping `BASE_VERSION` clean means every comparison is
untouched and only the display and API strings carry the label. `build_info` has
a test asserting this; if it fails, do not "fix" it by loosening the assert.

`+` is build metadata, which semver ignores for precedence, so a genuine upstream
release still reads as newer.

Checked before shipping `fork.1`: the update checker uses `BASE_VERSION`; the
protocol handshake only logs `server_version`; handoff compares our own
`version()` on both sides; Earshot reports but never compares it. Release notes
and product announcements are keyed by version string, so they use
`build_info::release_version()` (no label) — otherwise they would reappear on
every startup. If you add another version-keyed store, key it on
`release_version()`.

## Installing

> **Installing requires restarting the herdr server, and how you restart it
> decides whether every pane dies.** This is Kyle's deliberate act, not a step an
> agent performs.
>
> - `groundctl stop herdr` then `groundctl start herdr` — **panes survive.**
> - A `bootout`-style restart **SIGKILLs the process group and takes every pane
>   with it.** Do not reach for it.
>
> If a restart cannot take the socket back because an orphaned server still holds
> it, stop the orphan first, confirm the socket cleared, then start.

Build only, safe at any time:

```sh
cargo build --release --locked   # needs Zig 0.16 for the vendored libghostty-vt
just check
```

The release build is reproducible for a given commit and checkout path (proved
for `fork.1` and again on the VM for `9136cc19`), so "which commit is this
binary?" can always be answered by rebuilding the candidate and comparing digests.

To install, by hand:

```sh
# 1. back up what is running, with a dated suffix, and record what it was
cp -p ~/.local/bin/herdr ~/.local/bin/herdr.bak-$(date +%Y%m%d)
shasum -a 256 ~/.local/bin/herdr.bak-$(date +%Y%m%d)

# 2. swap atomically
cp target/release/herdr ~/.local/bin/herdr.new
chmod +x ~/.local/bin/herdr.new
mv ~/.local/bin/herdr.new ~/.local/bin/herdr

# 3. record BOTH digests here, and bump FORK_BUILD in the same change.
shasum -a 256 ~/.local/bin/herdr
shasum -a 1   ~/.local/bin/herdr
herdr --version

# 4. stop, confirm the socket is clear, then start -- panes survive this.
groundctl stop herdr
groundctl start herdr
```

Rollback is the same swap with the backup, and another stop/start.

Leave `~/.config/herdr/` alone unless a Quest Log item says otherwise. See Quest
Log #22 for the manifest pinning and override story.

## Git layout and tracking

Remotes (unchanged by this audit):

- `origin` = `Skylled/herdr` (our fork, `isFork: true`, parent `herdrdev/herdr`)
- `upstream` = `herdrdev/herdr` (fetch only, in practice)

Branches on `origin` that matter:

| Branch | Tip | What it is |
|---|---|---|
| `codex/rebase-upstream-2026-09-21` | `9136cc19` | **the build branch** — what the VM runs |
| `fork/master` | `5f9d5c71` | pre-rebase line of development (old base), fork.4 on the host |
| `backup/pre-rebase-2026-09-21` | `8d0446a3` | `fork/master` + the Q22 report; pre-rebase snapshot |
| `m1-local-master` | `8d0446a3` | identical to the backup above |
| `backup/mac-mini-2026-09-17` | `a6cde880` | fork.1-era snapshot |
| `fix/*` (four) | — | per-patch branches, all on the old base; patches 1-3 merged, 4 not |
| `master` | `32503f81` | stale mirror of upstream (#4148), behind our base |

### Tracking config, as found 2026-09-26 on the VM

- **There is no local `master` and no local `main`** in `/Users/kyle/Repos/herdr`.
  The clone has only `codex/rebase-upstream-2026-09-21`, tracking
  `origin/codex/rebase-upstream-2026-09-21`. So the 2026-09-15 foot-gun (local
  `master` tracking `upstream/master`, where a bare `git push` would aim at
  herdrdev) **does not exist on this VM**. Whether it still exists in the M6
  host's clone is unknown; the old record says it was fixed there on 2026-09-15
  (`branch.master.remote = origin`).
- `origin/HEAD` → `origin/master`, which is the stale mirror. A fresh
  `git checkout master` would create a local master from that, 58 commits behind
  the base we actually build.

**Recommendation (not applied):**

1. Make the build branch a stable name. Push the build branch to `origin` as
   `fork/master` (it is currently the old-base line; replacing it is a
   non-fast-forward, so that is Kyle's call), or better, a new name such as
   `fork/main`, and set it as the fork's default branch on GitHub so
   `origin/HEAD` points at what we run.
2. Leave `origin/master` as a plain mirror of upstream, or delete it; never build
   from it.
3. Any local `master`/`main` should track `origin`, never `upstream`; and to make
   an accidental push to upstream impossible rather than unlikely:
   `git remote set-url --push upstream DISABLED`.
4. Push with explicit refspecs (`git push origin HEAD:refs/heads/<name>`).

## Loose ends

- **`q22-manifest-override-report.md` is tracked at the repo root again.** It was
  deliberately untracked on 2026-09-15 (`79dd4b59`: "analysis of our deployment
  … not herdr source. It belongs on Quest Log #22"), then re-added on 2026-09-20
  (`b3228938`, a Sonnet session) and carried through the rebase. It describes the
  old `0.9.0` binary and the old base. Recommend moving it to Quest Log #22 and
  removing it from the tree in its own commit. Not touched here.
- **Patch 4's branch** is on the old base and unmerged; keep as a record or
  delete, but never merge.
- **`fork/master`, `m1-local-master`, both `backup/*`** are old-base snapshots.
  Once the M6 host is confirmed on a post-rebase build, three of the four are
  redundant.
- **The VM build was never given a bump or a digest commit** (see
  [Version label](#version-label)).
- **The agy screen tests are gone** from the build branch; patches 5 and 6 are
  protected by live smoke tests only.
- **#55 fleet acceptance test** (patch 3) and **agy live prompt-box check**
  (patch 6) are still outstanding.
- **Upstream `28360107` (#4457)** needs reading against patches 2 and 3 before
  the next rebase.
- **VM codex override** must be dropped when the build includes #4563.
- **Patches from Skylled/herdr#2** (`139e4d84` agy/opencode, `c4320480` codex
  `live_prompt_box`) are on the build branch but not inventoried above.

## Base decision

**2026-09-15: stay on `58271459`.** Kyle: *"let's stay put. Everything works, so
let's keep it that way."* The reasoning — installing is the expensive operation,
and a rebase would make one restart deliver our patches *and* many unexercised
upstream commits at once — still holds as a principle.

**2026-09-21: rebased onto `5a649142` anyway**, on
`codex/rebase-upstream-2026-09-21`, and that is what the VM has run since
2026-09-24. No record of who decided or why was found in the repo. The VM was a
fresh install, so the "one variable per restart" concern did not apply there; it
still applies to the M6 host if it moves from `0.9.0+fork.4`.

Next time, make the rebase its own job: rebase, full suite, install, verify, record
here, and only then land anything new on top.
