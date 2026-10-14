# QL-378: Antigravity CLI 1.3.3 detection evidence

Captured 2026-10-14 UTC (2026-10-13 in the guest). Base: freshly fetched
`Skylled/herdr:master`, `dd7de8eec3215dc6103fdfe5b1608101fa4a0bc0`.

The installed launcher had already advanced to 1.3.4. The updater retained
`~/.local/bin/agy.1791936841754770000.old`, which reports **1.3.3**. A scratch
copy at `/private/tmp/ql378-agy-capture/agy-1.3.3` has SHA-256
`96dd362d5583c60485f5277fa64e43dd64c3ff02a2bf109fd683b99777da370d`.
It was launched outside Herdr in a 100-column, 40-row PTY, with
`AGY_CLI_DISABLE_AUTO_UPDATE=1`, `--effort low`, and inherited Herdr variables
removed. The rendered screen confirmed Gemini 3.8 Flash · low. ANSI output was
rendered with the existing pyte capture environment. No global manifest
configuration, live override, or server reload was used.

## Real captures

- `idle-100.txt`: startup idle, empty `>` between horizontal rules, followed by
  `? for shortcuts` and the model/effort. No verbosity notice was visible.
- `approval-100.txt`: prompt was `Run the harmless shell command sleep 8 then
  printf QL378_OK. Do not read, edit, or create any files.` The real approval
  shows `Requesting permission for:`, `Run this command?`, numbered options,
  and `↑/↓ Navigate · tab Amend · ctrl+g edit/expand command` above `esc to cancel`.
- `working-100.txt`: that harmless command was approved **once**, with no
  persisted permission grant. `⢿ Running command...` coexists with the empty
  composer; the footer says `esc to cancel`. The command completed successfully.
- `idle-after-turn-100.txt`: the command result followed by the empty composer
  and shortcuts footer, with no spinner or interrupt footer.
- `trust-100.txt`: the scratch directory's initial trust dialog, before accepting
  trust for that scratch directory only.
- `help-menu-100.txt`: `/help` opened after the harmless turn; the composer is
  still above the menu. A menu must not acquire visible idle from that composer.
- `live-working-61.txt`: read-only `herdr agent read w6A:p5 --source detection
  --format text` from `pocket-ace-agy-r38i` on the installed Herdr
  `0.9.1+fork.5` server. The same screen was also read as ANSI. This is a
  bottom-buffer snapshot, not the user-visible scrolled viewport. It shows a
  spinner plus an empty composer, with `Global config.json no...` replacing
  the usual footer hint. The spinner vetoes idle even with this warning.

Fixtures remove trailing blank terminal rows and redact the account email to
`user@example.com`. Scratch captures are cropped to the active 1.3.3 banner,
excluding earlier trust-dialog scrollback. The live capture omits the unrelated
user brief above the activity rows. Active controls and their order are unchanged.
Original raw PTY output and uncropped live captures remain local, outside Git.

The existing live pane was never typed into, closed, or restarted. No existing
Herdr server or Muse session was stopped. Only the scratch child processes and
capture supervisors created for this task were terminated during cleanup.

## Notice and derived probes

The reported **“Verbosity now defaults to medium”** notice did not appear in
scratch startup, after `/clear`, or in the retained live scrollback. A second
scratch startup with a separate app-data notice-state file did not produce it
and did not establish full CLI data isolation; that attempt was stopped without
changing the user's notice state. Temporary copied auth material was deleted.
There is **no claimed real capture of the notice-visible state**.

The test's notice text is a constructed probe taken from QL-378, prepended above
real idle, working, and approval captures. Other clearly constructed probes
remove a spinner to check the interrupt footer, truncate the approval to its
controls, add typed input or unknown trailing output, and substitute the observed
permissions warning for the idle footer. These prove matcher behavior, not a
new live observation. The exact warning's full text is corroborated by the
1.3.3 binary; the truncated text is captured live.

## Cache and rules

The live `agent explain` identified cached remote **2026.10.05.1**. That manifest
has no idle rule and is newer than the fork's bundled **2026.10.02.1**, so it
supersedes the already-working basic prompt-box idle rule. Merely loosening the
old bundle would not fix this installed environment.

Both bundled and published agy manifests now use **2026.10.14.1** and identical
rules, so the engine rejects the older cache. Paired modern dialog controls have
higher priority than working. An active cancel footer or braille spinner means
working. Idle requires the empty framed composer at the bottom and either no
footer (older releases), the shortcuts footer, or the specifically observed
permissions warning. Working and dialog controls veto idle. Existing 1.2.x
fixtures remain supported; the old cancel-footer no-idle probe now correctly
permits a positive working match.

CRLF fixture variants exposed incorrect region offsets: `str::lines()` removes
`\r`, so summing line lengths plus one could slice a Unicode border mid-character.
The engine now obtains offsets from the existing line subslices in constant time,
with LF/CRLF/mixed-newline Unicode characterization coverage. No protocol,
persisted identity, or rendering behavior changed.

## Validation

The complete nextest run: **3,536 passed, 1 failed, 6 skipped**, across 16 binaries.
The sole failure remains `api_ping::ping_over_socket_returns_version`, expecting
`0.9.1` rather than the fork's `0.9.1+fork.5`; its assertion was left unchanged.
`just check` reaches this same failure. Formatting and Clippy pass. All remaining
`just test` recipe stages pass: 144 Python maintenance tests, 6 architecture tests,
and 51 Bun tests for workflows, integration assets, and docs. Existing Python
3.13 and `/private/tmp/herdr-ql366-tools` provide the required tooling.

Windows lint was attempted separately and cannot run without the absent SDK
configuration at `~/.local/share/herdr/windows-cross/libc.txt`. Nothing was
installed to satisfy it. The release binary is built with `just build`.

Offline `agent explain --file ... --agent agy --json` uses isolated config/state
directories, including a copy of the real 2026.10.05.1 remote cache. It verifies
that the release build chooses the bundled 2026.10.14.1 rules and reports positive
idle/working/blocked signals for the corresponding real captures, without
connecting to or launching any server. No installation or live hot-reload smoke
test has been performed; Kyle owns installation from a plain terminal.
