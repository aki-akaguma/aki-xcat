# Changelog: aki-xcat

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Include code review report: `docs/reviews/2026-06-01_code_review.3.md`

### Changed
- Reorganize code review reports into `docs/reviews/`
- Use `std::path::Path::join` for cross-platform path handling

### Fixed
- Correct identification of small files as plain text in `detect_file_type`

## [0.2.2] - 2026-05-20

### Added
- Include unit tests for `TextDecorator` struct in `src/run.rs`

### Changed
- Update dependencies: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2)
- Patch dependency: `getrandom` (0.3.4)
- Update dependency: `runnel` (0.4.2)
- Refactor file type detection logic with named constants and improved documentation
- Simplify file type detection logic using `read()` instead of `read_exact()` to avoid explicit `UnexpectedEof` handling
- Introduce `TextDecorator` struct in `src/run.rs` to improve maintainability of decoration logic

### Fixed
- Resolve Clippy warnings: `uninlined_format_args`, `needless_borrow`

### Removed
- Delete `memx-cdy` dependency

## [0.2.1] - 2025-09-15

### Added
- Incorporate `specs` directory
- Include support for `-X base_dir=dir` option
- Include additional tests

### Changed
- Improve `IntoIterator` compatibility for arguments in `execute()`
- Update dependency: `runnel` (0.4.0)
- Update dependency: `rust-version-info-file` (0.2)
- Refactor source code

### Fixed
- Resolve Clippy warnings: `derivable_impls`, `needless_lifetimes`
- Update Rust version requirement: 1.75.0

## [0.2.0] - 2025-07-31

### Added
- Include binary mode (`-b`) and default text mode
- Include support for bzip2 compression

### Changed
- Replace invalid UTF-8 with U+FFFD in text mode instead of returning an error
- Update dependencies: `anyhow` (1.0.98), `indoc` (2.0.6)
- Update dependencies: `flate2` (1.1.2), `lz4` (1.28.1), `zstd` (0.13.3)
- Update minimum supported Rust version (MSRV) to 1.75.0

### Removed
- Exclude `--pipe-in` option

## [0.1.36] - 2024-06-19

### Added
- Include GitHub Actions workflows: `test-ubuntu.yml`, `test-macos.yml`, `test-windows.yml`
- Include test status badges to `README.tpl`
- Include Miri support for tests
- Include Tarpaulin support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Exclude `cfg(has_not_matches)`
- Refactor `Makefile`
- Update dependencies: `flood-tide` (0.2.9), `flood-tide-gen` (0.1.20)
- Update dependencies: `memx-cdy` (0.1.11), `runnel` (0.3.16)
- Update dependencies: `exec-target` (0.2.8), `indoc` (2.0.5), `rust-version-info-file` (0.1.8)
- Update dependencies: `libflate` (2.1.0), `zstd` (0.13.1)

### Removed
- Delete `COPYING` file

### Fixed
- Resolve License files: `LICENSE-APACHE`, `LICENSE-MIT`
- Resolve Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
- Resolve Clippy warnings: `uninlined_format_args`, `unused_imports`, `derivable_impls`
- Update Rust version requirement: 1.56.0 to 1.65.0

## [0.1.35] - 2023-01-11

### Added
- Include badges to `README.tpl`
- Include Rust version requirement (1.56.0) in `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependency: `anyhow` (1.0.68)
- Update dependencies: `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19)
- Update dependencies: `memx-cdy` (0.1.10), `runnel` (0.3.15)
- Update dependencies: `flate2` (1.0.25), `lz4` (1.24.0), `xz2` (0.1.7)
- Update dependency: `zstd` (0.12.1+zstd.1.5.2)

### Fixed
- Resolve Clippy warnings: `Eq` implementation for `PartialEq` types, `uninlined_format_args`, `seek_to_start_instead_of_rewind`

## [0.1.34] - 2022-06-18

### Fixed
- Resolve Git log issues

## [0.1.33] - 2022-06-18

### Changed
- Migrate to 2021 edition
- Update dependencies: `cfg-iif` (0.2.3), `flood-tide` (0.2.5), `linux-procfs` (0.3.11)
- Update dependencies: `memx` (0.1.21), `memx-cdy` (0.1.8), `naive_opt` (0.1.18), `runnel` (0.3.11)
- Update dependencies: `assert-text` (0.2.6), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16)
- Update dependency: `rust-version-info-file` (0.1.6)
- Update dependency: `semver` (1.0.10)
- Update dependencies: `flate2` (1.0.24), `lzma-sys` (0.1.19), `miniz_oxide` (0.5.3), `xz2` (0.1.7)

## [0.1.32] - 2022-05-22

### Changed
- Update dependency: `zstd` (0.11.2+zstd.1.5.2)

## [0.1.31] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20)
- Update dependencies: `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6)
- Update dependencies: `flate2` (1.0.23), `lz4` (1.23.3), `zstd` (0.9.2+zstd.1.5.1)
- Update dependencies: `exec-target` (0.2.5), `rust-version-info-file` (0.1.5)

## [0.1.30] - 2021-11-15

### Added
- Include documentation improvements

## [0.1.29] - 2021-11-15

### Added
- Include documentation improvements

### Changed
- Update minimum supported Rust version (MSRV) to 1.51.0
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9)
- Update dependencies: `anyhow` (1.0.45), `cc` (1.0.72), `flate2` (1.0.22), `libc` (0.2.107), `pkg-config` (0.3.22)
- Update dependencies: `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `rust-version-info-file` (0.1.3)

## [0.1.28] - 2021-09-11

