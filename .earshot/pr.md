Draft: false
Title: fork.8: grow-only headless geometry, agy width and file-access rules (QL-390)

Build and install `0.9.1+fork.8` from merged master `60f78b03`, which includes PR 13's fork.7 record and PR 14's QL-390 fixes: grow-only headless geometry on detach, the agy 1.3.5 mode-prefix idle rule for narrow widths, and the blocked File access rule. Increment only `FORK_BUILD`; Cargo remains 0.9.1 and private protocol remains 22.

The release binary from source commit `265c804a5c5186cc976381a4938a18018b8d08b3` is installed at `/Users/kyle/.local/bin/herdr`, with the verified fork.7 backup at `/Users/kyle/.local/bin/herdr.bak-2026-10-15-fork.7`. Built and installed SHA-256: `63a14cfaa0b1f528c559441c02b7ee7472210bb3c4ba1e8a842b923766f6fa49`. FORK.md records both SHA-256 and SHA-1 digests, the backup, source and installation process, and moves fork.7 to historical (it is live).

Validation: release build with Zig 0.16.0; 3,548 Rust tests pass with --no-fail-fast, six skipped; all 201 maintenance, release workflow, architecture, integration and documentation tests pass (Bun 1.3.14 and just 1.57.0 used from temporary scratch copies, since neither is installed in the guest). Formatting, all-target Clippy and diff checks pass. Windows lint fails as expected because the guest has no Windows SDK; no Windows validation is claimed.

Ready for review into master. No server restart, stop or handoff was performed. Read-only status still reports the running fork.7 server, protocol 22. Kyle performs the fork.8 live handoff from a plain SSH shell after merge; the exact command is recorded in FORK.md.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01VJ67K8fhERp3zN6YeBZDSB
