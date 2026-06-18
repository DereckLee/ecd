//! Supported and planned character encodings for detection.

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
}
