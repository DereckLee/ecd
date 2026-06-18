# ecd

[![crates.io](https://img.shields.io/crates/v/ecd.svg)](https://crates.io/crates/ecd)
[![license](https://img.shields.io/crates/l/ecd.svg)](https://github.com/DereckLee/ecd/blob/main/LICENSE)

**ecd** is a fast, cross-platform command-line tool for detecting the character encoding of text files. It is designed to be quicker on large trees, easier to script, and free of external tools like `file(1)` or `find(1)`.

```bash
ecd check -f man.txt    # gbk
ecd check -d ./src      # [UTF-8] src/main.rs
```

## Why ecd?

|                   | encoding-checker           | ecd                         |
| ----------------- | -------------------------- | --------------------------- |
| Runtime           | Node.js                    | Single native binary        |
| Large directories | Serial, slow               | Parallel (`rayon`)          |
| Cross-platform    | Depends on `file` / `find` | Pure Rust                   |
| Script output     | Always `[ENC] path`        | Single file → encoding only |

## Features

- **38 encodings** from the [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/) — UTF-8, GBK, Big5, Shift_JIS, EUC-KR, Windows code pages, ISO-8859, KOI8, and more
- **Script-friendly output** — one file prints `gbk`, not `[GBK] path`
- **Recursive directory scan** with glob filters and `.gitignore` support
- **BOM fast-path** for UTF-8 / UTF-16
- **Parallel detection** with configurable worker threads
- **Encoding conversion** between any two supported encodings (`ecd convert`)

## Install

```bash
cargo install ecd
```

From source:

```bash
git clone https://github.com/DereckLee/ecd.git
cd ecd

cargo install --path .
```

## Usage

```bash
# Single file — prints encoding only (ideal for pipes)
ecd check -f man.txt
# gbk

# Scan a directory recursively
ecd check -d ./src
# [UTF-8] src/main.rs
# [GBK] docs/readme.txt
# [SKIP] assets/logo.dat

# Only Rust files
ecd check -d . -p "*.rs"

# Skip ASCII files
ecd check -d . -i ascii

# Stats on stderr
ecd check -d . -v

# List valid encoding names (one per line)
ecd encodings
# ascii
# big5
# ...
# utf-8

# Convert between supported encodings (stdout when -o is omitted)
ecd convert -f man.txt --from utf-8 --to gbk
ecd convert -f man.txt --from gbk --to utf-8 -o man.utf8.txt
```

### Options

| Flag                     | Description                                    |
| ------------------------ | ---------------------------------------------- |
| `-f`, `--file <PATH>`    | File to check (repeatable)                     |
| `-d`, `--dir <PATH>`     | Directory to scan recursively (repeatable)     |
| `-p`, `--pattern <GLOB>` | File glob when scanning dirs (default: `**/*`) |
| `-i`, `--ignore <ENC>`   | Skip files with this encoding                  |
| `-e`, `--exclude <NAME>` | Extra directory names to exclude               |
| `--no-default-excludes`  | Do not skip `.git`, `node_modules`, `target`   |
| `-j`, `--jobs <N>`       | Parallel worker threads                        |
| `-v`, `--verbose`        | Print stats to stderr                          |
| `-q`, `--quiet`          | Suppress normal output                         |
| `-h`, `--help`           | Show help                                      |
| `-V`, `--version`        | Show version                                   |

## Supported Encodings

Detection is powered by [charset-normalizer-rs](https://crates.io/crates/charset-normalizer-rs) over the WHATWG / IANA encodings from the Rust `encoding` crate.

| Group      | Encodings                                                     |
| ---------- | ------------------------------------------------------------- |
| Unicode    | `utf-8`, `utf-16le`, `utf-16be`                               |
| East Asian | `gbk`, `big5`, `shift_jis`, `euc-jp`, `euc-kr`, `iso-2022-jp` |
| Cyrillic   | `koi8-r`, `koi8-u`, `windows-1251`, `x-mac-cyrillic`          |
| ISO 8859   | `iso-8859-2` … `iso-8859-16` (incl. `iso-8859-8-i`)           |
| Windows    | `windows-1250` … `windows-1258`, `windows-874`                |
| Other      | `ibm866`, `macintosh`, `ascii`                                |

**Normalization notes**

- `gb18030` is reported as `gbk`
- UTF-8 with BOM is reported as `utf-8`
- Similar single-byte encodings (e.g. ISO-8859 vs Windows code pages) can be ambiguous on very short files — use longer samples when accuracy matters

**Planned** (not yet supported): `tis-620`, `cp437`, `cp850`, `cp932`, `gb2312`, `hz-gb-2312`, `utf-32le`, `utf-32be`, `utf-7`

The canonical list lives in [`src/encodings.rs`](src/encodings.rs). Run `ecd encodings` to print all valid names (supported + planned). **`ecd convert` accepts only the 38 supported encodings** (not planned).

## Man Page

```bash
make man
man ./man/ecd.1
```

Install system-wide:

```bash
sudo cp man/ecd.1 /usr/local/share/man/man1/
```

## Development

```bash
make help      # list targets
make test      # run tests
make check     # fmt + clippy
make build     # release build
make man       # regenerate man page
make fixtures  # regenerate per-encoding test fixtures
```

Every supported encoding has a fixture under `tests/fixtures/encodings/` and a test in `tests/encodings.rs`.

## License

MIT — see [LICENSE](LICENSE).
