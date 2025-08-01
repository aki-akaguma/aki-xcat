# Changelog: aki-xcat

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
* refactoring source codes

## [0.2.0] (2025-07-31)
### Added
* `-b` binary mode and default text mode
* bzip2 supports

### Changed
* In text mode, invalid UTF-8 is replaced with U+FFFD without error
* update depends: anyhow(1.0.98), indoc(2.0.6)
* update depends: flate2(1.1.2), lz4(1.28.1), zstd(0.13.3)
* minimum support rustc 1.75.0

### Removed
* option `--pipe-in`

## [0.1.36] (2024-06-19)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* miri supports on tests
* `tarpaulin` supports into `Makefile`

### Changed
* rename: `config` to `config.toml`
* remove: `cfg(has_not_matches)`
* refactored `Makefile`
* update depends: flood-tide(0.2.9), flood-tide-gen(0.1.20)
* update depends: memx-cdy(0.1.11), runnel(0.3.16)
* update depends: exec-taget(0.2.8), indoc(2.0.5), rust-version-info-file(0.1.8)
* update depends: libflate(2.1.0), zstd(0.13.1)

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* license files
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* clippy: `uninlined_format_args`, `unused_imports`, `derivable_impls`
* rust-version: "1.56.0" to "1.65.0"

## [0.1.35] (2023-01-11)
### Added
* badges into `README.tpl`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: flate2(1.0.25), lz4(1.24.0), xz2(0.1.7)
* update depends: zstd(0.12.1+zstd.1.5.2)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: uninlined_format_args, seek_to_start_instead_of_rewind

## [0.1.34] (2022-06-18)
### Fixed
* git log

## [0.1.33] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: cfg-iif(0.2.3), flood-tide(0.2.5), linux-procfs(0.3.11)
* update depends: memx(0.1.21), memx-cdy(0.1.8), naive_opt(0.1.18), runnel(0.3.11)
* update depends: assert-text(0.2.6), exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: flate2(1.0.24), lzma-sys(0.1.19), miniz_oxide(0.5.3), xz2(0.1.7)

## [0.1.32] (2022-05-22)
### Changed
* update depends: zstd(0.11.2+zstd.1.5.2)

## [0.1.31] (2022-05-22)
### Changed
* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: flate2(1.0.23), lz4(1.23.3), zstd(0.9.2+zstd.1.5.1)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.30] (2021-11-15)
### Added
* more documents

## [0.1.29] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.51.0 (2fd73fabe 2021-03-23)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), cc(1.0.72), flate2(v1.0.22), libc(0.2.107), pkg-config(0.3.22)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.28] (2021-09-11)
### Changed
* update crates: flate2(1.0.21)

## [0.1.27] (2021-09-11)
### Added
* depends: indoc(1.0.3)

### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* update crates: libflate(1.1.1)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.26] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-xcat")`
* update depends: zstd(0.9.0+zstd.1.5.0)

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.25] (2021-06-06)
### Changed
* update depends: zstd(0.8.3+zstd.1.5.0)

### Fixed
* bug: output a meager error message for compressed files containing invalid UTF-8 sequence

## [0.1.24] (2021-06-03)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug: command option: -X rust-version-info

## [0.1.23] (2021-04-23)
### Fixed
* bug: build.rs

## [0.1.22] (2021-04-23)
### Added
* command option: `-X`

### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## [0.1.21] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.20] (2021-04-07)
### Changed
* update depends: flood-tide(0.2), zstd(0.7)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

## [0.1.19] (2021-03-22)
### Changed
* update depends: anyhow, libflate

## [0.1.18] (2021-03-08)
### Changed
* update crate: runnel
* update crate: rustc_version ("0.3")

## [0.1.17] (2021-03-08)
### Changed
* update crate: runnel

## [0.1.16] (2021-03-05)
### Changed
* output line number for each lines
* output file or path name for each lines
* update crates

## [0.1.15] (2021-03-03)
### Added
* add lz4

### Changed
* update crate: runnel

### Fixed
* bug: double buffer in adapt_input()
* bug: signature match of input file

## [0.1.14] (2021-03-03)
### Added
* add option: '-p, --pipe-in <num>   write to pipe <num> [unimplemented]'

### Fixed
* fix misspell

## [0.1.13] (2021-02-24)
### Added
* support `xz2`, `zstd`

### Changed
* update dependencies: flate2

### Fixed
* bug: error display

## [0.1.12] (2021-02-22)
### Changed
* update crate: runnel, flood-tide-gen

### Fixed
* fix: add flush() on finish.

## [0.1.11] (2021-02-17)
### Added
* add doc

### Changed
* update crate runnel
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

## [0.1.10] (2021-02-07)
### Changed
* initial github

## 0.1.9 (2021-02-07)
### Added
* add xtask
* add stream module

### Changed
* import crate exec-target from local, for test.
* change optpa_util_1 to flood-tide and flood-tied-gen
* change AppError to anyhow::Error

## 0.1.8 (2020-12-29)
### Changed
* update crates

### Removed
* remove optpaerr-1

## 0.1.7 (2020-11-17)
### Added
* support `cargo deb`
* `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`

### Changed
* change optpa_util to optpa_util_1

### Fixed
* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster

## 0.1.6 (2020-05-10)
### Changed
* update crates

## 0.1.5 (2020-05-10)
### Changed
* change edition 2015 to 2018.
* update crates

## 0.1.4 (2020-03-30)
### Added
* support broken pipe and test

### Changed
* update crates

## 0.1.3 (2019-04-14)
### Added
* add rustc version info

### Changed
* update crates
* a lot of things

## 0.1.1 (2018-05-22)
### Added
* support `cfg(has_global_allocator)`
* support `libflate`,`flate2` and `flate` backends

### Changed
* update crates
* a lot of things

## 0.1.0 (2017-12-16)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.36..HEAD
[0.1.36]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.35..v0.1.36
[0.1.35]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.34..v0.1.35
[0.1.34]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.33..v0.1.34
[0.1.33]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.32..v0.1.33
[0.1.32]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.31..v0.1.32
[0.1.31]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.30..v0.1.31
[0.1.30]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.29..v0.1.30
[0.1.29]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.28..v0.1.29
[0.1.28]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.27..v0.1.28
[0.1.27]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.26..v0.1.27
[0.1.26]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.25..v0.1.26
[0.1.25]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-xcat/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-xcat/releases/tag/v0.1.10
