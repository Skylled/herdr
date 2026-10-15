Draft: false
Title: fork.7: agy 1.3.5 prompt rule (QL-390)

Build and install `0.9.1+fork.7` from merged master `9cb52abe`, including PR 11's fork.6 record and PR 12's QL-390 fix for Antigravity 1.3.5 mode placeholders. Increment only `FORK_BUILD`; Cargo remains 0.9.1 and private protocol remains 22.

The release binary from source commit `c5175e455c19e1f000faa492490ed5b29c2beccf` is installed at `/Users/kyle/.local/bin/herdr`, with the verified fork.6 backup at `/Users/kyle/.local/bin/herdr.bak-2026-10-15-fork.6`. Built and installed SHA-256: `4b679ddbcb1ce0e35398eb358af6d8d77aa4b0cbc05679030b875705b2f0d3fd`. FORK.md records both SHA-256 and SHA-1 digests, the backup, source and installation process.

Validation: release build with Zig 0.16.0; 3,537 Rust tests pass with --no-fail-fast and elevated Unix socket access, six skipped; all 201 maintenance, architecture, integration and documentation tests pass. Formatting, all-target Clippy and diff checks pass. Windows validation is unavailable because the guest has no Windows SDK.

Ready for review into master. No server restart, stop or handoff was performed. Read-only status still reports the running fork.6 server, protocol 22. Kyle performs the fork.7 live handoff from a plain SSH shell after merge; the exact command is recorded in FORK.md.
