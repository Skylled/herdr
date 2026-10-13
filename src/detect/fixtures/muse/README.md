# QL-366 evidence and handoff

The worktree is based on fork `origin/master` at `5a004b99`. This change is
limited to Muse manifests, synthetic engine regressions, and evidence. No
protocol, detection engine, filesystem policy, grant policy, or deployment change.

## Observed screens

Read-only captures on 2026-10-13 from the existing installed Herdr
`0.9.1+fork.5` server. `pane process-info --pane` identified both existing Muse
processes as `muse-bin-1.4.6-R6000.1`, Wonder Full 1.0, high effort.

- `bursar-working.txt` and `.ansi`: `bursar-muse-rk2n`, `w65:p1`, detection source.
  Composer `❯` coexists with `Calling tools`, running typecheck/test/build rows,
  and a persistent `Auto-review` footer. The old eight-line working region misses
  activity above the composer/tray and the footer rule classifies idle. The new
  tray rule requires the current selector and an anchored running tree row below
  the last horizontal border. Activity outvotes the composer regardless of tray
  row count. Historical `Backgrounded`, `Finished`, or assistant prose are not
  activity signals.
- `provost-approval.txt`: exact bottom-buffer excerpt transcribed from the first
  read-only `w68:p1` capture in this takeover, before the dialog disappeared.
  Two numbered Yes/No controls plus `/permissions` distinguish it from prose.
  There is no `Reviewing approval request` marker in this capture. Its human
  escalation interpretation follows the absence of the review marker, not the
  persistent `Auto-review` mode footer.
- `provost-working.txt`: a later read-only capture after that approval disappeared;
  a live Thinking line coexists with the composer and a six-item plan. This also
  demonstrates why the foreground region needs more than eight nonempty lines.
- `prior-trust-1.4.7.txt`: preserved from `/private/tmp/ql366-captures/` left by
  `herdr-muse-vppi`. Its launcher reports an upgrade from 1.4.6 to 1.4.7. It is
  **not** a 1.4.6 fixture and the nested session was not reused or operated.

No captured CLI screen is asserted against a bundled rule in unit tests.
`manifest/tests.rs` instead exercises synthetic region/gate/priority/unknown
publication semantics. These files support manual offline explanations.

## Source evidence and limits

QL-366 and its comments, and `~/Inbox/ql-363-earshot-muse.md`, identify the `❯`
composer, missing `max`, stale `/model` state, and an unclassified tall settings
screen on 1.4.6. The existing `/private/tmp/muse-strings.txt` (created before the
1.4.7 upgrade) contains the exact settings footer fragments `tab/enter change`,
`type to search`, `esc go back`, plus `Reviewing approval request` and separate
`AutomatedReviewing` / `HumanPending` representations. Kyle replied to the
pending-marker question: "'Reviewing approval request' I think".

Those strings corroborate the narrowly scoped settings and pending-review rules;
they are not live screen captures. Only the singular review label is supported.
Counted/plural review text, styling, location, long/wrapped approval controls,
settings transitions, and optional OSC modes remain unverified. The mode footer
alone never means review is pending. A label rule has higher priority than human
approval and ordinary foreground activity; menus publish unknown with no skip
flag so the previous idle/done state is not retained.

Standalone Muse 1.4.7 echo-provider comparison attempts used a separate temporary
config and no model prompts. They did not yield usable menus; the final PTY child
stalled and only those exact created processes were terminated. No guesses were
turned into a claimed live smoke-test result. No override was written to the live
Herdr config, and no manifest reload/deployment was performed.

The initial broad nextest run inadvertently selected API integration tests that
spawn test Herdr servers. It was stopped by fail-fast after a version assertion
(`0.9.1+fork.5` vs `0.9.1`), and was not repeated. Subsequent validation excludes
integration binaries and headless runtime tests. A process audit found only the
two server processes that predated this takeover; both were left untouched.

## Offline checking without a server

Build with `cargo build --locked`. Copy the candidate manifest into an isolated
`XDG_CONFIG_HOME/herdr-dev/agent-detection/muse.toml`, set an isolated
`XDG_STATE_HOME`, clear inherited `HERDR_SOCKET_PATH` and
`HERDR_CLIENT_SOCKET_PATH`, then run the debug binary's:

```text
agent explain --file src/detect/fixtures/muse/bursar-working.txt --agent muse --json
agent explain --file src/detect/fixtures/muse/provost-approval.txt --agent muse --json
agent explain --file src/detect/fixtures/muse/provost-working.txt --agent muse --json
```

File mode evaluates offline; it neither connects to nor launches a server.
Expected states: working, blocked, working. A composer or footer must not hide
running background work; a human approval must outvote ordinary activity.
Constructed label/menu/max probes may verify matcher execution but cannot prove
compatibility with live Muse. Capture those real transitions before claiming
QL-366 fully validated. Draft PR status records that remaining work.

## Prior work preserved

`/Users/kyle/Repos/herdr-worktrees/ql-366-muse-manifest`, branch
`feat/ql-366/muse-manifest-1.4.6`, was clean at `5a004b99` with no extra commits or
patches. Its catalog names `herdr-muse-vppi`; QL-366 records that owner terminated,
and the live agent list had no matching owner/cwd. The worktree and all previous
`/private/tmp/ql366-*` captures were preserved. A fresh registered worktree,
`ql-366-muse-codex`, on `feat/ql-366/muse-detection-codex`, owns this patch.
The main checkout and its untracked `diff.txt` were left intact.
