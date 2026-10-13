Draft: false
Title: fix: correct Muse activity and dialog detection for QL-366

QL-366: Muse Code 1.4.6 keeps its composer visible while foreground/background work runs. The old narrow activity region misses the live status and treats the shared model footer as idle, producing false done events. Current command approval choices also differ from the old paired controls.

Recognize both composer glyphs and max effort, require a composer for the footer fallback, detect anchored running task rows in the current tray, and let current human approval controls outrank ordinary activity. Menus/settings publish unknown instead of retaining stale done. An exact `Reviewing approval request` label outranks approval controls; the persistent Auto-review footer alone is never a pending-review signal. Bundled and remotely published Muse manifests stay aligned.

Validation: 36 focused synthetic detection-engine tests; 3,248 in-process Rust tests; Clippy and formatting; 144 Python maintenance tests; six architecture tests; published manifest validator. Nine manual offline checks passed (three observed captures, six explicitly constructed matcher probes). The observed captures change idle to working for active tasks and foreground-with-plan, and idle to blocked for the human approval excerpt.

Known validation limits (Kyle reviewed the PR and accepted these for merge): live settings/menu transitions and judge-pending to human-escalation/auto-approved transitions on Muse 1.4.6, including relevant OSC modes and wrapped/narrow geometry. Settings and the review label are corroborated by the existing 1.4.6 string dump and QL-363 report; Kyle also identified the review label, but those states were not live-captured during this takeover. No invented screen is presented as a capture. See `src/detect/fixtures/muse/README.md` for the committed evidence/handoff.

Limits: `just` and Bun are unavailable; Windows lint reports a missing SDK. An initial broad nextest attempt inadvertently selected server-spawning API tests and failed early on a 0.9.1+fork.5 vs 0.9.1 version assertion. That path was not repeated; subsequent tests exclude integration binaries and headless runtime tests. Read-only process checks found only the two Herdr servers predating this takeover. No live override, manifest reload, deployment, service restart, merge, grant allowlisting, or Muse filesystem policy change was performed.

Prior work: inspected and preserved the clean `feat/ql-366/muse-manifest-1.4.6` worktree and existing trust capture; no code patch or additional commit was there to recover. Main checkout and its untracked file remain intact. New registered branch: `feat/ql-366/muse-detection-codex`, based on fork `origin/master`.
