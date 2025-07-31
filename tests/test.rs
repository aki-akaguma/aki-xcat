const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[path = "./common/macros.rs"]
#[macro_use]
mod macros;

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
    fn test_mini() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_mini!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a\n");
        assert!(oup.status.success());
    }
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
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_bzip2() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_bzip2!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
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
                "ABCDEFG\nHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_num() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-n",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
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
                "    11\tABCDEFG\n",
                "    12\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_fnm_num() {
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
                fixture_bzip2!(),
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
                "\"bzip2text.txt.bz2\"     1\tABCDEFG\n",
                "\"bzip2text.txt.bz2\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_pnm_num() {
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
                fixture_bzip2!(),
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
                "\"fixtures/bzip2text.txt.bz2\"     1\tABCDEFG\n",
                "\"fixtures/bzip2text.txt.bz2\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_gz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(gz)]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_lz4() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(lz4)]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_xz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(xz)]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_zstd() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(zstd)]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_bzip2() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_invalid_utf8!(bzip2)]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, invalid_utf8_result!());
        assert!(oup.status.success());
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

mod test_4 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_bin_plain() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        #[cfg(not(windows))]
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        #[cfg(windows)]
        assert_eq!(oup.stdout, "abcdefg\r\nhijklmn\r\n");
        assert!(oup.status.success());
    }
}
