//! Read-only extraction from named permission matches. Never reconstruct shell
//! syntax from terminal wrapping: target_text and labels retain displayed rows.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PermissionDialog {
    pub kind: String,
    pub question: String,
    /// Display-only terminal rows, with wrapping/indentation preserved.
    /// May be clipped by the terminal; never an executable command or path.
    pub target_text: String,
    pub options: Vec<PermissionOption>,
    pub raw_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PermissionOption {
    pub number: String,
    pub label: String,
    pub selected: bool,
    /// once, conversation, session, workspace, persistent, deny, or unknown.
    /// Unknown deliberately does not imply that an option grants permission.
    pub scope: String,
}

pub(super) fn parse(agent: &str, rule: &str, screen: &str) -> Option<PermissionDialog> {
    let (kind, question, marker) = match (agent, rule) {
        ("muse", "command_permission_prompt") => (
            "command",
            "Would you like to run the following command?",
            None,
        ),
        ("agy", "command_permission_prompt") => (
            "command",
            "Run this command?",
            Some("Requesting permission for:"),
        ),
        ("agy", "file_access_dialog") => ("path", "Allow access to this file?", Some("Read:")),
        ("agy", "network_permission_prompt") => (
            "network",
            "Allow access to this URL?",
            Some("Requesting permission for:"),
        ),
        ("agy", "tool_permission_prompt") => (
            "tool",
            "Allow calling this tool?",
            Some("Requesting permission for:"),
        ),
        _ => return None,
    };
    let lines: Vec<_> = screen.lines().collect();
    let (question_index, question_end) = text_rows(&lines, question)?;
    // Muse displays the command before its options. Locate the real trailing
    // controls backwards from the footer so numbered command/heredoc rows and
    // command text containing navigation words never enter the option scanner.
    let (option_start, option_end, muse_target_start) = if agent == "muse" {
        let (footer, _) = text_rows(&lines, "Change approval defaults in /permissions")?;
        let first = (question_end..footer)
            .rev()
            .find(|&index| option_label(lines[index]) == "1. Yes, proceed (y)")?;
        let target =
            (question_end..first).find(|&index| lines[index].trim_start().starts_with('$'))?;
        (first, footer, Some(target))
    } else {
        (question_end, lines.len(), None)
    };
    let mut options: Vec<PermissionOption> = Vec::new();
    let mut end = option_start;
    for (index, line) in lines.iter().enumerate().take(option_end).skip(option_start) {
        let trimmed = line.trim();
        if trimmed.starts_with("Change approval defaults")
            || (trimmed.starts_with("↑/↓") && trimmed.contains("Navigate"))
            || trimmed.starts_with("esc to cancel")
        {
            end = index;
            break;
        }
        let label = option_label(line);
        if let Some((number, label)) = label
            .split_once(". ")
            .filter(|(number, _)| !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit()))
        {
            if options.iter().any(|option| option.number == number) {
                return None;
            }
            options.push(PermissionOption {
                number: number.to_string(),
                label: label.to_string(),
                selected: trimmed.starts_with(['>', '›', '⟩', '❯']),
                scope: String::new(),
            });
        } else if !trimmed.is_empty() {
            // A wrapped label is retained verbatim, including indentation.
            let Some(option) = options.last_mut() else {
                continue;
            };
            option.label.push('\n');
            option.label.push_str(line);
        }
        end = index + 1;
    }
    if options.len() < 2 {
        return None;
    }
    for option in &mut options {
        let label = option
            .label
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        option.scope = if label.starts_with("no,") {
            "deny"
        } else if matches!(kind, "network" | "tool") {
            // These layouts have only reconstructed fixtures. Their approval
            // lifetimes cannot be inferred from illustrative option wording.
            "unknown"
        } else if label.contains("persist to settings.json") {
            "persistent"
        } else if label.contains("this conversation") {
            "conversation"
        } else if label.contains("this session") {
            "session"
        } else if label.contains("this workspace") {
            "workspace"
        } else if label.contains("always") || label.contains("don't ask again") {
            "persistent"
        } else if label == "yes, proceed (y)"
            || label == "yes, run command"
            || label == "yes, allow access"
        {
            "once"
        } else {
            "unknown"
        }
        .to_string();
    }
    let (start, target_text) = if let Some(marker) = marker {
        let start = lines[..question_index]
            .iter()
            .rposition(|line| line.trim().starts_with(marker))?;
        // Reason rows describe the request, not its target, for every kind.
        let stop = lines[start + 1..question_index]
            .iter()
            .position(|line| line.trim_start().starts_with("Reason:"))
            .map(|offset| start + 1 + offset)
            .unwrap_or(question_index);
        if marker == "Read:" {
            let mut target = lines[start]
                .trim()
                .strip_prefix(marker)?
                .trim_start()
                .to_string();
            for line in &lines[start + 1..stop] {
                if !line.trim().is_empty() {
                    target.push('\n');
                    target.push_str(line);
                }
            }
            (start, target)
        } else {
            (
                start,
                lines[start + 1..stop]
                    .join("\n")
                    .trim_matches('\n')
                    .to_string(),
            )
        }
    } else {
        (
            question_index,
            lines[muse_target_start?..option_start]
                .join("\n")
                .trim_matches('\n')
                .to_string(),
        )
    };
    if target_text.trim().is_empty() {
        return None;
    }
    Some(PermissionDialog {
        kind: kind.to_string(),
        question: question.to_string(),
        target_text,
        options,
        raw_text: lines[start..end].join("\n"),
    })
}

fn option_label(line: &str) -> &str {
    line.trim()
        .trim_start_matches(['>', '›', '⟩', '❯'])
        .trim_start()
}

// Match a literal UI sentence across terminal word wraps without rewriting
// target or option text. Return the original row boundaries.
fn text_rows(lines: &[&str], text: &str) -> Option<(usize, usize)> {
    for start in (0..lines.len()).rev() {
        if lines[start].trim().is_empty() {
            continue;
        }
        let mut joined = String::new();
        for (index, line) in lines.iter().enumerate().skip(start) {
            for word in line.split_whitespace() {
                if !joined.is_empty() {
                    joined.push(' ');
                }
                joined.push_str(word);
            }
            if joined == text {
                return Some((start, index + 1));
            }
            if !text.starts_with(&joined) {
                break;
            }
        }
    }
    None
}
