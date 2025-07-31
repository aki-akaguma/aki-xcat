macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "
",
            indoc::indoc!(
                r#"
            Usage:
              aki-xcat [options] [<file>...]

            this is like a cat, zcat, xzcat, zstdcat, lz4cat and bzcat.
            with no <file> or when <file> is -, read standard input.
            automatic discovery file type: plain, gz, xz, zst, lz4 and bzip2.

            Options:
              -b, --bin             binary mode
              -n, --number          output line number for each lines
              -f, --file-name       output file name for each lines
                  --path-name       output path name for each lines

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Argument:
              <file>         utf-8 encoded text file or binary file.
                             A compressed file of it by gzip, xz, zstd, lz4, bzip2.

            Examples:
              You can simple use. Just arrange the files.
                aki-xcat file1 file2.gz file3.xz file4.zst file5.lz4 file6.bz2
            "#
            ),
            "
"
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.
"
    };
}

macro_rules! program_name {
    () => {
        "aki-xcat"
    };
}

macro_rules! version_msg {
    () => {
        concat!(
            program_name!(),
            " ",
            env!("CARGO_PKG_VERSION"),
            "
"
        )
    };
}

macro_rules! fixture_mini {
    () => {
        "fixtures/mini.txt"
    };
}
macro_rules! fixture_plain {
    () => {
        "fixtures/plain.txt"
    };
}
macro_rules! fixture_gz {
    () => {
        "fixtures/gztext.txt.gz"
    };
}
macro_rules! fixture_xz {
    () => {
        "fixtures/xztext.txt.xz"
    };
}
macro_rules! fixture_zstd {
    () => {
        "fixtures/zstext.txt.zst"
    };
}
macro_rules! fixture_lz4 {
    () => {
        "fixtures/lz4text.txt.lz4"
    };
}
macro_rules! fixture_bzip2 {
    () => {
        "fixtures/bzip2text.txt.bz2"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt.gz"
    };
}

macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
    (gz) => {
        "fixtures/invalid_utf8.txt.gz"
    };
    (lz4) => {
        "fixtures/invalid_utf8.txt.lz4"
    };
    (xz) => {
        "fixtures/invalid_utf8.txt.xz"
    };
    (zstd) => {
        "fixtures/invalid_utf8.txt.zst"
    };
    (bzip2) => {
        "fixtures/invalid_utf8.txt.bz2"
    };
}
macro_rules! invalid_utf8_result {
    () => {
        "���\n"
    };
}
