Title: docs: record fork.5 as installed on the guest (QL-283)

Records `0.9.1+fork.5` as installed and live on the guest. QL-283 is closed.

## Commits

- `85912345` chore: bump fork build to fork.5 (the commit the installed binary was built from)
- `6e5e31ce` docs: draft guest-era install record for fork.5 in FORK.md
- `3a4ec613` docs: record fork.5 as installed on the guest (fills the TBD cells)

## What the record now says

- **Installed binary** `/Users/kyle/.local/bin/herdr`: SHA-256 `e338d2ce262b37e8c6ded2ee0d239822e88a9576d9af8b22aaee732b90257ad5`, SHA-1 `6f0c34941b099d2660e605ca695759d108162cc6`. Identical to the build output of `85912345`.
- **fork.4 backup** `/Users/kyle/.local/bin/herdr.bak-2026-10-02-fork.4`: SHA-256 `a056c207580511a4fcc81b34f8f63b55ff871a9989d008a2f4be87f70dd0f704`, SHA-1 `82576e4ab8de60193ab437ed919a6cfe8d90a3c6`. Identical to the fork.4 record.
- The install log from step 3 of the install steps was not written, so these digests were taken afterwards, read-only, from the files themselves.
- **Restart:** a `launchctl kickstart` of `gui/501/dev.skylled.guest.herdr` with no live panes, not the planned live-handoff. launchd owns the server (pid 67170).
- **Live check:** `session_list` reports `0.9.1+fork.5`. A fresh agy 1.2.14 session read as answerable, then as blocked on a permission prompt. Both correct.
- A stop or start of the guest's server ends pane processes. `herdr server live-handoff` is the way to keep them alive.

## Validation

Docs only. The digests were recomputed against the files on disk. The running server, the installed binary and `~/.config/herdr` were not touched.

## Remaining

None for QL-283. Older Quest Log references in FORK.md still use the legacy `#N` form; they are unchanged here.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HipzqJrGc7mp9Kyh1Bcgmv
