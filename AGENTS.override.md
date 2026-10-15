# Skylled/herdr fork instructions

Read [AGENTS.md](AGENTS.md) in full FIRST, before doing any task work. All of
its engineering rules remain in force. Codex automatically loads this file
INSTEAD OF AGENTS.md at this directory, so you must open AGENTS.md explicitly.
Only the process items listed below supersede conflicting upstream instructions;
this fork-owned file does not replace or relax upstream engineering requirements.

Critical engineering rules (a safety net, not a substitute for reading AGENTS.md):

- Keep state separate from runtime, rendering pure, platform behavior isolated,
  and shared runtime/session facts on the server/API side of the client boundary.
- Treat render/layout, PTY parsing, detection, and client fanout as multiplicative
  performance paths. Use narrow accessors, short locks, and hidden-pane early
  exits. Profile 1 and at least 15 populated panes when widening these paths.
- Preserve the frozen endpoint contract: published codecs and method shapes are
  immutable. Add new codecs/capabilities with old fallbacks; never change frozen
  fixtures to bless a wire change. Check stable/preview protocol publication.
- Detection changes require real bottom-buffer text/ANSI evidence, explicit
  invariant AND/OR gates, and the throwaway-session manifest hot-reload/live-smoke
  loop. Never infer agent status from the user viewport. Test the engine with
  synthetic manifests rather than adding bundled-agent screen classifiers.
- Use the just testing recipes and run just check before committing, subject only
  to the Windows SDK exception below. Characterize refactor-risk behavior before
  moving code; preserve identity/state invariants and protocol compatibility.
- Track every local vendored-libghostty patch in vendor/libghostty-vt.patches.md
  and vendor/patches/libghostty-vt/, including verification and removal conditions.
  Recheck patches on vendor updates; indexed patches must reverse-apply cleanly.

Fork process overrides:

- Commit without proposing messages or waiting for alignment. Use lowercase
  conventional commits, no emojis, and no AI co-author lines.
- Kyle owns review and merges in this fork, not Can. Agents never merge PRs.
- Do not wait for Greptile or CodeRabbit reviews in this fork.
- Cite Quest Log work as `QL-<number>` in commits, PR titles/bodies, and replies.
  Do not use `refs #N` or GitHub closing keywords for Quest Log work. Preserve
  genuine GitHub issue and PR references as GitHub references.
- Pushes and PR requests require a live Earshot grant for this repository and
  branch. Run `/usr/local/libexec/earshot-grants/grant-check` in the worktree.
  Use the granted `grant-push` command for Codex, agy, and scripts. Request PRs
  by committing `.earshot/pr.md` in the exact format grant-check prints; never
  use `gh pr create`. After the host confirms the ready PR, remove the request
  file in a follow-up commit and grant-push it. Without a matching live grant,
  stop at local commits and report the limitation.
- Run `just check` before committing. Windows cross-lint is optional when the
  Windows SDK configuration is absent: report the skipped Windows check and run
  the remaining native, maintenance, integration, architecture, and docs checks.
  Do not download an SDK or accept its license as part of routine validation.
- Follow the user's assigned worktree and branch. Do not build an application
  binary, install, deploy, restart the live server, or hand off unless explicitly
  requested. Test compilation remains part of the normal validation suite.

These overrides apply only to Skylled/herdr. They do not authorize changes or
contributions to upstream herdrdev/herdr or relax its contributor guardrail.
