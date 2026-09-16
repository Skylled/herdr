# Quest Log #22 — Phase 1 local manifest candidates

Authored and self-reviewed on 2026-09-12. **Not activated. Evidence is incomplete; this is not an end-to-end readiness claim.** No reload, restart, stop, kill, remote fetch, pane input, session creation, commit, stash, reset, checkout, or clean was performed.

Only this untracked report is written in the Herdr checkout. The two existing staged files belong to another session and were left alone. Earshot, Claude's manifest, both installed Herdr binaries, and `manifest_check = false` remain unchanged.

## File status and path correction

The two candidate files are written and hash-verified at the requested staging paths:

- `/Users/kyle/.local/state/herdr/agent-detection/opencode.toml`
- `/Users/kyle/.local/state/herdr/agent-detection/agy.toml`

These are **staged files, not discoverable local overrides at that location**. `src/detect/manifest.rs:1129` loads overrides from `config_dir()/agent-detection/<agent>.toml`. `src/detect/manifest_update.rs:417` uses `state_dir()/agent-detection/remote/<agent>.toml` for cached remote manifests. Loose TOML files at the state-directory root are read by neither mechanism.

The actual override directory `/Users/kyle/.config/herdr/agent-detection/` existed but was empty. It was left empty for Phase 1. Kyle must install the reviewed files there before reloading. Do not put these files into `remote/` or use `update-agent-manifests`.

Both files retain all three original bundled rules, with two added rules each and local version `2026.09.12.22`. They replace entire manifests at load time. Claude remains bundled at `2026.09.04.1`.

## Schema and provenance

Read `src/detect/manifest.rs`, its matching/validation/loading code, `manifest_update.rs`, `src/config/io.rs`, `src/cli/agent.rs`, `src/cli/server.rs`, and `/Users/kyle/Repos/earshot/src/toolbox/prompt-signature.ts`. Consulted the project throwaway-repro skill for CLI guidance only; the requested Phase 1 overrides its live reproduction workflow.

- Manifest and gate structs reject unknown fields. TOML literal strings preserve regex backslashes. The engine uses Rust regex syntax, including `\z`, without lookaround.
- Direct `contains`, `regex`, and `line_regex` arrays are AND conditions. Each line regex must match some line. Contains is case-insensitive; regex case folding is explicit.
- `all` is AND; nonempty `any` is OR; any matching child of `not` vetoes its gate.
- Nested gates share the parent rule's region. They cannot select their own region.
- `bottom_non_empty_lines(N)` selects a contiguous suffix starting with the Nth last nonempty line; intervening and trailing blank lines remain.
- The highest-priority matching rule wins; earlier rules win ties. Visible-state flags come from that winner. Idle is priority 50, below working and blocked, and its own vetoes are checked independently.
- No-match behavior remains idle with `default_known_agent_idle_fallback`, `matched_rule: null`, and `visible_idle: false`. Earshot's existing refusal on fallback with an idle-capable ruleset must remain.
- Overrides take precedence over eligible remote and bundled rules. Invalid files may fall back with a warning: source, version, and warnings must be verified.

Executable: `/Users/kyle/.local/bin/herdr`, reporting `herdr 0.9.0`. Read-only API requests targeted `/Users/kyle/.config/herdr/herdr.sock`. `HERDR_SESSION`, `XDG_CONFIG_HOME`, and `XDG_STATE_HOME` were unset. Repo HEAD: `58271459401fa461707c042f7f03867ddcfff3ea`, branch `fix/alt-screen-read-truncated-fallback`.

`herdr server agent-manifests --json` (API method `server.agent_manifests`) returned these entries before and after offline testing:

| agent | source | source_kind | active_version | local_override_shadowing_remote |
|---|---|---|---|---|
| opencode | bundled | bundled | 2026.06.10.1 | false |
| agy | bundled | bundled | 2026.06.24.1 | false |
| claude | bundled | bundled | 2026.09.04.1 | false |

## Findings and honest gaps

