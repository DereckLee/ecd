use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use encoding_rs::Encoding;

use crate::detect::is_binary_content;
use crate::encodings::{self, EncodingError};

#[derive(Debug, Clone, Copy, Default)]
pub struct ConvertOptions {
    pub strict: bool,
    pub write_bom: bool,
    pub force: bool,
}

#[derive(Debug)]
pub enum ConvertError {
    Encoding(EncodingError),
    BinaryInput,
    DecodeError { from: String },
    EncodeError { to: String },
    InputNotFound(PathBuf),
    OutputExists(PathBuf),
    SamePathWithoutForce,
    Io(std::io::Error),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encoding(err) => write!(f, "{err}"),
            Self::BinaryInput => write!(f, "binary content"),
            Self::DecodeError { from } => write!(f, "failed to decode as {from}"),
            Self::EncodeError { to } => write!(f, "failed to encode as {to}"),
            Self::InputNotFound(path) => write!(f, "path not found: {}", path.display()),
            Self::OutputExists(path) => {
                write!(f, "output file exists: {} (use --force)", path.display())
            }
            Self::SamePathWithoutForce => {
                write!(
                    f,
                    "output path equals input path (use --force to overwrite)"
                )
            }
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ConvertError {}

impl From<EncodingError> for ConvertError {
    fn from(err: EncodingError) -> Self {
        Self::Encoding(err)
    }
}

impl From<std::io::Error> for ConvertError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn convert_bytes(
    input: &[u8],
    from: &str,
    to: &str,
    opts: &ConvertOptions,
) -> Result<Vec<u8>, ConvertError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if is_binary_content(input) {
        return Err(ConvertError::BinaryInput);
    }

    let from_enc = encodings::lookup_encoding(from)?;

    let (decoded, _, decode_had_errors) = from_enc.decode(input);
    if opts.strict && decode_had_errors {
        return Err(ConvertError::DecodeError {
            from: from.to_ascii_lowercase(),
        });
    }

    encode_string(&decoded, to, opts)
}

fn encode_string(decoded: &str, to: &str, opts: &ConvertOptions) -> Result<Vec<u8>, ConvertError> {
    let to_lower = to.to_ascii_lowercase();
    match to_lower.as_str() {
        "utf-16le" => Ok(encode_utf16le(decoded, opts.write_bom)),
        "utf-16be" => Ok(encode_utf16be(decoded, opts.write_bom)),
        _ => {
            let to_enc = encodings::lookup_encoding(to)?;
            let (encoded, _, encode_had_errors) = to_enc.encode(decoded);
            if opts.strict && encode_had_errors {
                return Err(ConvertError::EncodeError { to: to_lower });
            }
            let mut result = encoded.into_owned();
            if opts.write_bom && to_lower == "utf-8" {
                result = prepend_bom(encoding_rs::UTF_8, result);
            }
            Ok(result)
        }
    }
}

fn encode_utf16le(text: &str, write_bom: bool) -> Vec<u8> {
    let code_units: Vec<u16> = text.encode_utf16().collect();
    let mut out = Vec::with_capacity(if write_bom { 2 } else { 0 } + code_units.len() * 2);
    if write_bom {
        out.extend_from_slice(&[0xFF, 0xFE]);
    }
    for unit in code_units {
        out.extend_from_slice(&unit.to_le_bytes());
    }
    out
}

fn encode_utf16be(text: &str, write_bom: bool) -> Vec<u8> {
    let code_units: Vec<u16> = text.encode_utf16().collect();
    let mut out = Vec::with_capacity(if write_bom { 2 } else { 0 } + code_units.len() * 2);
    if write_bom {
        out.extend_from_slice(&[0xFE, 0xFF]);
    }
    for unit in code_units {
        out.extend_from_slice(&unit.to_be_bytes());
    }
    out
}

pub fn convert_file(
    input_path: &Path,
    output: Option<&Path>,
    from: &str,
    to: &str,
    opts: &ConvertOptions,
) -> Result<(), ConvertError> {
    if !input_path.is_file() {
        return Err(ConvertError::InputNotFound(input_path.to_path_buf()));
    }

    if let Some(out_path) = output {
        if out_path == input_path && !opts.force {
            return Err(ConvertError::SamePathWithoutForce);
        }
        if out_path.exists() && !opts.force {
            return Err(ConvertError::OutputExists(out_path.to_path_buf()));
        }
    }

    let input = fs::read(input_path)?;
    let converted = convert_bytes(&input, from, to, opts)?;

    match output {
        Some(out_path) => fs::write(out_path, converted)?,
        None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(&converted)?;
        }
    }

    Ok(())
}

