# QL-416 permission dialog evidence

Real fixtures reused without alteration:

- `../muse/provost-approval.txt`: Muse 1.4.6-R6000.1, read-only Herdr
  bottom-buffer excerpt transcribed during QL-366. It contains the full wrapped
  command, the Yes/No choices, and `/permissions` footer. See the sibling Muse
  README for provenance and limits. No always choice appears on this screen.
- `../agy-1.3.5/command-{60,100}.txt` and
  `../agy-1.3.5/file-access-{60,100}.txt`: genuine Herdr detection snapshots
  captured in QL-390's disposable session. See that directory's README for
  versions, prompts, hot-reload observations, and cleanup.

`network-reconstructed.txt` and `tool-reconstructed.txt` are **reconstructed**,
not captured screens. Their question titles come from the agy changelog evidence
already recorded beside `agy_approval_dialogs_are_never_idle`; the target,
options and footer layout are illustrative. They test matcher/parser behavior
only, not compatibility with a current CLI. No genuine URL/tool screen is in the
repository. No new paid agent prompts were submitted for QL-416.

Extraction preserves displayed command and option wrapping. It does not recover
an unwrapped shell command from terminal cells. The parser runs only on explicit
explain requests, using the winning rule's region; normal state polling does not
parse or store dialog content. Generic footer-only matches do not fabricate a
parsed target when the header has scrolled out of the detection buffer.

## QL-416 smoke and profiling

Installed Herdr `0.9.1+fork.9`, disposable `repro-ql416-20261019`, Muse
`1.4.10`, `--provider echo --approval-mode untrusted --approval-judge off
--no-session-log --trust-workspace`. Isolated XDG config/state and manifest
copies under `/private/tmp/ql416-repro`; no global overrides were changed.
Manifest reload accepted Muse `2026.10.19.1`; real text/ANSI and explain
confirmed idle. The harmless `! printf ql416-permission-probe` shell-mode probe
completed directly, without an approval. It does **not** validate the new blocked
transition. No dialog was answered. The created named session was stopped and
deleted and its sole outer pane closed. No new agy live smoke was performed;
URL/tool variants and current CLI approval transitions remain unverified.

Supporting debug-profile test `ql416_detection_scale_profile` evaluates populated
100-row transcripts at 1 and 15 panes, comparing the previous rules with the
candidate. Microseconds per complete polling pass, one sample (not release CPU
or a CI threshold):

| Agent | Panes | Previous | Candidate | Delta |
| --- | ---: | ---: | ---: | ---: |
| Muse | 1 | 172.40 | 231.52 | +59.12 |
| Muse | 15 | 2657.02 | 3423.83 | +766.81 |
| agy | 1 | 550.31 | 659.23 | +108.92 |
| agy | 15 | 8171.91 | 9972.68 | +1800.77 |

The cost grows with pane count; no new locks, terminal snapshots, render work,
or parsing were added to polling. Structured extraction is demand-only.
