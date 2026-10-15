Title: fix: tighten codex idle detection for shortcuts and warnings footer (QL-390)

### Summary

Rework Codex CLI idle detection rules to correctly match idle screens with a `? for shortcuts` line and optional trailing status (`⚠ N warning(s) · f2 to view`) below the model/cwd footer, without over-matching interactive prompts, navigation footers, paste placeholders, or extra output.

### Root Cause Analysis

- Screen lines above the composer (such as `1 background terminal running · /ps … · /stop …` and `⚠ 5h limit …`) are already accepted by `(?m)^›[ \t]+Ask Codex to do anything`, which anchors at the composer line.
- The real trigger causing idle Codex sessions (e.g., `herdr-codex-mgoy`) to be classified as `unclassified-screen` is the line **below** the model/cwd footer: `? for shortcuts` followed on the same line by right-aligned `⚠ N warning(s) · f2 to view`.
- The previously proposed trailing pattern was overly permissive (`(?:\r?\n[ \t]{2,}[^\r\n]+)*`), which could over-match indented choice options (`› 1. Yes / 2. No`), menu footers (`↑↓ navigate · enter select · esc close`), paste placeholders, or extra output.

### Changes

1. **Tighten `live_prompt_box` Regex:**
   - In `src/detect/manifests/codex.toml` and `distribution/agent-detection/codex.toml`, reworked the regex to:
     - Anchor on `› Ask Codex to do anything`.
     - Match the model/cwd line containing ` · `.
     - Allow at most one wrapped continuation line (for long paths or descriptions), constrained so it cannot start with `?`, `›`, `[`, `↑`, or `↓`.
     - Optionally match one `? for shortcuts` line carrying optional trailing status (e.g. `⚠ 2 warnings · f2 to view`).
     - Accept only blank lines to `\z`.
2. **Strengthen Dialog Vetoes:**
   - Added `{ contains = ["enter select"] }` and `{ contains = ["esc close"] }` to `live_prompt_blocked` and to `live_prompt_box`'s `not` veto list.
   - Added `{ regex = ['(?i-u:\[pasted\s+text\b)'] }` to `live_prompt_box`'s `not` veto list.
3. **Tests in `src/detect/manifest/tests.rs`:**
   - Added positive tests from the verbatim capture at ~120 columns and ~60 columns (both wrapped cwd and wrapped at dot separator).
   - Added negative tests covering each listed over-match:
     - Indented `› 1. Yes / 2. No` options.
     - Option selections with `↑↓ navigate · enter select · esc close` footer.
     - Standalone `↑↓ navigate · enter select · esc close` and arrowless variants.
     - Paste placeholders below and inside the composer.
     - Extra output lines.
   - Retained active turn working vetoes and approval dialog blocker tests.

### Validation

- `cargo test ql390` passed cleanly.
- `cargo nextest run --locked` passed.
- `cargo clippy --all-targets --locked -- -D warnings` passed with 0 warnings.
- `cargo fmt --check` passed.
- `python3 scripts/agent_detection_manifest_check.py --require-published` passed.
- 150 repository maintenance and UI architecture tests passed.
