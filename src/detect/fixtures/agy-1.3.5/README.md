# agy 1.3.5 detection captures (QL-390)

Captured 2026-10-15 on macOS with installed `herdr 0.9.1+fork.7` and `agy 1.3.5`
(Gemini 3.8 Flash, high), in disposable session `repro-ql390-1115`.
A real `herdr agent attach` client supplied 60 or 100 columns and 32 rows.
Each text fixture comes from `herdr agent read w1:p1 --source detection --format text`.
Only the relevant composer or dialog suffix is retained; startup banner, account
identity, shell output, and prior transcript are omitted. Text inside that suffix
is unchanged. Full text/ANSI evidence and explain JSON are in the local scratch
capture directory `/var/tmp/herdr-ql390/`.

`idle-mode0` is Auto, `mode1` Plan, `mode2` default (empty), and `mode3`
Accept-edits. Shift+Tab cycled modes without submitting a turn. At 60 columns,
Auto and Accept-edits placeholders end in `...`; Plan still fits. After the
hot reload, all four modes were visibly idle at 60 columns.

For permissions, agy's global `allowNonWorkspaceAccess: true` initially bypassed
file dialogs. A test-only HOME reused the existing login/onboarding state and
set `allowNonWorkspaceAccess: false`, leaving global settings unchanged. Asking
the file tool to read `/Users/kyle/Repos/herdr/Cargo.toml` outside the trusted
scratch workspace produced the captured file-access dialog. Before hot reload:
`working` / `cancel_footer_working`; after: `blocked` / `file_access_dialog`.
The captured dialog has three options. The two-option variant in the test is
explicitly derived to cover the report's wording, not another live capture.

After rejecting file access with Escape, requesting the harmless command
`printf ql390-command-probe` produced the Command dialog. It remained blocked
at both widths via the existing paired permission controls. The command was
rejected with Escape. Working turns retain the cancel footer and were observed
as working. No background tasks or optional OSC-specific variants were exercised.

The bundled and published manifests are tested identically. Tests cover LF/CRLF,
all four modes at both widths, busy vetoes, typed drafts, both permission dialogs,
selection movement, missing dialog gates, and a working cancel footer alone.
The temporary manifest override is removed after the live checks; only the
disposable session and its outer pane are stopped/deleted during cleanup.
