//! Per-encoding detection coverage using the same `encoding` crate as the detector.

use std::path::PathBuf;

use ecd::detect::{Detection, detect_bytes, detect_file};
use ecd::encodings::SUPPORTED;
use encoding::{EncoderTrap, EncodingRef};

struct EncodingCase {
    encoding: &'static str,
    text: Option<&'static str>,
    acceptable: &'static [&'static str],
}

fn text_case(
    encoding: &'static str,
    text: &'static str,
    acceptable: &'static [&'static str],
) -> EncodingCase {
    EncodingCase {
        encoding,
        text: Some(text),
        acceptable,
    }
}

fn encoding_ref(name: &str) -> EncodingRef {
    encoding::label::encoding_from_whatwg_label(name)
        .unwrap_or_else(|| panic!("unsupported test encoding label: {name}"))
}

fn case_bytes(case: &EncodingCase) -> Vec<u8> {
    let text = case.text.expect("case needs text");
    let enc = encoding_ref(case.encoding);
    let mut bytes = enc
        .encode(text, EncoderTrap::Strict)
        .expect("encode fixture text");
    if case.encoding == "utf-16le" {
        bytes.splice(0..0, [0xFF, 0xFE]);
    } else if case.encoding == "utf-16be" {
        bytes.splice(0..0, [0xFE, 0xFF]);
    }
    bytes
}

