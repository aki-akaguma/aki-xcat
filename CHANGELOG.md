aki-xcat TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.23 (2021-04-23)
=====

* fix build.rs

0.1.22 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* add command option: -X
* update depends: bug fix: regex(1.4.6)

0.1.21 (2021-04-19)
=====

* update depends: flood-tide-gen(0.1.10)

0.1.20 (2021-04-07)
=====

* update depends: flood-tide(0.2), zstd(0.7)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

0.1.19 (2021-03-22)
=====

* update depends: anyhow, libflate

0.1.18 (2021-03-08)
=====

* update crate: runnel
* update crate: rustc_version ("0.3")

0.1.17 (2021-03-08)
=====

* update crate: runnel

0.1.16 (2021-03-05)
=====

* output line number for each lines
* output file or path name for each lines
* update crates

0.1.15 (2021-03-03)
=====

* bug fix: double buffer in adapt_input()
* update crate: runnel
* add lz4
* bug fix: signature match of input file

0.1.14 (2021-03-03)
=====

* add option: '-p, --pipe-in <num>   write to pipe <num> [unimplemented]'
* fix misspell

0.1.13 (2021-02-24)
=====

* fix error display
* add support xz2, zstd
* update dependencies: flate2

0.1.12 (2021-02-22)
=====

* fix bug: add flush() on finish.
* update crate: runnel, flood-tide-gen

0.1.11 (2021-02-17)
=====

* update crate runnel
* add doc
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

0.1.10 (2021-02-07)
=====

* initial github

0.1.9 (2021-02-07)
=====

* import crate exec-target from local, for test.
* add xtask
* add stream module
* change optpa_util_1 to flood-tide and flood-tied-gen
* change AppError to anyhow::Error

0.1.8 (2020-12-29)
=====

* update crates
* remove optpaerr-1

0.1.7 (2020-11-17)
=====

* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster
* add support cargo deb
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* change optpa_util to optpa_util_1

0.1.6 (2020-05-10)
=====

* update crates

0.1.5 (2020-05-10)
=====

* change edition 2015 to 2018.
* update crates

0.1.4 (2020-03-30)
=====

* add support broken pipe and test
* update crates

0.1.3 (2019-04-14)
=====

* add rustc version info
* update crates
* a lot of things

0.1.1 (2018-05-22)
=====

* add support cfg(has_global_allocator)
* add support libflate,flate2 and flate backends
* update crates
* a lot of things

0.1.0 (2017-12-16)
=====
first commit
