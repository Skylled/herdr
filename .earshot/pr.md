Draft: false
Title: fix: detect codex idle with background terminals (QL-390)

Codex CLI sessions that finish a turn while background terminals remain running show background terminal status lines and/or 5h limit warnings above the composer, while newer versions add an optional `? for shortcuts` footer line below the model line. Update `live_prompt_box` in `src/detect/manifests/codex.toml` and `distribution/agent-detection/codex.toml` to support the shortcuts footer and wrapped model lines across varying terminal widths. Retain existing vetoes against active spinners, elapsed turn timers, interrupt hints, queued inputs, and approval dialogs to prevent false idle transitions. Advance the codex detection manifest version to 2026.10.15.1.

Validation:
- Added engine-level test `ql390_codex_background_terminal_idle_and_blockers` in `src/detect/manifest/tests.rs` covering single and multiple background terminal lines, 5h limit warnings with varying indentation and width, wrapped model footers, presence/absence of shortcuts hints, CRLF line endings, clean turn completions, and clean startups across both bundled and published manifests.
- Verified active turns (`Working (12s • esc to interrupt)`, `Thinking (45s • esc to interrupt)`, `Working (12s)`, queued follow-up inputs) remain working (`visible_working: true`, not idle).
- Verified approval and trust dialogs remain blocked (`visible_blocker: true`, not idle).
- Ran full test suite via `cargo nextest run --locked`: 3,552 tests passed, 6 skipped.
- Ran maintenance and manifest checks: `python3.13 scripts/agent_detection_manifest_check.py` passed (`agent detection manifests ok`); 144 maintenance unit tests passed; 6 UI hot-path architecture tests passed; `cargo fmt --check` and `cargo clippy --all-targets --locked -- -D warnings` passed cleanly.
