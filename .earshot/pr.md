Title: fix: keep codex working with queued questions

Quest Log #239. Codex 0.159.0 sets its OSC title to `Action Required` for any unanswered async (`request_user_input`) question, even while the screen still shows `Working (…)` and `Queued follow-up inputs / ? 1 question`. The priority-1100 `osc_title_blocked` rule wins over every screen-working rule, so Herdr reports blocked, the observer opens a turn, and Pocket Ace gets a card that can't be dismissed.

**Change**
- New `queued_question_working` rule (priority 1150) in `src/detect/manifests/codex.toml` and `distribution/agent-detection/codex.toml`, both `2026.09.30.1`. It needs a live elapsed-time status line, then the `Queued follow-up inputs` section, then a collapsed `? N question(s)` summary, with no later response marker. Reconnect-failed, `press enter to confirm`, `enter to submit answer/all` and `allow command?` veto it.
- Fixture `src/detect/fixtures/codex-queued-question-working.txt` plus `codex_queued_question_working_fixture` against both manifest copies.
- FORK.md patch 10 (renumbered from 9 on rebase; #201 took 9).

Codex's original commit was `fbc95f10` on `76a0da21`. It is rebased onto current `master` (`987abc70`, includes #3); conflicts were the manifest version line and FORK.md numbering only.

**Checked**: `cargo nextest run detect::`, 98 passed on the rebased branch.

**Not checked**: no live Codex question was issued, so the screen is from the report and the Codex source, not a `herdr agent read --source detection` capture. This only takes effect on the VM once its `~/.config/herdr/agent-detection/codex.toml` override is updated or retired.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_0198MVe5q2wDimejo47ggEgK
