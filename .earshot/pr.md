Draft: false
Title: docs: clarify Skylled fork agent workflow (QL-378)

Add a short `Skylled fork overrides` section near the top of `AGENTS.md` so agents use Kyle's fork workflow: commit without proposing messages first, leave review and merges to Kyle, request PRs through live Earshot grants and `.earshot/pr.md`, and wait for Greptile/CodeRabbit only when those bots are configured on the fork.

All inherited upstream text remains byte-for-byte unchanged for future upstream syncs. `CLAUDE.md` already symlinks to `AGENTS.md`, so it automatically reads the same override. This is a separate documentation-only branch off `origin/master`, independent of the QL-378 detection fix.

Validation: `git diff --check` passes; confirmed the original text is unchanged and the symlink reads the new section. Formatting and Clippy pass. The required `just check` reaches the pre-existing `api_ping::ping_over_socket_returns_version` failure (`0.9.1` expected versus `0.9.1+fork.5` actual), unrelated to this instruction-only change. No runtime, installation, or service change. QL-378.