fn cases() -> Vec<EncodingCase> {
    vec![
        text_case(
            "utf-8",
            "我没有埋怨，磋砣的只是一些时间。héllo 😀",
            &["utf-8"],
        ),
        text_case(
            "ascii",
            "Dead-simple ASCII payload for detection testing. 0123456789",
            &["ascii", "utf-8"],
        ),
        text_case(
            "gbk",
            "春眠不觉晓，处处闻啼鸟。夜来风雨声，花落知多少。\n床前明月光，疑是地上霜。举头望明月，低头思故乡。\n编码检测工具需要足够多样的汉字样本才能准确识别GBK编码格式。",
            &["gbk", "gb18030"],
        ),
        text_case(
            "big5",
            "繁體中文測試資料：臺灣香港常用漢字樣本。",
            &["big5"],
        ),
        text_case(
            "shift_jis",
            "日本語の文字コード検出テスト。ひらがなカタカナ漢字。",
            &["shift_jis"],
        ),
        text_case(
            "euc-jp",
            "ユニコードではない日本語テキスト。長い文章で判定精度を上げる。",
            &["euc-jp"],
        ),
        text_case(
            "euc-kr",
            "대한민국 헌법. 제1조 ① 대한민국은 민주공화국이다. ② 대한민국의 주권은 국민에게 있고, 모든 권력은 국민으로부터 나온다.\
             대한민국 헌법. 제1조 ① 대한민국은 민주공화국이다. ② 대한민국의 주권은 국민에게 있고, 모든 권력은 국민으로부터 나온다.\
             대한민국 헌법. 제1조 ① 대한민국은 민주공화국이다. ② 대한민국의 주권은 국민에게 있고, 모든 권력은 국민으로부터 나온다.",
            &["euc-kr"],
        ),
        text_case(
            "iso-2022-jp",
            "ISO-2022-JP sample: 日本語のテキスト。",
            &["iso-2022-jp", "shift_jis", "euc-jp"],
        ),
        text_case("ibm866", "Русский текст в IBM866: привет мир.", &["ibm866"]),
        text_case(
            "iso-8859-2",
            "Polski tekst ISO-8859-2: Zażółć gęślą jaźń.",
            &["iso-8859-2", "iso-8859-1", "windows-1250"],
        ),
        text_case(
            "iso-8859-3",
            "Esperanto: Ĉi tiu teksto estas provo. Eĥoŝanĝi ĉiuĵaŭde.",
            &["iso-8859-3", "iso-8859-1"],
        ),
        text_case(
            "iso-8859-4",
            "Latviešu ISO-8859-4: Āžūģķļņš un garāks teksts.",
            &["iso-8859-4", "iso-8859-2", "windows-1257"],
        ),
        text_case(
            "iso-8859-5",
            "Русский кириллица ISO-8859-5 образец текста.",
            &["iso-8859-5", "windows-1251", "koi8-r"],
        ),
        text_case(
            "iso-8859-6",
            "نص عربي لاختبار ISO-8859-6. المزيد من الكلمات العربية.",
            &["iso-8859-6", "windows-1256"],
        ),
        text_case(
            "iso-8859-7",
            "Ελληνικά κείμενο ISO-8859-7. Περισσότερο ελληνικό κείμενο.",
            &["iso-8859-7", "windows-1253"],
        ),
        text_case(
            "iso-8859-8",
            "עברית לבדיקת ISO-8859-8. טקסט עברי ארוך יותר.",
            &["iso-8859-8", "iso-8859-5", "windows-1255", "iso-8859-8-i"],
        ),
        text_case(
            "iso-8859-8-i",
            "עברית לוגית ISO-8859-8-I. טקסט נוסף בעברית.",
            &["iso-8859-8-i", "iso-8859-8", "iso-8859-5", "windows-1255"],
        ),
        text_case(
            "iso-8859-10",
            "Nordic ISO-8859-10: ÆØÅ æøå. Mer nordisk tekst.",
            &["iso-8859-10", "ibm866", "iso-8859-1", "windows-1252"],
        ),
        text_case(
            "iso-8859-13",
            "Baltic ISO-8859-13: ĄČĘĖĮŠŲŪŽ ąčęėįšųūž.",
            &["iso-8859-13", "iso-8859-5", "windows-1257"],
        ),
        text_case(
            "iso-8859-14",
            "Gaeilge ISO-8859-14: áéíóú ÁÉÍÓÚ.",
            &["iso-8859-14", "iso-8859-5", "iso-8859-1"],
        ),
        text_case(
            "iso-8859-15",
            "Euro ISO-8859-15: € Šš Žž Œœ Ÿ.",
            &["iso-8859-15", "iso-8859-5", "windows-1252"],
        ),
        text_case(
            "iso-8859-16",
            "Romanian ISO-8859-16: ăâîșț ĂÂÎȘȚ.",
            &["iso-8859-16", "iso-8859-13", "windows-1250"],
        ),
        text_case(
            "koi8-r",
            "Русский KOI8-R: привет, мир! Длинный русский текст.",
            &["koi8-r", "iso-8859-5", "windows-1251"],
        ),
        text_case(
            "koi8-u",
            "Українська KOI8-U: привіт, світ! Більше українського тексту.",
            &["koi8-u", "koi8-r"],
        ),
        text_case(
            "windows-1250",
            "Polski Windows-1250: zażółć gęślą jaźń.",
            &["windows-1250", "iso-8859-2"],
        ),
        text_case(
            "windows-1251",
            "Русский Windows-1251: привет мир. Кириллица.",
            &["windows-1251", "iso-8859-7", "iso-8859-5", "koi8-r"],
        ),
        text_case(
            "windows-1252",
            "Windows-1252: café naïve résumé. Curly quotes.",
            &["windows-1252", "iso-8859-1", "iso-8859-15"],
        ),
        text_case(
            "windows-1253",
            "Ελληνικά Windows-1253: καλημέρα κόσμε.",
            &["windows-1253", "iso-8859-7", "iso-8859-5"],
        ),
        text_case(
            "windows-1254",
            "Türkçe Windows-1254: ğüşiöç İıŞş.",
            &["windows-1254", "iso-8859-1"],
        ),
        text_case(
            "windows-1255",
            "עברית Windows-1255 לבדיקה. טקסט עברי.",
            &["windows-1255", "iso-8859-8", "iso-8859-8-i", "iso-8859-5"],
        ),
        text_case(
            "windows-1256",
            "العربية Windows-1256 للاختبار. المزيد من النص.",
            &["windows-1256", "iso-8859-6"],
        ),
        text_case(
            "windows-1257",
            "Latvian Windows-1257: āčēģīķļņš ŅŪŽ.",
            &["windows-1257", "ibm866", "iso-8859-13", "iso-8859-4"],
        ),
        text_case(
            "windows-1258",
            "Vietnamese Windows-1258: àáâđêôơư.",
            &["windows-1258", "euc-kr"],
        ),
        text_case(
            "windows-874",
            "ภาษาไทย Windows-874 ตัวอย่างข้อความยาว.",
            &["windows-874"],
        ),
        text_case(
            "macintosh",
            "Macintosh Roman: résumé naïve façade. æÆ © ®.",
            &["macintosh", "windows-1252"],
        ),
        text_case(
            "x-mac-cyrillic",
            "Mac Cyrillic: русский текст. Ещё кириллица.",
            &["x-mac-cyrillic", "iso-8859-7", "windows-1251", "koi8-r"],
        ),
        text_case(
            "utf-16le",
            "UTF-16 LE sample with CJK: 中文测试.",
            &["utf-16le"],
        ),
        text_case("utf-16be", "UTF-16 BE sample text.", &["utf-16be"]),
    ]
}