1. **OpenCode currently bypasses screen manifests.** `herdr agent explain w7:p8 --json` reports `screen_detection_skipped: true`, reason `full_lifecycle_hook_authority`, no evaluated rules, no manifest source/version, and no matched rule. `src/app/api/agents.rs:264` has that explicit early return. The agent list marks both existing OpenCode panes as skipped. An offline affirmative match proves the rule works when evaluated; it does not prove a reload will supply live matched_rule for these hooked panes. No code or hook settings were changed.
2. **Agy's current permission dialog is already misclassified.** Pane `w7:p9` says `Run this command?` and `ctrl+g edit/expand command`. The bundled rule requires either `do you want to proceed?` or both `tab amend` and `edit command`; neither alternative matches. Its live explanation is fallback idle with no visible blocker. The candidate preserves that rule and adds `local_permission_run_command` using multiple explicit dialog controls as AND gates.
3. **Missing captured states:** neither agent was observed working; OpenCode was not observed blocked. No turns were started and no permissions answered to manufacture evidence. Synthetic checks below verify logic, not actual rendering.
4. OpenCode's idle rule is narrower than Earshot's interim signature: only the observed `Build · ...` footer, a physical blank line before the full empty box, and at least 20 bottom-rule glyphs are accepted. Unobserved modes such as Plan, cropped boxes, narrow geometry, or changed layouts may fall back. The blank-line requirement prevents matching an empty suffix below half-typed input. A typed string identical to the vendor's `Ask anything` placeholder remains visually indistinguishable under this placeholder convention, as in the interim spec.
5. Text regexes cannot authenticate UI controls versus identical output text. Idle matching is bounded to the detection snapshot's foot and requires prompt structure. Existing whole-recent negative rules are preserved. The new vetoes cover the prompt-adjacent regions from the spec. Real transition coverage, unseen dialog layouts, and other terminal widths remain unverified.

## Captured evidence

Read-only captures were saved by approximately `2026-09-12T11:49:33Z`, before installation or activation. All screen blocks here are detection-source reads. Visible-source reads of the same four panes were inspected and matched the respective text. ANSI-format reads of the two Earshot panes returned the same plain text; no additional styling evidence was available.

| agent | requested state | live capture | offline candidate winner | current live result |
|---|---|---|---|---|
| opencode | idle | w7:p8 / earshot-oc-lhlt | live_prompt_box; visible_idle=true | screen detection skipped by hook authority |
| opencode | idle, second footer | wG:p1 / claudegotchi-muse-oc-x7pg | live_prompt_box; visible_idle=true | list says done, detection skipped |
| opencode | working | unavailable | synthetic checks only | not observed |
| opencode | blocked | unavailable | synthetic checks only | not observed |
| agy | idle | wH:p1 / claudegotchi-gemini-agy-4vlz | live_prompt_box; visible_idle=true | default_known_agent_idle_fallback |
| agy | working | unavailable | synthetic checks only | not observed |
| agy | blocked | w7:p9 / earshot-agy-1lwy | local_permission_run_command; visible_blocker=true | default_known_agent_idle_fallback |

### OpenCode idle — exact capture

Command: `herdr agent read w7:p8 --source detection --format text`. Editable body empty; Build/model footer present; no busy footer. Offline winner: `live_prompt_box` (50), idle, fallback null.

```text
     37 - Thirty-seven is prime.
     38 - Thirty-eight is an even number.
     39 - Thirty-nine is three times thirteen.
     40 - Forty marks the end of this count.

     ▣  Build · Muse Spark 1.3 Free · 8.1s

  ┃
  ┃  Reply with exactly: signature-ok
  ┃

     Thought · 29ms

     signature-ok

     ▣  Build · Muse Spark 1.3 Free · 1.6s

  ┃
  ┃
  ┃
  ┃  Build · Muse Spark 1.3 Free OpenCode Zen
  ╹▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
   /Users/kyle/Repos/earshot     10.7K (1%)  ctrl+p commands
```

### OpenCode idle — second exact capture

Command: `herdr agent read wG:p1 --source detection --format text`. Wrapped path/status footer also passes; winner `live_prompt_box`.

```text
        daemon tunnel, the Worker has no /device/register —
        I implemented adapter-only (the sender lives where
        the change is observed) and documented it. Worker-
        side registration would be the follow-up if needed.

     What Kyle must do before this can run: create a
     Firebase project + service-account key (free tier),
     store the JSON outside the repo (0600), set
     CLAUDEGOTCHI_FCM_KEY_PATH in the adapter's Ground
     Control environment and restart it, add google-
     services.json to the watch app build, install the new
     APK, and confirm doctor shows fcm push: enabled (1
     device(s) registered) after first launch.

     ▣  Build · Muse Spark 1.3 Free · 8m 26s

  ┃
  ┃
  ┃
  ┃  Build · Muse Spark 1.3 Free OpenCode Zen
  ╹▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
   /Users/kyle/Repos/Claudegotchi-159.4K (15% ctrl+p
   muse                                       commands
```

### Agy idle — exact capture

Command: `herdr agent read wH:p1 --source detection --format text`. Final three nonempty lines frame a bare `>`; no nearby spinner. Offline winner: `live_prompt_box` (50), idle, fallback null.