fn prepend_bom(encoding: &'static Encoding, output: Vec<u8>) -> Vec<u8> {
    let bom: &[u8] = if encoding == encoding_rs::UTF_8 {
        &[0xEF, 0xBB, 0xBF]
    } else {
        return output;
    };

    let mut with_bom = Vec::with_capacity(bom.len() + output.len());
    with_bom.extend_from_slice(bom);
    with_bom.extend_from_slice(&output);
    with_bom
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::encodings::{EncodingError, SUPPORTED};

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    fn fixture_path(relative: &str) -> PathBuf {
        fixtures_dir().join(relative)
    }

    fn load_fixture(relative: &str) -> Vec<u8> {
        fs::read(fixture_path(relative)).expect("read fixture")
    }

    fn default_opts() -> ConvertOptions {
        ConvertOptions::default()
    }

    fn strict_opts() -> ConvertOptions {
        ConvertOptions {
            strict: true,
            ..Default::default()
        }
    }

    fn bom_opts() -> ConvertOptions {
        ConvertOptions {
            write_bom: true,
            ..Default::default()
        }
    }

    #[test]
    fn all_supported_labels_resolve() {
        for enc in SUPPORTED {
            encodings::lookup_encoding(enc).unwrap_or_else(|err| panic!("{enc}: {err}"));
        }
    }

    #[test]
    fn unsupported_encoding_rejected() {
        assert!(matches!(
            encodings::lookup_encoding("tis-620"),
            Err(EncodingError::Unsupported(_))
        ));
    }

    #[test]
    fn each_fixture_to_utf8() {
        let manifest = load_manifest();
        for (file, expected) in manifest {
            let bytes = load_fixture(&file);
            convert_bytes(&bytes, &expected, "utf-8", &default_opts())
                .unwrap_or_else(|err| panic!("{file} ({expected} -> utf-8): {err}"));
        }
    }

    #[test]
    fn utf8_to_utf16le_produces_wide_bytes() {
        let sample = "UTF-16 LE sample with CJK: 中文测试.";
        let out = convert_bytes(sample.as_bytes(), "utf-8", "utf-16le", &default_opts()).unwrap();
        assert!(
            out.len() > sample.len(),
            "expected UTF-16 output, got {} bytes",
            out.len()
        );
        assert!(
            out.windows(2).any(|w| w[1] == 0),
            "expected UTF-16LE NUL pairs"
        );
    }

    #[test]
    fn utf8_to_each_supported() {
        let sample = "Hello, encoding conversion test. Zażółć gęślą jaźń.";
        for enc in SUPPORTED {
            convert_bytes(sample.as_bytes(), "utf-8", enc, &default_opts())
                .unwrap_or_else(|err| panic!("utf-8 -> {enc}: {err}"));
        }
    }

    #[test]
    fn pairwise_via_fixtures() {
        let manifest = load_manifest();
        for (file, from) in &manifest {
            let bytes = load_fixture(file);
            for to in SUPPORTED {
                convert_bytes(&bytes, from, to, &default_opts()).unwrap_or_else(|err| {
                    panic!("{file} ({from} -> {to}): {err}");
                });
            }
        }
    }

    #[test]
    fn round_trip_utf8_hub() {
        let manifest = load_manifest();
        for (file, enc) in manifest {
            let original = load_fixture(&file);
            let opts = default_opts();
            let via_utf8 = convert_bytes(&original, &enc, "utf-8", &opts)
                .unwrap_or_else(|err| panic!("{file} ({enc} -> utf-8): {err}"));
            let round = convert_bytes(&via_utf8, "utf-8", &enc, &opts)
                .unwrap_or_else(|err| panic!("{file} (utf-8 -> {enc}): {err}"));

            let utf8_from_orig = convert_bytes(&original, &enc, "utf-8", &opts)
                .unwrap_or_else(|err| panic!("{file} ({enc} -> utf-8): {err}"));
            let utf8_from_round = convert_bytes(&round, &enc, "utf-8", &opts)
                .unwrap_or_else(|err| panic!("{file} ({enc} -> utf-8 after round): {err}"));
            assert_eq!(
                utf8_from_orig, utf8_from_round,
                "round-trip mismatch for {enc} ({file})"
            );
        }
    }

    #[test]
    fn strict_rejects_lossy() {
        let input = "hello 中文".as_bytes();
        let err = convert_bytes(input, "utf-8", "ascii", &strict_opts()).unwrap_err();
        assert!(matches!(err, ConvertError::EncodeError { .. }));
    }

    #[test]
    fn bom_output() {
        let input = b"hello";
        let out = convert_bytes(input, "ascii", "utf-8", &bom_opts()).unwrap();
        assert_eq!(&out[..3], &[0xEF, 0xBB, 0xBF]);
    }

    #[test]
    fn binary_input_rejected() {
        let data = b"hello\x00world\xff\xfe";
        let err = convert_bytes(data, "utf-8", "gbk", &default_opts()).unwrap_err();
        assert!(matches!(err, ConvertError::BinaryInput));
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(
            convert_bytes(b"", "utf-8", "gbk", &default_opts())
                .unwrap()
                .is_empty()
        );
    }

    fn load_manifest() -> Vec<(String, String)> {
        SUPPORTED
            .iter()
            .map(|enc| (format!("encodings/{enc}.bin"), (*enc).to_string()))
            .collect()
    }
}
