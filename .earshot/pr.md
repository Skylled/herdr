Title: feat: recognize and parse Muse and agy permission dialogs (QL-416)

Muse command approvals and agy approvals previously had only generic blockers, so consumers could not identify the request reliably. Add named command/network/tool rules and give the existing agy file rule precedence over generic controls. Keep bundled and published manifests aligned; agy's idle rule is unchanged.

Expose an optional `permission_dialog` through the existing `agent.explain` JSON API and CLI (including offline file mode and text output). It contains the question, displayed command/path/target, exact wrapped option labels, selected option, and one-time/conversation/session/workspace/persistent/deny scope. Parsing happens only on explicit explain requests. Nothing answers dialogs. No frozen codecs or endpoint method shapes changed.

Real fixture regressions reuse Muse 1.4.6's full command approval and agy 1.3.5's command/file screens at 60 and 100 columns. URL/tool fixtures are explicitly **reconstructed**, using known question titles and illustrative controls; those layouts are not verified against real screens. Missing targets return null rather than invented content. Also fix a CRLF byte-offset panic in the existing horizontal-rule region helper exposed by the broader agy suite, with a synthetic regression.

Validation:
- Native formatting/clippy and all 3,555 non-ignored Rust tests passed; maintenance, architecture and integration asset checks passed.
- `just check` reached the optional Windows cross-lint, skipped because the Windows SDK configuration is absent; no SDK downloaded. Docs contract checks run separately.
- Captured-screen tests cover LF/CRLF, selection movement, missing controls, stale dialogs, exact displayed command/option content, scopes and JSON serialization.
- A debug profile compares previous/candidate detection rules with 1 and 15 populated panes; full measurements are in the fixture README. At 15 panes the added matching cost was 0.77ms per polling pass for Muse and 1.80ms for agy. Parsing adds no polling/render work.
- Disposable Herdr 0.9.1+fork.9 / Muse 1.4.10 echo-provider smoke reloaded the manifest and confirmed idle. A harmless shell-mode probe ran without an approval, so it did not verify the new blocked transition. No new agy live smoke or real URL/tool capture. The session and outer pane were cleaned up; no dialogs were answered.

QL-416. No deployment, installation-record changes, or FORK.md release entries.
