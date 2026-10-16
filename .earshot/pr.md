Title: fix: constrain codex shortcuts and warnings idle footer (QL-390)

Codex can leave `? for shortcuts` with a right-aligned `⚠ N warning(s) · f2 to view` below the model/cwd footer after a turn completes. Match exactly that optional line in both bundled and published manifests (`2026.10.16.1`); background-terminal and limit notices above the composer already work.

Remove the arbitrary continuation line and unconstrained shortcut suffix. Reject empty footer tails, extra output, paste placeholders, and unsupported wrapping. An unrecognized empty-composer layout returns Unknown before the retained OSC idle title can override a failed screen match. Numbered Yes/No pairs, complete picker navigation footers, and update-menu controls block input. Existing composer dialog gates now handle wrapped wording. Remove the broad transcript vetoes for `[Pasted text`, `enter select`, and `esc close`.

Regression coverage varies warning counts (including singular 1), CRLF, indentation, malformed tails, selected Yes/No options, realistic dialogs, and OSC titles. Remove the hand-split “60-col capture” positives; unsupported wrapping fails closed. Update FORK.md with upstream investigation, behavior, and evidence.

Validation: targeted QL-390 test; final `just ci` (fmt, clippy, 3,552 Rust tests, maintenance, architecture, integration assets); docs contracts; published-manifest check. `just check` was attempted; Windows SDK lint is unavailable and skipped under AGENTS.override.md's explicit exception. Six pre-existing Rust tests are skipped.

Live smoke: installed Herdr 0.9.1+fork.9 and Codex 0.162.1, disposable named session and isolated config/state, candidate manifest hot-reloaded. Idle → real `/review` picker Blocked → Idle verified with detection text/ANSI and explain captures. No model turn was submitted; warning-summary, working, and background variants are regression inputs rather than newly observed live states. Test override/session/pane removed; global override untouched. No installation or handoff.

Review clarification: Herdr's known-agent fallback is Idle, but Earshot explicitly refuses `default_known_agent_idle_fallback`; it does not authorize typing. The global fallback policy remains unchanged. The OSC idle bypass was actionable and is now guarded for unsupported empty-composer layouts.
