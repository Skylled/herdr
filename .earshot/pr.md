Draft: false
Title: fix: recover handoff screens and wrapped agy trust prompts (QL-390)

Idle panes with a persisted agent session could survive live handoff with an empty terminal: export explicitly skipped their screen replay. Replay the same bounded primary-screen history used for shell panes regardless of agent-session identity. Request the existing resize/redraw nudge immediately after committed ownership and reader resumption, keeping the later first-client nudge for final client geometry.

Reproduced the empty-visible failure in a disposable server pair: an idle process with persisted pi session metadata survived, while `pane.read` with `source: visible` returned `text: ""`. Its regression test fails on the original implementation and passes with this change. This reproduces the reported symptom without handing off the live server; the original Codex fork.6 → fork.7 incident was not repeated. The API-only alternate-screen/SIGWINCH smoke test passes with the fix, but also recovered through ordinary layout resizing on the original code on this machine, so it does not independently reproduce that incident.

Handoff carries PTYs/processes, metadata and selected input modes, plus at most 8 KiB of recent primary-screen ANSI. It does not transfer complete emulator state or full scrollback. Alternate-screen content is not replayed; recovery depends on the application responding to resize/SIGWINCH. An application that ignores the nudge may remain blank. An old exporting server can still omit agent history on the first upgrade. The importing-server nudge helps only applications that redraw. Document these limits in staged English/Japanese/Chinese session-state docs.

Captured agy 1.3.5's actual first-run dialog using installed Herdr 0.9.1+fork.8 in an isolated named session and new folders. The existing trust rule works at 56 columns, but at 28 columns the question wraps after “of” and classification falls back to idle. Make the question and trust/exit option gates whitespace tolerant. After manifest hot reload, the real dialog is blocked at both widths, and the normal composer remains idle at both widths. Detection text/ANSI evidence is under `/private/tmp/ql390-repro/`. The temporary override, named session and outer panes have been removed. No paid prompt was intended; an initial Ctrl+C cleanup left agy open and a shell command was accidentally submitted as a turn. It was interrupted; both test folders remained empty.

Validation:
- Full Rust suite: 3,549 passed, 6 skipped; includes live-handoff process, rollback and screen coverage.
- Formatting and all-target clippy passed.
- Maintenance: 144 Python tests and 5 release-workflow tests passed; 6 UI architecture tests and all 39 integration-asset tests passed.
- `just check` reached Windows lint, then stopped because the Windows SDK configuration is absent at `~/.local/share/herdr/windows-cross/libc.txt`. No SDK was installed. Native Rust, maintenance, architecture and integration checks are green; this change adds no Windows code or protocol fields.
- Documentation contract tests passed separately.
- Live agy trust/composer smoke checks at 28/56 columns passed; bundled/published manifest validation passed.
- Read-only roundtable review found no blockers.

Overlap: PR 16 changes the same bundled and published agy manifests for background-task composer detection and uses version `2026.10.15.3`. This PR changes only the trust-dialog gates and uses `.4`; retain both rule changes when integrating. No changes to claude.toml, FORK.md or AGENTS.override.md.

Refs QL-390. No release build, installation or live-server handoff performed.
