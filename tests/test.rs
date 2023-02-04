const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-xcat [options] [<file>...]

            this is like a cat, zcat, xzcat, zstdcat and lz4cat.
            with no <file> or when <file> is -, read standard input.
            automatic discovery file type: plain, gz, xz, zst and lz4.

            Options:
              -n, --number          output line number for each lines
              -f, --file-name       output file name for each lines
                  --path-name       output path name for each lines
              -p, --pipe-in <num>   read from pipe <num> [unimplemented]

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Argument:
              <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd, lz4.

            Examples:
              You can simple use. Just arrange the files.
                aki-xcat file1 file2.gz file3.xz file4.zst file5.lz4
            "#
            ),
            "\n"
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-xcat"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
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
}

//mod helper;

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_1 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;

    //
    #[test]
    fn test_non_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_stdin() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--", "-"], b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert!(oup.status.success());
    }
}

mod test_2 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_plain() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_gz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_gz!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_xz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_xz!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_zstd!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    #[cfg(feature = "lz4")]
    #[test]
    fn test_lz4() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_lz4!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[test]
    fn test_plain_gz_xz_zst_lz4() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "abcdefg\nhijklmn\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_num() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-n",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "     1\tabcdefg\n",
                "     2\thijklmn\n",
                "     3\tABCDEFG\n",
                "     4\tHIJKLMN\n",
                "     5\tABCDEFG\n",
                "     6\tHIJKLMN\n",
                "     7\tABCDEFG\n",
                "     8\tHIJKLMN\n",
                "     9\tABCDEFG\n",
                "    10\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_fnm_num() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-n",
                "-f",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"plain.txt\"     1\tabcdefg\n",
                "\"plain.txt\"     2\thijklmn\n",
                "\"gztext.txt.gz\"     1\tABCDEFG\n",
                "\"gztext.txt.gz\"     2\tHIJKLMN\n",
                "\"xztext.txt.xz\"     1\tABCDEFG\n",
                "\"xztext.txt.xz\"     2\tHIJKLMN\n",
                "\"zstext.txt.zst\"     1\tABCDEFG\n",
                "\"zstext.txt.zst\"     2\tHIJKLMN\n",
                "\"lz4text.txt.lz4\"     1\tABCDEFG\n",
                "\"lz4text.txt.lz4\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_pnm_num() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-n",
                "--path-name",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"fixtures/plain.txt\"     1\tabcdefg\n",
                "\"fixtures/plain.txt\"     2\thijklmn\n",
                "\"fixtures/gztext.txt.gz\"     1\tABCDEFG\n",
                "\"fixtures/gztext.txt.gz\"     2\tHIJKLMN\n",
                "\"fixtures/xztext.txt.xz\"     1\tABCDEFG\n",
                "\"fixtures/xztext.txt.xz\"     2\tHIJKLMN\n",
                "\"fixtures/zstext.txt.zst\"     1\tABCDEFG\n",
                "\"fixtures/zstext.txt.zst\"     2\tHIJKLMN\n",
                "\"fixtures/lz4text.txt.lz4\"     1\tABCDEFG\n",
                "\"fixtures/lz4text.txt.lz4\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!()]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_gz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(gz)]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(gz),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_lz4() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(lz4)]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(lz4),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_xz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(xz)]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(xz),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_zstd() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(zstd)]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(zstd),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "\"{}\" \"{}\" | head -n 2",
            TARGET_EXE_PATH,
            fixture_text10k!()
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
}
