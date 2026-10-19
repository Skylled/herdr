Title: feat: recognize and parse Muse and agy permission dialogs (QL-416)

Muse command approvals and agy approvals previously had only generic blockers, so consumers could not identify the request reliably. Add named command/network/tool rules and give the existing agy file rule precedence over generic controls. Bundled and published manifests stay aligned; agy's idle rule is unchanged.

Expose optional `permission_dialog` data through the existing `agent.explain` API and CLI, including offline file mode and text output. It includes the question, displayed target, exact wrapped option labels, selected option and permission scope. Parsing is demand-only; nothing answers dialogs. No frozen codecs or endpoint method shapes change.

Muse review findings addressed (QL-416):
- Locate Muse's trailing option block backwards from its permission footer before scanning options. Commands containing `Navigate`, numbered heredoc/markdown rows, or a row identical to the first approval option retain all their text and never become options. Duplicate option numbers reject extraction.
- All approving URL/tool choices now have `unknown` scope until verified from real captures, including the illustrative always choice. Reconstructed labels no longer imply one-time approval.
- Replace Muse's separate line gates with one contiguous multiline match through the live footer. Accept word-wrapped questions, denial labels and footer text. Disconnected controls and visibly quoted transcript probes cannot claim the named rule.
- Stop target extraction at `Reason:` for every agy kind. Targets remain display-only, retaining wraps/indentation; pre-existing terminal clipping cannot be detected or recovered.
- Test extraction through the winning rule's region, matching production.

Real fixtures reuse Muse 1.4.6's full command approval and agy 1.3.5's command/file screens at 60 and 100 columns. URL/tool fixtures remain explicitly **reconstructed**; their targets, option labels and layout are illustrative. Narrow Muse and adversarial probes are derived variants, not new captures. A synthetic test also covers the CRLF byte-offset panic fixed in the horizontal-rule region helper.

Validation after review fixes:
- `just check`: native formatting/clippy, all 3,557 non-ignored Rust tests, maintenance, architecture and integration asset checks passed. The optional Windows cross-lint stopped because the Windows SDK configuration is absent, per the fork exception; no SDK downloaded.
- `just docs-contract-test`: 7 tests passed separately.
- Four focused QL-416 tests pass, including the new `ql416_muse_command_body_cannot_become_options` and `ql416_agy_reason_is_not_part_of_any_target` regressions, plus scope/region assertions in the existing tests.
- The updated debug detection profile compares previous/candidate rules at 1 and 15 populated panes. At 15 panes the added matching cost was 0.20ms/poll for Muse and 1.66ms/poll for agy; full measurements and caveats are in the fixture README. Extraction adds no polling/render work.

Deferred / Kyle's decision:
- No live blocked smoke for either agent was obtained in QL-416. The earlier disposable Herdr 0.9.1+fork.9 / Muse 1.4.10 echo-provider smoke reloaded the manifest and confirmed idle; a harmless shell-mode probe ran without an approval. No new live smoke was attempted for this review. Current approval transitions, real narrow Muse layouts, and real agy URL/tool dialogs remain unverified.
- Per Muse's process finding, these requested screen-classifying regressions differ from AGENTS.md's engine-only detection-test policy. Kyle needs to resolve that policy/exception question; no policy files were changed.
- Text detection cannot distinguish an identical verbatim UI transcript at the live tail from a real dialog. The contiguous match reduces disconnected/quoted-text false positives without claiming that guarantee.

QL-416. No dialogs answered, deployment, installation-record changes, or FORK.md release entries. The disposable session and outer pane from the earlier smoke were cleaned up.
