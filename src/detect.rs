use std::fs::File;
use std::io::Read;
use std::path::Path;

use charset_normalizer_rs::from_bytes;
use encoding_rs::Encoding;

pub const SAMPLE_SIZE: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Detection {
    Encoded(String),
    Skip(SkipReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    Empty,
    Binary,
    Unknown,
    ReadError(String),
}

pub fn detect_bytes(data: &[u8]) -> Detection {
    if data.is_empty() {
        return Detection::Skip(SkipReason::Empty);
    }

    if let Some((encoding, _bom_len)) = Encoding::for_bom(data) {
        return Detection::Encoded(normalize_encoding_name(encoding.name()));
    }

    if looks_binary(data) {
        return Detection::Skip(SkipReason::Binary);
    }

    match from_bytes(data, None) {
        Ok(results) => {
            if let Some(best) = results.get_best() {
                Detection::Encoded(normalize_encoding_name(best.encoding()))
            } else {
                Detection::Skip(SkipReason::Unknown)
            }
        }
        Err(_) => Detection::Skip(SkipReason::Unknown),
    }
}

pub fn detect_file(path: &Path) -> Detection {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Detection::Skip(SkipReason::ReadError(err.to_string())),
    };

    let mut buf = vec![0u8; SAMPLE_SIZE];
    let n = match file.read(&mut buf) {
        Ok(n) => n,
        Err(err) => return Detection::Skip(SkipReason::ReadError(err.to_string())),
    };
    buf.truncate(n);

    detect_bytes(&buf)
}

pub fn normalize_encoding_name(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        "utf_8" | "utf-8-sig" => "utf-8".to_string(),
        "gb18030" => "gbk".to_string(),
        other => other.to_string(),
    }
}

/// Returns true when content looks like binary (not UTF-16 text).
pub fn is_binary_content(data: &[u8]) -> bool {
    looks_binary(data)
}

fn looks_binary(data: &[u8]) -> bool {
    if data.contains(&0) {
        // UTF-16 text often contains NUL bytes in alternating positions.
        let utf16_le = data.windows(2).filter(|w| w[1] == 0 && w[0] != 0).count();
        let utf16_be = data.windows(2).filter(|w| w[0] == 0 && w[1] != 0).count();
        let pairs = data.len() / 2;
        if pairs > 0 && (utf16_le > pairs / 4 || utf16_be > pairs / 4) {
            return false;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_bom() {
        let data = b"\xEF\xBB\xBFhello";
        assert_eq!(detect_bytes(data), Detection::Encoded("utf-8".to_string()));
    }

    #[test]
    fn plain_utf8() {
        let data = b"hello world";
        match detect_bytes(data) {
            Detection::Encoded(enc) => assert!(enc == "utf-8" || enc == "ascii"),
            other => panic!("expected encoded, got {other:?}"),
        }
    }

    #[test]
    fn empty_is_skip() {
        assert_eq!(detect_bytes(b""), Detection::Skip(SkipReason::Empty));
    }

    #[test]
    fn binary_is_skip() {
        let data = b"hello\x00world\xff\xfe";
        assert_eq!(detect_bytes(data), Detection::Skip(SkipReason::Binary));
    }

    #[test]
    fn normalize_names() {
        assert_eq!(normalize_encoding_name("UTF-8"), "utf-8");
        assert_eq!(normalize_encoding_name("GBK"), "gbk");
        assert_eq!(normalize_encoding_name("gb18030"), "gbk");
    }
}