```text
      • Save the JSON file on the Mac (e.g. ~/.
      config/claudegotchi/firebase-service-account.json).
  3. Configure the Environment:
      • In ~/Repos/ground-control/services.yaml (or your
      local environment for claudegotchi-adapter), add:
        CLAUDEGOTCHI_FCM_KEY_PATH: "/path/to/firebase-
      service-account.json"

      • Restart the adapter: groundctl restart
      claudegotchi-adapter.
  4. Link the Watch App:
      • In Firebase Console, add an Android app with
      package name com.claudegotchi.wear.
      • Download google-services.json and place it at
      wear/app/google-services.json.
      • In wear/app/build.gradle.kts, apply the Google
      services plugin (or build and launch the app; Google
      Play Services will generate an FCM token, cache it,
      and POST it to POST /device/register on the adapter).


─────────────────────────────────────────────────────────────
>
─────────────────────────────────────────────────────────────
```

### Agy blocked — exact capture

Command: `herdr agent read w7:p9 --source detection --format text`. Offline winner: `local_permission_run_command` (310), blocked, visible_blocker true. The idle rule itself is false.

```text
● ListDir(~/Repos/earshot)
● Bash(git log -n 5 --oneline)
● Bash(find . -maxdepth 3 -not -path '*/...) (ctrl+o to
expand)

Command
─────────────────────────────────────────────────────────────

Requesting permission for:
   find . -maxdepth 3 -not -path '*/.*' -not -path
'./node_modules*' -not -path './dist*' | sort

Run this command?
> 1. Yes, run command
  2. Yes, and always allow in this conversation for commands
that start with 'find . -maxdepth 3 -not -path '*/.*' -not
-path './node_modules*' -not -path './...'
  3. Yes, and always allow for commands that start with 'find
. -maxdepth 3 -not -path '*/.*' -not -path
'./node_modules*' -not -path './...' (Persist to
settings.json)
  4. No, cancel

  ↑/↓ Navigate · tab Amend · ctrl+g edit/expand command
```

### Current live explanations — not candidate results

`herdr agent explain w7:p8 --json`:

```json
{"agent":"opencode","cached_remote_version":null,"evaluated_rules":[],"fallback_reason":null,"local_override_shadowing_remote":false,"manifest_source":null,"manifest_version":null,"matched_rule":null,"remote_update_error":null,"remote_update_status":null,"screen_detection_skip_reason":"full_lifecycle_hook_authority","screen_detection_skipped":true,"skip_state_update":false,"skipped_update_reason":null,"state":"idle","visible_blocker":false,"visible_idle":false,"visible_working":false,"warning":null}
```

`herdr agent explain w7:p9 --json`:

```json
{"agent":"agy","cached_remote_version":null,"evaluated_rules":[{"evidence":{"all_count":0,"any_count":2,"contains":["requesting permission for:"],"line_regex":[],"not_count":0,"regex":[],"region_bytes":898,"region_preview":"● ListDir(~/Repos/earshot)\n● Bash(git log -n 5 --oneline)\n● Bash(find . -maxdepth 3 -not -path '*/...) (ctrl+o to\nexpand)\n\nCommand\n─────────────────────────────────────────────────────────────\n\nRequesting permission for:\n   find . -maxdepth..."},"id":"permission_prompt","matched":false,"priority":300,"region":"whole_recent","state":"blocked"},{"evidence":{"all_count":0,"any_count":0,"contains":[],"line_regex":["^\\s*[\\u2800-\\u28FF]+\\s+\\p{Alphabetic}+\\w*ing\\b"],"not_count":0,"regex":[],"region_bytes":898,"region_preview":"● ListDir(~/Repos/earshot)\n● Bash(git log -n 5 --oneline)\n● Bash(find . -maxdepth 3 -not -path '*/...) (ctrl+o to\nexpand)\n\nCommand\n─────────────────────────────────────────────────────────────\n\nRequesting permission for:\n   find . -maxdepth..."},"id":"spinner_working","matched":false,"priority":100,"region":"whole_recent","state":"working"},{"evidence":{"all_count":0,"any_count":0,"contains":[],"line_regex":["(?i)·\\s*[1-9][0-9]*\\s+task"],"not_count":0,"regex":[],"region_bytes":186,"region_preview":". -maxdepth 3 -not -path '*/.*' -not -path\n'./node_modules*' -not -path './...' (Persist to\nsettings.json)\n  4. No, cancel\n\n  ↑/↓ Navigate · tab Amend · ctrl+g edit/expand command\n"},"id":"background_tasks_working","matched":false,"priority":90,"region":"bottom_non_empty_lines(5)","state":"working"}],"fallback_reason":"default_known_agent_idle_fallback","local_override_shadowing_remote":false,"manifest_source":"bundled","manifest_version":"2026.06.24.1","matched_rule":null,"remote_update_error":null,"remote_update_status":null,"screen_detection_skipped":false,"skip_state_update":false,"skipped_update_reason":null,"state":"idle","visible_blocker":false,"visible_idle":false,"visible_working":false,"warning":null}
```

