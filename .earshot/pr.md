Draft: false
Title: fix: detect agy idle with background tasks (QL-390)

Agy 1.3.5 accepts input after a turn finishes while terminal tasks continue, but its extra task rows made the composer fail Herdr's visible-idle gate. Allow timestamped running-task rows and the collapsed `↓ N more` summary between the composer separators. Preserve the active spinner/cancel vetoes and all blocked rules. Keep the published catalog aligned and advance the manifest to 2026.10.15.3.

Validation:
- `just check`: 3,548 Rust tests passed (6 skipped), plus formatting/clippy, 144 maintenance tests, architecture and integration checks. Windows cross-lint is unavailable because this machine has no Windows SDK libc configuration; no SDK was installed.
- `just docs-contract-test`: passed (7 tests). Manifest/catalog validation and `git diff --check` passed.
- Live smoke: installed Herdr 0.9.1+fork.8 and agy 1.3.5, Gemini 3.8 Flash Low, disposable named session with isolated config/state and manifest hot reload. Reproduced stock background-screen failure; verified ordinary idle, multiple running tasks, truncated Plan-mode composer at 41 columns, and collapsed task summary. Verified active spinner plus cancel footer remains working; folder trust and command permission dialogs remain blocked.
- Removed the temporary override and disposable session. The live server and its configuration were untouched. No application build/install/handoff.
