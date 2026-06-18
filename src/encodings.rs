//! Supported and planned character encodings for detection.

use std::fmt;

use clap::builder::PossibleValuesParser;
use encoding_rs::Encoding;

/// Encodings currently detectable via charset-normalizer-rs (WHATWG / IANA).
pub const SUPPORTED: &[&str] = &[
    // Unicode
    "utf-8",
    "utf-16le",
    "utf-16be",
    // East Asian — multibyte
    "gbk",
    "big5",
    "shift_jis",
    "euc-jp",
    "euc-kr",
    "iso-2022-jp",
    // Cyrillic
    "koi8-r",
    "koi8-u",
    "windows-1251",
    "x-mac-cyrillic",
    // European — ISO 8859
    "iso-8859-2",
    "iso-8859-3",
    "iso-8859-4",
    "iso-8859-5",
    "iso-8859-6",
    "iso-8859-7",
    "iso-8859-8",
    "iso-8859-8-i",
    "iso-8859-10",
    "iso-8859-13",
    "iso-8859-14",
    "iso-8859-15",
    "iso-8859-16",
    // European — Windows
    "windows-1250",
    "windows-1252",
    "windows-1253",
    "windows-1254",
    "windows-1255",
    "windows-1256",
    "windows-1257",
    "windows-1258",
    // Other
    "ibm866",
    "windows-874",
    "macintosh",
    "ascii",
];

/// Encodings not yet supported; may be added in future releases.
pub const PLANNED: &[&str] = &[
    // Legacy encodings outside the WHATWG set used by charset-normalizer-rs
    "tis-620",
    "cp437",
    "cp850",
    "cp932",
    "gb2312",
    "hz-gb-2312",
    "utf-32le",
    "utf-32be",
    "utf-7",
];

/// Group labels for documentation.
pub const GROUPS: &[(&str, &[&str])] = &[
    ("Unicode", &["utf-8", "utf-16le", "utf-16be"]),
    (
        "East Asian",
        &[
            "gbk",
            "big5",
            "shift_jis",
            "euc-jp",
            "euc-kr",
            "iso-2022-jp",
        ],
    ),
    (
        "Cyrillic",
        &["koi8-r", "koi8-u", "windows-1251", "x-mac-cyrillic"],
    ),
    (
        "ISO 8859",
        &[
            "iso-8859-2",
            "iso-8859-3",
            "iso-8859-4",
            "iso-8859-5",
            "iso-8859-6",
            "iso-8859-7",
            "iso-8859-8",
            "iso-8859-8-i",
            "iso-8859-10",
            "iso-8859-13",
            "iso-8859-14",
            "iso-8859-15",
            "iso-8859-16",
        ],
    ),
    (
        "Windows",
        &[
            "windows-1250",
            "windows-1252",
            "windows-1253",
            "windows-1254",
            "windows-1255",
            "windows-1256",
            "windows-1257",
            "windows-1258",
            "windows-874",
        ],
    ),
    ("Other", &["ibm866", "macintosh", "ascii"]),
];

pub fn is_supported(name: &str) -> bool {
    let normalized = name.to_ascii_lowercase();
    SUPPORTED.iter().any(|enc| *enc == normalized)
}

/// Clap value parser and shell completion source for supported encoding names.
pub fn supported_encoding_parser() -> PossibleValuesParser {
    PossibleValuesParser::new(SUPPORTED)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    Unsupported(String),
    Unresolvable(String),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(name) => write!(f, "unsupported encoding: {name}"),
            Self::Unresolvable(name) => write!(f, "unresolvable encoding: {name}"),
        }
    }
}

impl std::error::Error for EncodingError {}

/// Resolve a supported canonical encoding name to an `encoding_rs` codec.
pub fn lookup_encoding(name: &str) -> Result<&'static Encoding, EncodingError> {
    let normalized = name.to_ascii_lowercase();
    if !is_supported(&normalized) {
        return Err(EncodingError::Unsupported(normalized));
    }
    Encoding::for_label(normalized.as_bytes()).ok_or(EncodingError::Unresolvable(normalized))
}

/// All valid encoding names (supported + planned), sorted and unique.
pub fn all_names() -> Vec<&'static str> {
    let mut names: Vec<_> = SUPPORTED.iter().chain(PLANNED.iter()).copied().collect();
    names.sort_unstable();
    names.dedup();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_cover_all_supported() {
        let mut from_groups: Vec<&str> = GROUPS
            .iter()
            .flat_map(|(_, encs)| encs.iter().copied())
            .collect();
        from_groups.sort_unstable();
        from_groups.dedup();

        let mut supported = SUPPORTED.to_vec();
        supported.sort_unstable();

        assert_eq!(from_groups, supported);
    }

    #[test]
    fn planned_does_not_overlap_supported() {
        for enc in PLANNED {
            assert!(
                !is_supported(enc),
                "planned encoding {enc} is already supported"
            );
        }
    }

    #[test]
    fn all_supported_labels_resolve() {
        for enc in SUPPORTED {
            lookup_encoding(enc).unwrap_or_else(|err| panic!("{enc}: {err}"));
        }
    }

    #[test]
    fn lookup_rejects_planned() {
        assert!(matches!(
            lookup_encoding("tis-620"),
            Err(EncodingError::Unsupported(_))
        ));
    }

    #[test]
    fn all_names_includes_supported_and_planned() {
        let names = all_names();
        assert_eq!(names.len(), SUPPORTED.len() + PLANNED.len());
        for enc in SUPPORTED {
            assert!(names.contains(enc), "missing supported encoding {enc}");
        }
        for enc in PLANNED {
            assert!(names.contains(enc), "missing planned encoding {enc}");
        }
    }
}
