//! Build identity helpers.

pub const BASE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// FORK: our build counter, bumped once per INSTALLED build. See FORK.md.
///
/// Rendered as semver build metadata (`+fork.N`), which the spec defines as
/// ignored for precedence -- `0.9.0+fork.1` compares equal to `0.9.0`, so an
/// upstream release still reads as newer and the update check is unaffected.
///
/// This deliberately does NOT live in Cargo.toml. `update::Version::parse`
/// splits `CARGO_PKG_VERSION` on '.' and requires exactly three integer parts,
/// and `Version::current()` `.expect()`s the result -- so a `+fork.N` there
/// would panic the update checker at runtime. Keeping it here means
/// `BASE_VERSION` stays a clean `0.9.0` for every comparison, and only the
/// human- and API-facing string carries the label.
pub const FORK_BUILD: u32 = 2;

pub fn channel() -> &'static str {
    non_empty(option_env!("HERDR_BUILD_CHANNEL")).unwrap_or("stable")
}

pub fn build_id() -> Option<&'static str> {
    non_empty(option_env!("HERDR_BUILD_ID"))
}

pub fn version() -> String {
    let base = release_version();
    // FORK: identify our builds. Nothing parses this string -- the preview form
    // above (`0.9.0-preview.3`) would already fail Version::parse, so display
    // version and comparison version are separate by existing design.
    format!("{base}+fork.{FORK_BUILD}")
}

/// The upstream release this build is of, with no fork label.
///
/// FORK: release notes and product announcements are keyed by the release they
/// describe, not by which of our builds is running. Keying them on `version()`
/// would mean a stored "seen" marker written for `0.9.0` never matches
/// `0.9.0+fork.1`, so the notes would reappear on every startup forever. This
/// is exactly `version()` minus our label, so preview builds keep their own
/// `-preview.N` identity.
pub fn release_version() -> String {
    match channel() {
        "stable" => BASE_VERSION.to_string(),
        channel => match build_id() {
            Some(build_id) => format!("{BASE_VERSION}-{channel}.{build_id}"),
            None => format!("{BASE_VERSION}-{channel}"),
        },
    }
}

pub fn is_preview() -> bool {
    channel() == "preview"
}

fn non_empty(value: Option<&'static str>) -> Option<&'static str> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn stable_version_defaults_to_cargo_version() {
        assert!(!super::version().is_empty());
    }

    /// FORK: the label must never reach the version comparator. If this fails,
    /// the update checker panics at runtime via Version::current().expect().
    #[test]
    fn fork_label_does_not_break_version_parsing() {
        assert!(
            crate::update::Version::parse(super::BASE_VERSION).is_some(),
            "BASE_VERSION must stay parseable; the fork label belongs in version(), not Cargo.toml"
        );
        assert!(!super::BASE_VERSION.contains('+'));
        assert!(super::version().contains("+fork."));
    }
}