## Offline verification and self-review

Used the installed binary's `agent explain --file`, which evaluates a file locally without a server request. Process-only temporary XDG config/state directories isolate candidate loading; this is not a reload and does not alter the running server. No new binary or source edits were needed. `just check` was not run in the dirty checkout for this external TOML-only task.

Exact example:

```sh
XDG_CONFIG_HOME=/private/tmp/herdr-q22.qPupgn/config XDG_STATE_HOME=/private/tmp/herdr-q22.qPupgn/state /Users/kyle/.local/bin/herdr agent explain   --file /private/tmp/herdr-q22.qPupgn/evidence/opencode-idle-detection.txt   --agent opencode --json
```

`python3 /private/tmp/herdr-q22.qPupgn/validate.py` passed 25 checks: four captured screens and 21 synthetic cases. Assertions cover candidate source/version, no parser warnings, expected winner and visible flags, and rejection by the idle rule itself in every negative case. TOML comparison verifies all three original rules and aliases are preserved.

Raw captures, inputs, full JSON explanations, scripts, and `validation-results.json` remain under `/private/tmp/herdr-q22.qPupgn/`. This report embeds the principal evidence and complete rule files so it does not depend on temporary storage surviving.

| case | type | resulting state | winner | idle rule matches |
|---|---|---|---|---|
| oc_idle_captured | captured | idle | live_prompt_box | true |
| oc_idle_second_captured | captured | idle | live_prompt_box | true |
| agy_idle_captured | captured | idle | live_prompt_box | true |
| agy_blocked_captured | captured | blocked | local_permission_run_command | false |
| oc_working_interrupt_synthetic | synthetic | working | local_interrupt_footer_working | false |
| oc_working_blocks_synthetic | synthetic | working | progress_bar_working | false |
| oc_working_both_synthetic | synthetic | working | local_interrupt_footer_working | false |
| oc_working_old_hint_synthetic | synthetic | working | interrupt_hint_working | false |
| oc_working_ctrl_c_synthetic | synthetic | working | interrupt_hint_working | false |
| oc_blocked_retained_box_synthetic | synthetic | blocked | permission_required | false |
| oc_blocked_form_synthetic | synthetic | blocked | permission_required | false |
| oc_partial_text_synthetic | synthetic | idle | none: fallback | false |
| oc_placeholder_synthetic | synthetic | idle | live_prompt_box | true |
| oc_missing_bottom_synthetic | synthetic | idle | none: fallback | false |
| oc_unobserved_plan_mode_synthetic | synthetic | idle | none: fallback | false |
| oc_only_box_truncated_synthetic | synthetic | idle | none: fallback | false |
| agy_working_synthetic | synthetic | working | spinner_working | false |
| agy_background_task_synthetic | synthetic | working | background_tasks_working | false |
| agy_partial_text_synthetic | synthetic | idle | none: fallback | false |
| agy_missing_bottom_synthetic | synthetic | idle | none: fallback | false |
| agy_blocked_retained_box_synthetic | synthetic | blocked | local_permission_run_command | false |
| agy_old_permission_synthetic | synthetic | blocked | permission_prompt | false |
| agy_spinner_inset_synthetic | synthetic | working | spinner_working | false |
| empty_oc_synthetic | synthetic | idle | none: fallback | false |
| empty_agy_synthetic | synthetic | idle | none: fallback | false |

Unmatched/partial-input cases still return Herdr's fallback idle; they do not supply affirmative evidence.

### Synthetic working/blocked screens — not live captures

These are constructed counterexamples with the prompt retained, based on the interim spec and bundled controls. They do not satisfy the missing live evidence. Full inputs follow.

OpenCode working: append `■■■■ esc interrupt` below the captured idle screen. Winner: `local_interrupt_footer_working` (120); `progress_bar_working` also matches (100); idle rule false.

```text
     37 - Thirty-seven is prime.
     38 - Thirty-eight is an even number.
     39 - Thirty-nine is three times thirteen.
     40 - Forty marks the end of this count.

     ▣  Build · Muse Spark 1.3 Free · 8.1s

  ┃
  ┃  Reply with exactly: signature-ok
  ┃

     Thought · 29ms

     signature-ok

     ▣  Build · Muse Spark 1.3 Free · 1.6s

  ┃
  ┃
  ┃
  ┃  Build · Muse Spark 1.3 Free OpenCode Zen
  ╹▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
   /Users/kyle/Repos/earshot     10.7K (1%)  ctrl+p commands
  ■■■■ esc interrupt
```

