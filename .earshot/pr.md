Draft: false
Title: chore: install guest Herdr fork.6 for QL-378

The guest now has `0.9.1+fork.6` installed on disk from source commit `6e06eb672e084263946a80d6dc6810d4a61ec0dd`, based on fork master `433b80da`. This includes PR 8 (Muse/codex, QL-366), PR 9 (Antigravity 1.3.3, QL-378), and PR 10 (fork workflow, QL-378), with existing fork patches preserved and no additional upstream commits.

Bump the display label from fork.5 to fork.6, correct integration-test version assertions to compare API/status with the built CLI, and record the source, full SHA-256/SHA-1, verified dated backup, and install procedure in FORK.md. The previous binary reported fork.5 with a digest that did not match its historical install record.

Validation: release build succeeded; formatting, all-target Clippy, 3,537 tests (6 skipped), 144 maintenance tests, release workflow/architecture/integration checks passed. `just check` stops only at the missing Windows SDK; docs contract checks passed separately (7 tests). This is a macOS install; no Windows validation is claimed.

Kyle completed `server live-handoff` on 2026-10-15. The running server and CLI both report `0.9.1+fork.6`, private protocol 22 remains compatible, and the installed SHA-256 matches `1a6aed86b4bbf260d1d9d4506938e6cf7e273828f9d8414e7c0579705c8172e7`. The Codex override stays because upstream #4563 is not included. No further implementation work remains.

Target: `Skylled/herdr` default branch `master`. Ready for review; agents do not merge.