fn assert_detected(bytes: &[u8], case: &EncodingCase) {
    match detect_bytes(bytes) {
        Detection::Encoded(got) => {
            let got = got.as_str();
            let ok = case.acceptable.contains(&got);
            assert!(
                ok,
                "encoding {}: got {got}, expected one of {:?}",
                case.encoding, case.acceptable
            );
        }
        other => panic!("encoding {}: unexpected {other:?}", case.encoding),
    }
}

#[test]
fn supported_list_matches_cases() {
    let case_names: Vec<_> = cases().iter().map(|c| c.encoding).collect();
    let mut expected = SUPPORTED.to_vec();
    expected.sort_unstable();
    let mut from_cases = case_names;
    from_cases.sort_unstable();
    assert_eq!(from_cases, expected, "SUPPORTED must match test cases");
}

#[test]
fn detects_all_supported_encodings() {
    for case in cases() {
        let bytes = case_bytes(&case);
        assert_detected(&bytes, &case);
    }
}

/// Print detector output for each case to tune `acceptable` lists:
/// `cargo test calibrate_encodings -- --ignored --nocapture`
#[test]
#[ignore]
fn calibrate_encodings() {
    for case in cases() {
        let bytes = case_bytes(&case);
        match detect_bytes(&bytes) {
            Detection::Encoded(got) => {
                println!("{}: {got}", case.encoding);
            }
            other => println!("{}: {:?}", case.encoding, other),
        }
    }
}
/// Regenerate `tests/fixtures/encodings/` and `encodings.json`:
/// `cargo test write_encoding_fixtures -- --ignored`
#[test]
#[ignore]
fn write_encoding_fixtures() {
    use std::fs;
    use std::io::Write;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/encodings");
    fs::create_dir_all(&root).expect("create encodings dir");

    let all = cases();
    let mut lines = vec!["[".to_string()];
    for (i, case) in all.iter().enumerate() {
        let bytes = case_bytes(case);
        let file = format!("encodings/{}.bin", case.encoding);
        fs::write(root.join(format!("{}.bin", case.encoding)), &bytes).expect("write fixture");

        let acceptable: Vec<String> = case.acceptable.iter().map(|s| format!("\"{s}\"")).collect();
        let comma = if i + 1 == all.len() { "" } else { "," };
        lines.push(format!(
            "  {{\"file\": \"{file}\", \"expected\": \"{}\", \"acceptable\": [{acceptable}]}}{comma}",
            case.encoding,
            acceptable = acceptable.join(", ")
        ));
    }
    lines.push("]".to_string());

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/encodings.json");
    let mut f = fs::File::create(manifest).expect("create manifest");
    writeln!(f, "{}", lines.join("\n")).expect("write manifest");
}

#[test]
fn cli_fixtures_match_manifest() {
    let manifest_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/encodings.json");
    let manifest = read_manifest(&manifest_path);

    for (file, expected, acceptable) in manifest {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(&file);
        match detect_file(&path) {
            Detection::Encoded(got) => {
                assert!(
                    acceptable.iter().any(|a| a == &got),
                    "{}: got {got}, expected one of {acceptable:?} (primary: {expected})",
                    file
                );
            }
            other => panic!("{}: unexpected {other:?}", file),
        }
    }
}

fn read_manifest(path: &std::path::Path) -> Vec<(String, String, Vec<String>)> {
    let raw = std::fs::read_to_string(path).expect("read manifest");
    let value: serde_json::Value = serde_json::from_str(&raw).expect("parse manifest");
    value
        .as_array()
        .expect("manifest array")
        .iter()
        .map(|entry| {
            let file = entry["file"].as_str().expect("file").to_string();
            let expected = entry["expected"].as_str().expect("expected").to_string();
            let acceptable = entry
                .get("acceptable")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_else(|| vec![expected.clone()]);
            (file, expected, acceptable)
        })
        .collect()
}
