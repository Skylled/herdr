# Skylled/herdr fork instructions

This file is owned by the Skylled/herdr fork. Read [AGENTS.md](AGENTS.md) for
upstream engineering, architecture, detection, compatibility, and testing rules.
Codex selects this file instead of AGENTS.md at the same directory, so read that
file explicitly. The following fork process takes precedence over conflicting
upstream maintainer workflow, commit style, and local-machine instructions:

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