OpenCode blocked: append the bundled `△ Permission required` marker. Winner: `permission_required` (300); idle rule false. This tests a gate, not a captured dialog.

```text
     37 - Thirty-seven is prime.
     38 - Thirty-eight is an even number.
     39 - Thirty-nine is three times thirteen.
     40 - Forty marks the end of this count.

     ▣  Build · Muse Spark 1.3 Free · 8.1s

  ┃
  ┃  Reply with exactly: signature-ok
  ┃

     Thought · 29ms

     signature-ok

     ▣  Build · Muse Spark 1.3 Free · 1.6s

  ┃
  ┃
  ┃
  ┃  Build · Muse Spark 1.3 Free OpenCode Zen
  ╹▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
   /Users/kyle/Repos/earshot     10.7K (1%)  ctrl+p commands
  △ Permission required
```

Agy working: insert `⣻  Generating...` immediately above the captured prompt. Winner: `spinner_working` (100); idle rule false.

```text
      • Save the JSON file on the Mac (e.g. ~/.
      config/claudegotchi/firebase-service-account.json).
  3. Configure the Environment:
      • In ~/Repos/ground-control/services.yaml (or your
      local environment for claudegotchi-adapter), add:
        CLAUDEGOTCHI_FCM_KEY_PATH: "/path/to/firebase-
      service-account.json"

      • Restart the adapter: groundctl restart
      claudegotchi-adapter.
  4. Link the Watch App:
      • In Firebase Console, add an Android app with
      package name com.claudegotchi.wear.
      • Download google-services.json and place it at
      wear/app/google-services.json.
      • In wear/app/build.gradle.kts, apply the Google
      services plugin (or build and launch the app; Google
      Play Services will generate an FCM token, cache it,
      and POST it to POST /device/register on the adapter).


⣻  Generating...

─────────────────────────────────────────────────────────────
>
─────────────────────────────────────────────────────────────
```

## Rule files, verbatim

### opencode.toml

```toml
id = "opencode"
version = "2026.09.12.22"
min_engine_version = 1
updated_at = "2026-09-12T11:49:33Z"
aliases = ["open-code", "herdr:opencode"]

[[rules]]
id = "permission_required"
state = "blocked"
priority = 300
region = "whole_recent"
visible_blocker = true
any = [
  { contains = ["△ Permission required"] },
  { contains = ["esc dismiss"], any = [{ contains = ["enter confirm"] }, { contains = ["enter submit"] }, { contains = ["enter toggle"] }], all = [{ any = [{ contains = ["↑↓ select"] }, { contains = ["⇆ tab"] }] }] },
]

[[rules]]
id = "interrupt_hint_working"
state = "working"
priority = 110
region = "whole_recent"
visible_working = true
any = [
  { contains = ["esc to interrupt"] },
  { contains = ["ctrl+c to interrupt"] },
  { contains = ["press esc to interrupt"] },
  { line_regex = ['(?i).*opencode.*esc (again to )?interrupt'] },
]

[[rules]]
id = "progress_bar_working"
state = "working"
priority = 100
region = "whole_recent"
visible_working = true
regex = ['(■|⬝){4,}']

# Q22 local candidate. The blank physical line before the box is deliberate:
# do not match an empty suffix of a box containing partially typed input.
# Build is the only mode footer observed in this Phase 1 capture.
[[rules]]
id = "local_interrupt_footer_working"
state = "working"
priority = 120
region = "bottom_non_empty_lines(10)"
visible_working = true
regex = ['(?i)\besc[ \t]+(?:again[ \t]+to[ \t]+)?interrupt\b']

[[rules]]
id = "live_prompt_box"
state = "idle"
priority = 50
region = "bottom_non_empty_lines(10)"
visible_idle = true
regex = ['(?m)^[ \t]*\n(?:[ \t]*┃[ \t]*(?:Ask anything[^\n]*)?\n)+[ \t]*┃[ \t]+Build[ \t]+·[^\n]+\n[ \t]*╹▀{20,}[^\n]*(?:\n[^\n┃╹]*)*\z']
not = [
  { regex = ['[■⬝]{4,}'] },
  { regex = ['(?i)\besc[ \t]+(?:again[ \t]+to[ \t]+|to[ \t]+)?interrupt\b'] },
  { contains = ["ctrl+c to interrupt"] },
  { contains = ["△ Permission required"] },
  { contains = ["esc dismiss"] },
]
```

