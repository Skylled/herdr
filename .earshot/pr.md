Title: fix: recognize agy 1.2.14 prompt box footer as idle (QL-283)

QL-283. Antigravity CLI 1.2.14 draws a status line under its prompt box: `? for shortcuts` on the left, the model (`Gemini 3.8 Flash · high`) on the right. The fork's `live_prompt_box` rule required the box's bottom rule to be the last line on screen, so the idle screen matched nothing and fell back to `default_known_agent_idle_fallback`. Earshot reports that as `unclassified-screen` and won't brief agy panes.

**Change**
- `live_prompt_box` in `src/detect/manifests/antigravity.toml` and `distribution/agent-detection/antigravity.toml` (both now `2026.10.02.1`) allows at most one footer line under the box, and only one that starts with `? for shortcuts`. Older screens that end at the rule still match. Any other footer, typed prompt text, or output below the box still falls back.
- More idle vetoes, for QL-22's caveat that a looser rule must never call an approval dialog idle. agy 1.2.14's own shipped changelog says approval prompts now name the action, "for example `Run this command?`, `Allow access to this URL?`, or `Allow calling this tool?`". Those titles now veto idle, as do `Approve this action?`, `Send input to this task?`, their option labels (`yes, allow`, `no, deny`, `yes, accept this change`, `yes, send input`), `enter confirm`, and `esc to cancel`.
- New fixtures in `src/detect/fixtures/`, with the account email replaced by `user@example.com`:
  - `agy-1.2.14-idle.txt` and `agy-1.2.14-trust-dialog.txt`: real 1.2.14 captures (`herdr agent read --source detection`, throwaway named session, no prompt sent).
  - `agy-permission-run-command.txt`: the real permission dialog from the Q22 report (2026-09-12).
  - `agy-1.2.1-idle.txt` and `agy-1.2.1-working.txt`: Earshot's real 1.2.1 captures, kept as the older-version baseline.
- Three tests run against both manifest copies: approval dialogs are never idle (written first; it passed on the old rule and still passes), idle with and without the footer, and working with the box still drawn. Like the codex fixture from QL-239, these are QL-283's explicit exception to upstream's synthetic-only screen-test convention.
- FORK.md gets patch 11. The docs/next agents page gets one sentence on the footer.

**Base**: `origin/master` at `6caa9cd0`. It now contains the whole build branch (`codex/rebase-upstream-2026-09-21` at `76a0da21`) plus PR 3 and PR 4, and fork PRs target it. That is also the direction of QL-153.

**Checked**
- The new tests failed before the rule change: the 1.2.14 idle screen was not idle. They pass after it.
- `cargo nextest run`: 3529 of 3530 pass. The one failure is the known `api_ping::ping_over_socket_returns_version`, which expects `0.9.1` and gets `0.9.1+fork.4`. It is unrelated to this change.
- Also passing: the Python maintenance tests (144) and UI hot-path tests (6), `agent_detection_manifest_check.py`, `cargo fmt --check`, and `cargo clippy --all-targets -D warnings`.
- On the real 1.2.14 idle capture, the installed fork.4 server reported fallback with every rule unmatched. This build's `agent explain --file` reports `live_prompt_box`, `visible_idle: true`. The 1.2.14 trust dialog still reads `folder_trust_dialog` / blocked.

**Not checked**
- No 1.2.14 working screen or approval dialog was captured, because that needs a real turn. The 1.2.14 working cases put the 1.2.1 spinner above the 1.2.14 box. The URL, tool and action dialog cases are synthetic, built from the changelog titles and the binary's option strings. If `permission_prompt` doesn't match those dialogs, they fall back to unknown idle (`visible_idle: false`, not briefable). They never read as visible idle.
- Bun tests weren't run (bun isn't installed on this VM), and `just` isn't installed either, so the recipe steps were run by hand.
- Not installed, and no server serving this rule has read a live agy pane. The VM has no `agy.toml` override, and its cached remote agy manifest (`2026.06.24.1`) is older, so a build of this branch serves the new rule directly. Deploying is Kyle's call.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01P7AtzMns5539irxPHhugJd
