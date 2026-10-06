Title: fix: recognize claude background shells and named dialogs (QL-321)

Claude sessions with a finished foreground turn and background shells still running now report working from the activity summary directly above the prompt box. Shell counts in the footer and conversation text away from that summary do not claim the new rule. The existing MCP task signal remains separate.

Adds precise blocked rules `file_permission_prompt`, `trust_folder_prompt`, `mcp_server_prompt`, and `mcp_servers_select`. Each requires the dialog header and visible choices/controls below the last horizontal rule. Named rules take precedence over the broad form/permission fallbacks; the several-server checkbox dialog now reports blocked.

Both bundled and distribution Claude manifests advance to `2026.10.06.1`. Includes unchanged Claude Code 2.1.291 captures at 100/60 columns from earshot PR 131 (source commit recorded), the earlier captured background-shell summary, and regression coverage for cursor positions, quoted/partial dialogs, idle/Bash neighbors, shell counts, and existing MCP work. No Earshot files were modified.

Validation:
- `just test-one ql321`: 2 passed, exercising both manifest copies and fixture variants.
- Formatting and Clippy from `just check`: passed.
- `just ci-tests 'not test(ping_over_socket_returns_version)'`: 3,531 Rust tests passed, 7 skipped. Maintenance was rerun with a repaired temporary Bun runtime.
- `just maintenance-test ui-hot-path-architecture-test integration-assets-test`: passed.
- `just docs-contract-test`: 7 passed.
- Manifest catalog validator and `git diff --check`: passed.
- Full `just check` remains blocked by the existing fork version assertion in `ping_over_socket_returns_version`: API returns `0.9.1+fork.5`, test expects Cargo version `0.9.1`. This follows unchanged fork build identity code. Windows lint also cannot run because the Windows SDK configuration is absent; no SDK license/download was attempted.

Validation replays captured screens through Herdr's detector; no new live Claude smoke test or manifest override was performed. Herdr was not installed or restarted. To deploy, merge this fork PR and ship the updated manifest/build (a local manifest override can be loaded with `herdr server reload-agent-manifests`). Earshot PR 131 should use the new precise rule IDs for its dialog checks.

refs QL-321