### agy.toml

```toml
id = "agy"
version = "2026.09.12.22"
min_engine_version = 1
updated_at = "2026-09-12T11:49:33Z"
aliases = ["antigravity", "antigravity-cli"]

[[rules]]
id = "permission_prompt"
state = "blocked"
priority = 300
region = "whole_recent"
visible_blocker = true
contains = ["requesting permission for:"]
any = [
  { contains = ["do you want to proceed?"] },
  { contains = ["tab amend", "edit command"] },
]

[[rules]]
id = "spinner_working"
state = "working"
priority = 100
region = "whole_recent"
visible_working = true
line_regex = ['^\s*[\u2800-\u28FF]+\s+\p{Alphabetic}+\w*ing\b']

[[rules]]
id = "background_tasks_working"
state = "working"
priority = 90
region = "bottom_non_empty_lines(5)"
visible_working = true
line_regex = ['(?i)·\s*[1-9][0-9]*\s+task']

# The live 2026-09-12 permission dialog says "Run this command?" and
# "edit/expand command"; the bundled permission_prompt does not match it.
[[rules]]
id = "local_permission_run_command"
state = "blocked"
priority = 310
region = "bottom_non_empty_lines(24)"
visible_blocker = true
contains = ["requesting permission for:", "run this command?", "tab amend"]
line_regex = [
  '^[ \t]*(?:>[ \t]*)?[1-9]\.[ \t]+Yes, run command[ \t]*$',
  '^[ \t]*(?:>[ \t]*)?[1-9]\.[ \t]+No, cancel[ \t]*$',
]

[[rules]]
id = "live_prompt_box"
state = "idle"
priority = 50
region = "bottom_non_empty_lines(8)"
visible_idle = true
regex = ['(?m)^[ \t]*─{20,}[ \t]*\n(?:[ \t]*\n)*[ \t]*>[ \t]*\n(?:[ \t]*\n)*[ \t]*─{20,}[ \t]*(?:\n[ \t]*)*\z']
not = [
  { regex = ['[\u2800-\u28FF]+\s+\p{Alphabetic}+\w*ing\b'] },
  { regex = ['(?i)·\s*[1-9][0-9]*\s+task'] },
  { contains = ["requesting permission for:"] },
  { contains = ["run this command?"] },
  { contains = ["do you want to proceed?"] },
  { contains = ["tab amend"] },
]
```

### SHA-256

- `opencode.toml`: `dd27f645845f75fdd8744c103fc284f2e270e06384a0b14784846c303e2e9b20`

- `agy.toml`: `84ee58cf600576ab54d383bb241d322671e2ace4d5061cad439544dd1f551992`

## Kyle's activation procedure — supplied only, not executed

Phase 1 stops with missing live captures and the OpenCode hook-authority issue unresolved. Before relying on this for unattended typing, Kyle should capture/review those states and decide how to handle the bypass. Keep Earshot's interim code.

The staged state-directory files cannot be activated by reload alone. Install them into the actual config override directory first. This command checks the exact reviewed hashes and refuses to overwrite existing overrides or symlinks. If preflight fails, stop and inspect.

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib
src = Path('/Users/kyle/.local/state/herdr/agent-detection')
dst = Path('/Users/kyle/.config/herdr/agent-detection')
expected = {
    'opencode.toml': 'dd27f645845f75fdd8744c103fc284f2e270e06384a0b14784846c303e2e9b20',
    'agy.toml': '84ee58cf600576ab54d383bb241d322671e2ace4d5061cad439544dd1f551992',
}
contents = {}
for name, digest in expected.items():
    target = dst / name
    if target.exists() or target.is_symlink():
        raise SystemExit(f'STOP: existing override: {target}')
    data = (src / name).read_bytes()
    if hashlib.sha256(data).hexdigest() != digest:
        raise SystemExit(f'STOP: candidate hash mismatch: {name}')
    contents[name] = data
dst.mkdir(parents=True, exist_ok=True)
for name, data in contents.items():
    with (dst / name).open('xb') as f:
        f.write(data)
print('Installed files only; no reload performed.')
PY
```

Only after that succeeds, Kyle's exact activation command for the inspected server is:

```sh
HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr server reload-agent-manifests
```

This maps to `server.reload_agent_manifests`. Do not run `update-agent-manifests`, re-enable `manifest_check`, or restart the server.

Read-only verification:

```sh
HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr server agent-manifests --json

HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr agent explain w7:p8 --json

HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr agent explain wH:p1 --json

HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr agent explain w7:p9 --json
```

Re-identify panes with `agent list` if the session changed. Expected candidate metadata: `source_kind: "local override"`, source under `/Users/kyle/.config/herdr/agent-detection/`, `active_version: "2026.09.12.22"`, no warnings. Claude must remain bundled `2026.09.04.1`.

Agy's captured idle/blocked screens should yield the respective winners if still displayed. Hooked OpenCode may retain null matched_rule because evaluation is skipped; that is an unresolved result, not a successful affirmative-idle rollout. Check real working and blocked transitions under Kyle's supervision before retiring Earshot's interim signatures.

## Exact rollback — supplied only, not executed

**Phase 1 as delivered:** no live override is installed or activated; no runtime rollback is needed. The loose staged files remain inert.

**After installation/reload:** move only these exact reviewed candidates out of the loader's filenames, retaining them as `.toml.q22-disabled`. This refuses modified files, symlinks, or existing rollback destinations, and handles partial installation. Both original override paths were absent. No deletion, binary restoration, config.toml change, or Claude change is involved.

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib
directory = Path('/Users/kyle/.config/herdr/agent-detection')
expected = {
    'opencode.toml': 'dd27f645845f75fdd8744c103fc284f2e270e06384a0b14784846c303e2e9b20',
    'agy.toml': '84ee58cf600576ab54d383bb241d322671e2ace4d5061cad439544dd1f551992',
}
moves = []
for name, digest in expected.items():
    source = directory / name
    if source.is_symlink():
        raise SystemExit(f'STOP: unexpected symlink: {source}')
    if not source.exists():
        continue
    destination = directory / (name + '.q22-disabled')
    if destination.exists() or destination.is_symlink():
        raise SystemExit(f'STOP: rollback destination exists: {destination}')
    if hashlib.sha256(source.read_bytes()).hexdigest() != digest:
        raise SystemExit(f'STOP: override has changed: {source}')
    moves.append((source, destination))
for source, destination in moves:
    source.rename(destination)
print('Candidate loader paths removed; reload required if previously activated.')
PY
```

If that succeeds and the candidates were activated, Kyle runs:

```sh
HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr server reload-agent-manifests

HERDR_SOCKET_PATH=/Users/kyle/.config/herdr/herdr.sock /Users/kyle/.local/bin/herdr server agent-manifests --json
```

Expected restored entries: opencode bundled `2026.06.10.1`, agy bundled `2026.06.24.1`, Claude bundled `2026.09.04.1`, assuming no unrelated manifest changes since capture. If different, inspect provenance. The report and staged copies do not participate in runtime loading.

## Preservation record

Reference hashes, verified unchanged at completion:

- `.git/index`: `4ffa968340099c15d173117e10b28cbf814ef0a566df72490fc66b2d9fb811ad`
- Bundled Claude: `2926a5739ee9f534d0e95135075271c82e10cb00511394480d525d2b9d265cdd`
- `~/.config/herdr/config.toml`: `7818215c438d8310110936871afde00e57578989c9ba890309b47597c5fcbe45`
- Earshot `src/toolbox/prompt-signature.ts`: `37d75ec022171155dce526dd915c7b077bfc7f74c925d27c8e704f6f4baea776`
- `~/.local/bin/herdr`: `95ab73bf07d57b95e98302a4ce61da087b18a1b59df5aa39313958e39bb2afcb`
- `~/.local/bin/herdr.bak-0.9.0-stock-20260910`: `32b53df09872628059c789a69f02a6b8e29e14ddf26711421f3463f70c1aef17`

The staged files remain `docs/next/website/src/content/docs/agent-automation.mdx` and `src/server/alt_screen_read.rs`; this report is untracked. No source files, branch, index entries, configuration, binaries, or Earshot files were modified by this work.


## 2026-09-12 — Follow-up: OpenCode hook authority and Earshot answerability

**Conclusion:** normal, delivered lifecycle reports are handled correctly; the missing idle manifest is not the reason a hooked OpenCode pane lacks `matched_rule`. However, the current path is **not safe against stale hook idle while OpenCode remains running**. This is a source-established failure path, not a reproduced incident.

**1. What supplies state, and can it report blocked?**

The bundled `HerdrAgentStatePlugin` sends sequenced `pane.report_agent` requests with `source="herdr:opencode"`, `agent="opencode"` and session identity. Its event mapping is:

| OpenCode callback/event | Reported state |
|---|---|
| `chat.message`; recognized active/busy/pending/retry/running/streaming/working `session.status`; tool before/after; permission/question replies; question rejection; compaction | working |
| `permission.asked`, `question.asked`, `session.error` | blocked |
| `session.idle` or idle `session.status` | idle |

