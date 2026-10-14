Draft: false
Title: fix: recognise Antigravity 1.3.3 idle prompts (QL-378)

Herdr can report agy idle only by fallback because the cached `2026.10.05.1` manifest has no idle rule and supersedes the fork's older bundled rules. Synchronize the bundled and published agy manifests at `2026.10.14.1` so the empty framed prompt gets a positive idle signal, with working and dialog controls vetoing idle.

- Retain modern permission/question controls, recognize the active cancel footer and spinner, and allow the specifically observed cached-permissions warning as an alternate idle footer.
- Add real Antigravity 1.3.3 idle, returned-idle, working, approval, trust, and help captures, preserving 1.2.x regression coverage. The real verbosity notice could not be reproduced; notice coverage is explicitly a constructed probe above real captures.
- Fix the CRLF Unicode region-offset panic exposed by the fixtures by deriving offsets from existing line subslices, with focused newline/boundary coverage.

Validation: formatting and Clippy pass. Complete nextest: **3,536 passed, 1 failed, 6 skipped**. The only failure is the pre-existing `api_ping::ping_over_socket_returns_version` assertion (`0.9.1` expected versus `0.9.1+fork.5` actual), left unchanged; `just check` reaches that same failure. All remaining suite stages pass: 144 maintenance tests, 6 architecture tests, and 51 Bun tests. Windows lint was attempted but the SDK configuration is absent.

`just build` succeeds. Offline checks of the release binary with an isolated copy of the actual stale cache select the new bundled manifest and positively classify the corresponding real idle/working/blocked captures. Capture provenance and limits are recorded in `src/detect/fixtures/agy-1.3.3/README.md`. No installation, restart, live manifest reload, or input to the existing agy session was performed; installation remains Kyle's from a plain terminal. QL-378.
