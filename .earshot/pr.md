Draft: false
Title: fix: recover handoff screens and wrapped agy trust prompts (QL-390)

Supersedes PR 17: rebased onto `origin/master` (`3a4191ee`) after PR 16 merged. The normal grant push to the original branch was rejected as non-fast-forward; no force push was attempted. This replacement uses the user-authorized `fix/ql-390/herdr-handoff-redraw-agy-rebased` branch.

Idle panes with a persisted agent session could survive live handoff with an empty terminal because export skipped their screen replay. Replay the same bounded primary-screen history used for shells regardless of agent-session identity. Request the existing resize/redraw nudge once, after committed ownership and reader resumption. Remove the pending first-client nudge and its state; later client geometry changes use ordinary resizing.

The ownership redraw request is now a flag with no caller-supplied geometry. The PTY actor starts with the inherited/spawned size and records each successfully applied resize, including temporary nudge and restoration sizes. It nudges from that actor-owned state. This handles both a client resize drained alongside the nudge and one already applied before a delayed nudge is queued. The latter reproduced the re-review gap: the old code reverted a real PTY from 32×100 to 24×80. Both deterministic tests pass with the fix and verify rows, columns and pixel dimensions. A resize queued during the nudge drains afterward; failed resizes do not replace the last successful geometry.

A: findings and real-agent smoke evidence

The idle persisted-session integration test reproduces the original symptom in a disposable server pair: the process survives but `pane.read` with `source: visible` returns `text: ""`. It fails before the replay fix and passes afterward. The original live Codex fork.6 → fork.7 server was never touched.

The review follow-up used the checkout's debug test executable as both the exporting and importing server in named session `ql390-real-1732`, with isolated config/state and separate throwaway folders. No prompts were submitted and no model turns were requested. Actual `agent read --source visible` and `--source detection`, in text and ANSI, were saved before/after along with `agent explain`, process info and snapshots.

| Real CLI | Invocation details | Single-nudge smoke result |
| --- | --- | --- |
| Claude Code 2.1.300 | `--permission-mode manual --effort low --model haiku`; persisted Claude session identity present | Empty composer and wrapped unsent draft byte-identical before/after; idle |
| Codex 0.162.1 | `--no-daemon --no-alt-screen --sandbox read-only --ask-for-approval on-request -c model_reasoning_effort="low"`; GPT-6.1-Sol shown | Empty composer and wrapped unsent draft byte-identical before/after; idle |
| agy 1.3.5 | `--effort low`; Gemini 3.8 Flash shown | Empty composer and wrapped unsent draft byte-identical before/after; idle |

Draft tests held geometry at 60×18; empty-composer tests held it at 59×18. Each draft contained `QL390 UNSENT draft for cursor recovery across handoff` followed by `with enough text to wrap onto another composer line`. After handoff, Left/Left then inserting `XY` produced exactly one `composer liXYne` in each real screen. Client reattachment retained all composers; ordinary geometry and dynamic footer changes did not duplicate them. All three original foreground process groups survived every handoff. The asynchronous nudge has an intermediate smaller geometry, so comparisons use settled output, not the immediate ownership acknowledgment. No replay cursor corruption was observed, so replay remains enabled.

These real-agent captures predate the final actor-local geometry refinement; that refinement is covered by both PTY race tests and the full suite.

Raw evidence and hashes: `/private/tmp/ql390-real-1732/`, especially `before-single-nudge/`, `after-single-nudge-settled/`, `cursor-edit/`, `after-single-nudge-reattach/`, `before-empty-final/`, `after-empty-final/`, and `smoke-summary.json`. The disposable session and outer pane were stopped/deleted; all three test folders remained empty.

Handoff carries PTYs/processes, metadata and selected input modes, plus at most 8 KiB of recent primary-screen ANSI. It does not transfer complete emulator state or full scrollback. Alternate-screen content is not replayed; recovery depends on the application responding to resize/SIGWINCH. An old exporter can still omit agent history on its first upgrade, and an application that ignores redraw may remain blank. The synthetic alternate-screen smoke also recovers on the original code through ordinary layout resizing on this machine, so it does not independently reproduce the incident. Staged English/Japanese/Chinese docs describe these limits and the single nudge.

B: agy trust dialog

Captured the actual agy 1.3.5 trust dialog using installed Herdr 0.9.1+fork.8. The existing rule works at 56 columns, but at 28 columns the question wraps after “of” and falls back to idle. Whitespace-tolerant question and trust/exit option gates classify it as blocked at both widths after hot reload; the normal composer remains idle at both widths. Earlier evidence is in `/private/tmp/ql390-repro/`; its override/session/panes were removed. During that earlier run, a shell command was accidentally submitted to agy after an incomplete exit; it was interrupted and test folders remained empty. The new real-agent handoff smokes submitted no prompts.

Validation:
- Full Rust suite: 3,551 passed, 6 skipped, including handoff, rollback, process-I/O and both PTY resize-race tests.
- Formatting, all-target clippy, 144 Python maintenance tests, 5 release-workflow tests, 6 UI architecture tests and all 39 integration-asset tests passed.
- Documentation contracts: 7 passed separately; bundled/published manifest validation passed.
- Read-only roundtable review found no remaining concerns.
- `just check` stopped at Windows lint because `~/.local/share/herdr/windows-cross/libc.txt` is absent. No SDK was installed. Native checks are green; no Windows code or protocol fields changed.

Rebase: PR 16 is merged. The only rebase conflicts were the version line in the bundled and published agy manifests, both resolved to `version = "2026.10.15.4"`. Both copies are byte-identical, and the merged background-task idle rules plus wrapped trust-prompt gates were explicitly checked and retained. No changes to claude.toml, FORK.md or AGENTS.override.md.

Refs QL-390. No release build, installation or live-server handoff performed.