Tracked child permission/question events are routed to their root session; child idle events are ignored. Unknown session-status values report identity only. Thus a dialog becomes blocked only when its corresponding event produces an accepted state report; drawing it is insufficient. The state plugin on disk at `~/.config/opencode/plugins/herdr-agent-state.js` byte-matches this checkout's version 11. [Plugin mapping](/Users/kyle/Repos/herdr/src/integration/assets/opencode/herdr-agent-state.js:131).

The exact ingestion chain is `handle_pane_report_agent` → `AppEvent::HookStateReported` → `set_hook_authority_with_session_ref` / `set_hook_authority_at` → `recompute_effective_state`. Session/owner and sequence checks precede acceptance; effective state takes `authority.state`. The separate TUI plugin reports selected-session identity, not readiness. API `done` is merely unseen idle, not another hook state. [API ingress](/Users/kyle/Repos/herdr/src/app/api/panes.rs:1531), [event dispatch](/Users/kyle/Repos/herdr/src/app/actions.rs:1683), [arbitration](/Users/kyle/Repos/herdr/src/terminal/state.rs:2149), [done mapping](/Users/kyle/Repos/herdr/src/app/api_helpers.rs:96).

**2. What if reporting stops?**

- **Reporter failure while OpenCode remains recognized:** the last accepted state persists without a time limit until another accepted report or an explicit lifecycle/identity change clears or replaces authority. “Live” checks agent/process ownership and absence of an observed exit, not heartbeat age. `reported_at` supports ordering; it is not a lease. Screen updates are skipped/ignored, including visible working and blockers. Last working/blocked therefore stays non-answerable; last idle can stay dangerously answerable. [Authority predicate](/Users/kyle/Repos/herdr/src/terminal/state.rs:1805), [screen suppression](/Users/kyle/Repos/herdr/src/terminal/state.rs:774), [detector early exit](/Users/kyle/Repos/herdr/src/pane.rs:876).
- **Dropped hook request:** every report uses a short-lived socket. Timeout (500 ms), error and close resolve the attempt; there is no state-report retry, heartbeat, acknowledgement validation, or automatic authority clear. Later events can recover reporting, but merely restoring the socket does not replay the lost state. If Herdr's entire API is unavailable, Earshot's probes fail and it refuses; the stale-idle case requires its later probes to succeed. [Transport](/Users/kyle/Repos/herdr/src/integration/assets/opencode/herdr-agent-state.js:55).
- **Actual foreground-agent exit detected:** `process_exited` bypasses suppression, clears matching authority/session identity and releases agent identity before recomputation. This is not permanent idle for a dead agent. Detection is observational, not instantaneous; Herdr also probes the live foreground job before sending. Killing only a backend/reporter while the recognized TUI survives belongs to the first case. [Exit clearing](/Users/kyle/Repos/herdr/src/terminal/state.rs:401), [effective identity](/Users/kyle/Repos/herdr/src/terminal/state.rs:1812), [send-time process check](/Users/kyle/Repos/herdr/src/app/agents.rs:426).

**3. Can Earshot be wrong today?**

**Yes, under that failure condition.** Concrete counterexample: Herdr accepts idle; OpenCode starts another turn and reaches a permission dialog; subsequent working/blocked reports are lost while the foreground TUI survives. Herdr retains idle/done. Its hooked explanation has null fallback and false visible flags, so Earshot skips its busy/empty-prompt signature and labels the pane answerable. `interactive_ready` is launch readiness, not hook freshness. The send path also passes: Herdr rejects its stored blocked state and a missing foreground agent, but does not independently inspect the screen or require stored idle. [Hooked explanation](/Users/kyle/Repos/herdr/src/app/api/agents.rs:264), [Earshot verdict](/Users/kyle/Repos/earshot/src/toolbox/session-list.ts:164), [Earshot send gate](/Users/kyle/Repos/earshot/src/toolbox/session-answer.ts:180), [Herdr send gate](/Users/kyle/Repos/herdr/src/app/api/agents.rs:138).

Conversely, losing only the blocked report after working was accepted leaves working, which Earshot correctly refuses. A correctly delivered blocked report is refused by both layers. No fix is justified merely because hooked `matched_rule` is null, and adding manifest rules cannot repair this bypass.

These sources settle the failure semantics, not whether events have actually been lost here or whether the installed OpenCode emits every event in the required order. Settling that needs a separately authorized, isolated trace correlating native events, accepted reports and screen transitions, including a deliberately dropped report. No such experiment or live operation was performed; only this report was appended.
