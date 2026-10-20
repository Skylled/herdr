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
review-fix candidate. Microseconds per complete polling pass, one sample (not release CPU
or a CI threshold):

| Agent | Panes | Previous | Candidate | Delta |
| --- | ---: | ---: | ---: | ---: |
| Muse | 1 | 162.51 | 177.21 | +14.70 |
| Muse | 15 | 2455.08 | 2650.29 | +195.21 |
| agy | 1 | 509.35 | 617.55 | +108.20 |
| agy | 15 | 7628.32 | 9288.86 | +1660.54 |

The cost grows with pane count; no new locks, terminal snapshots, render work,
or parsing were added to polling. Structured extraction is demand-only.


## PR 21 review follow-up (QL-416)

Muse's option scan starts at the last real `1. Yes, proceed (y)` control before
its permission footer, after the displayed `$` command. Derived adversarial
probes contain `Navigate`, numbered heredoc rows, and even a command row equal
to that first control; these are command text, never options. Derived narrow
probes wrap the question, denial label and footer at word boundaries. A single
contiguous match requires the question, command, adjacent options and live tail;
a quoted transcript with visible quote prefixes or disconnected options cannot
claim the named rule. Identical verbatim UI text cannot be distinguished from a
real dialog using text alone.

URL/tool approving choices now all publish `unknown` scope, including the
illustrative always option. No live evidence establishes their approval lifetime.
`Reason:` stops target extraction for all agy kinds. Target text remains
**display-only**: wrapped paths keep their newlines and indentation, and clipping
that happened before the detection snapshot cannot be detected or recovered.
Tests extract from the winning rule's region, as production does.

No additional live blocked smoke was performed during this review follow-up.
The requested screen-classifying fixture tests differ from AGENTS.md's
engine-only detection-test policy; this policy/exception question is explicitly
left for Kyle in PR 21. Policy files are unchanged.
