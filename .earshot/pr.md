Draft: false
Title: fix: recognize agy 1.3.5 idle placeholders (QL-390)

QL-390: Antigravity 1.3.5's `Auto mode: edits & safe commands auto-approved` prompt line was not recognised as idle. Its Plan and Accept-edits modes also add composer placeholders.

Accept the three observed Auto, Plan, and Accept-edits placeholders alongside the default empty prompt. Preserve all 20 vetoes, the prompt frame and footer anchor. Update the bundled and published manifests together to 2026.10.15.1.

Validation after rebasing onto origin/master: all 3,537 nextest tests pass with --no-fail-fast, six skipped; all 201 maintenance, architecture, integration and docs tests pass. Formatting and diff checks pass. All 12 API socket tests also pass independently with elevated access; the initial sandboxed run could not create their sockets.

Last night's live Antigravity 1.3.5 test in an isolated named session verified all four modes (Auto, Plan, Accept-edits and default empty) select idle / live_prompt_box. A Gemini Flash low probe transitions to working / cancel_footer_working and back to idle; the startup trust dialog selects blocked / folder_trust_dialog. The bundled manifest was also verified without an override. See FORK.md for the recorded evidence.

Ready for review into master. Fork.7 build, installation and live handoff remain Kyle's separate step after merge.
