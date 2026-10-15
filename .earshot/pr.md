Draft: false
Title: fork.9: agy background idle and trust rules, claude shell-done, handoff redraw (QL-390)

Build and install `0.9.1+fork.9` from master `b053c92f`, containing merged PRs 15, 16 and 18. The sole source change is `FORK_BUILD` 8 → 9 in source commit `c19e5201cd42b5c662da50d9150a826f53bb0e74`; Cargo stays at `0.9.1` and private protocol stays at 22. FORK.md records the included patches, build provenance, install/backup checksums and manual handoff command.

Release output and installed `/Users/kyle/.local/bin/herdr` match:

- SHA-256: `58111736bbfeab1fcfe8f1421e4b3addc5c9ce3ed09996995022456d42f981f2`
- SHA-1: `47ad28466955b28f937397dd5e3374840aadc141`
- Backup: `/Users/kyle/.local/bin/herdr.bak-2026-10-15-fork.8`, verified against the fork.8 record before an exclusive staging copy and atomic rename.

Validation: release build with Zig 0.16.0; formatting and all-target clippy passed; all 3,551 Rust tests passed (6 skipped), plus 201 maintenance/workflow/architecture/integration/documentation tests. `just check` reached Windows lint and stopped at the expected missing SDK configuration. No SDK was installed. Scratch just/Bun and installed Python 3.13 were used.

The installed CLI reports fork.9. Read-only server status before and after the swap is identical: the live server remains fork.8 on protocol 22. No server stop, restart or handoff was performed. No background agent terminals were started, and all build/check command sessions have finished.

Kyle's manual command, from a plain SSH shell outside Herdr after review/merge:

```sh
/Users/kyle/.local/bin/herdr server live-handoff --import-exe /Users/kyle/.local/bin/herdr --expected-version 0.9.1+fork.9 --expected-protocol 22
```

Refs QL-390.