### Changed
- Update dependency: `flate2` (1.0.21)

## [0.1.27] - 2021-09-11

### Added
- Include dependency: `indoc` (1.0.3)

### Changed
- Resolve Clippy warnings
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8)
- Update dependency: `libflate` (1.1.1)
- Rewrite `TARGET_EXE_PATH` using `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
- Update dependency: `exec-target` (0.2.3)

## [0.1.26] - 2021-06-24

### Added
- Include `memx_cdy::memx_init()` for fast memory operations

### Changed
- Rewrite `TARGET_EXE_PATH` using `env!("CARGO_BIN_EXE_aki-xcat")`
- Update dependency: `zstd` (0.9.0+zstd.1.5.0)

### Fixed
- Correct bug in `#[cfg(feature = "debian_build")]`

## [0.1.25] - 2021-06-06

### Changed
- Update dependency: `zstd` (0.8.3+zstd.1.5.0)

### Fixed
- Correct inadequate error message for compressed files containing invalid UTF-8 sequences

## [0.1.24] - 2021-06-03

### Added
- Include support for `debian_build` feature

### Changed
- Update dependency: `flood-tide` (0.2.2)
- Update dependency: `regex` (1.5.4)

### Fixed
- Correct bug in `-X rust-version-info` command option

## [0.1.23] - 2021-04-23

### Fixed
- Correct bug in `build.rs`

## [0.1.22] - 2021-04-23

### Added
- Include command option: `-X`

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1)
- Update dependency: `regex` (1.4.6)

## [0.1.21] - 2021-04-19

### Changed
- Update dependency: `flood-tide-gen` (0.1.10)

## [0.1.20] - 2021-04-07

### Changed
- Update dependencies: `flood-tide` (0.2), `zstd` (0.7)
- Update dependencies: `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), `runnel` (0.3.6)

## [0.1.19] - 2021-03-22

### Changed
- Update dependencies: `anyhow`, `libflate`

## [0.1.18] - 2021-03-08

### Changed
- Update dependency: `runnel`
- Update dependency: `rustc_version` (0.3)

## [0.1.17] - 2021-03-08

### Changed
- Update dependency: `runnel`

## [0.1.16] - 2021-03-05

### Changed
- Output line numbers for each line
- Output file or path names for each line
- Update dependencies

## [0.1.15] - 2021-03-03

### Added
- Include support for lz4 compression

### Changed
- Update dependency: `runnel`

### Fixed
- Correct double buffering in `adapt_input()`
- Correct signature matching for input files

## [0.1.14] - 2021-03-03

### Added
- Include option: `-p, --pipe-in <num>` (unimplemented)

### Fixed
- Correct misspellings

## [0.1.13] - 2021-02-24

### Added
- Include support for `xz2` and `zstd`

### Changed
- Update dependency: `flate2`

### Fixed
- Correct error display bug

## [0.1.12] - 2021-02-22

### Changed
- Update dependencies: `runnel`, `flood-tide-gen`

### Fixed
- Ensure `flush()` is called on finish

## [0.1.11] - 2021-02-17

### Added
- Include documentation

### Changed
- Update dependency: `runnel`
- Rename `AAA-admin` section to `AAA-text` in `package.metadata.deb`

## [0.1.10] - 2021-02-07

### Changed
- Initial release on GitHub

## [0.1.9] - 2021-02-07

### Added
- Include xtask
- Include stream module

### Changed
- Import `exec-target` crate from local for testing
- Switch from `optpa_util_1` to `flood-tide` and `flood-tide-gen`
- Replace `AppError` with `anyhow::Error`

## [0.1.8] - 2020-12-29

### Changed
- Update dependencies

### Removed
- Exclude `optpaerr-1` dependency

## [0.1.7] - 2020-11-17

### Added
- Include support for `cargo deb`
- Include `README.md`, `LICENSE-APACHE`, `LICENSE-MIT` files

### Changed
- Switch from `optpa_util` to `optpa_util_1`

### Fixed
- Correct compatibility issue with `rustc_version` (=0.2.3) on deb10-buster

## [0.1.6] - 2020-05-10

### Changed
- Update dependencies

## [0.1.5] - 2020-05-10

### Changed
- Migrate from 2015 edition to 2018 edition
- Update dependencies

## [0.1.4] - 2020-03-30

### Added
- Include support for broken pipes and associated tests

### Changed
- Update dependencies

## [0.1.3] - 2019-04-14

### Added
- Include `rustc` version information

### Changed
- Update dependencies

## [0.1.1] - 2018-05-22

### Added
- Include support for `cfg(has_global_allocator)`
- Include support for `libflate`, `flate2`, and `flate` backends

### Changed
- Update dependencies

## [0.1.0] - 2017-12-16

### Added
- Include first commit

[Unreleased]: https://github.com/aki-akaguma/aki-xcat/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/aki-akaguma/aki-xcat/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/aki-akaguma/aki-xcat/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.36...v0.2.0
[0.1.36]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.35...v0.1.36
[0.1.35]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.34...v0.1.35
[0.1.34]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.33...v0.1.34
[0.1.33]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.32...v0.1.33
[0.1.32]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.31...v0.1.32
[0.1.31]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.30...v0.1.31
[0.1.30]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.29...v0.1.30
[0.1.29]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.28...v0.1.29
[0.1.28]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.27...v0.1.28
[0.1.27]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.26...v0.1.27
[0.1.26]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.25...v0.1.26
[0.1.25]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.24...v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.23...v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.22...v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.21...v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-xcat/releases/tag/v0.1.10
