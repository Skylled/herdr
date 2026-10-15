Draft: false
Title: fix: recognize agy 1.3.5 idle placeholders (QL-390)

Accept the observed Auto, Plan, and Accept-edits composer placeholders alongside the empty prompt. Keep all 20 vetoes, the frame and footer anchor; bump bundled and published manifests to 2026.10.15.1. No upstream placeholder fix exists at fetched upstream/master 916fcf8d.

Live release-binary verification on a separate named server/socket: all four modes select idle / live_prompt_box; a Gemini Flash low probe transitions to working / cancel_footer_working and back to idle. Tested the bundled manifest with no override. Main server and installed binary untouched; no fork build bump or installation performed.

Validation: 3,536 nextest tests pass, six skipped; one pre-existing fork-version assertion fails (ping_over_socket_returns_version expects 0.9.1, receives 0.9.1+fork.5). Test and version code match origin/master. All 144 maintenance tests, manifest validation, formatting and diff checks pass. See FORK.md for evidence. Kyle installs as fork.7 with live handoff after review.
