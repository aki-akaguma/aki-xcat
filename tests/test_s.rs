#[path = "./common/macros.rs"]
#[macro_use]
mod macros;

#[rustfmt::skip]
macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe.perr().lock()
                .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_0 {
    use libaki_xcat::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(&["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[] as &[&str], "abcdefg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_stdin() {
        let (r, sioe) = do_execute!(&["--", "-"], "abcdefg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\n");
        assert!(r.is_ok());
    }
}

mod test_2 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_mini() {
        let (r, sioe) = do_execute!(&[fixture_mini!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_plain() {
        let (r, sioe) = do_execute!(&[fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_gz() {
        let (r, sioe) = do_execute!(&[fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_xz() {
        let (r, sioe) = do_execute!(&[fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        let (r, sioe) = do_execute!(&[fixture_zstd!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "lz4")]
    #[test]
    fn test_lz4() {
        let (r, sioe) = do_execute!(&[fixture_lz4!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_bzip2() {
        let (r, sioe) = do_execute!(&[fixture_bzip2!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2() {
        let (r, sioe) = do_execute!(
            &[
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "abcdefg\nhijklmn\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
                "ABCDEFG\nHIJKLMN\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_fnm_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                "-f",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[cfg(feature = "lz4")]
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_pnm_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                "--path-name",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_gz() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!(gz)], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_lz4() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!(lz4)], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_xz() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!(xz)], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_zstd() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!(zstd)], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_bzip2() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!(bzip2)], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), invalid_utf8_result!());
        assert!(r.is_ok());
    }
}

mod test_3 {
    /*
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}

mod test_4 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_bin_plain() {
        let (r, sioe) = do_execute!(&["-b", fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        #[cfg(not(windows))]
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        #[cfg(windows)]
        assert_eq!(buff!(sioe, sout), "abcdefg\r\nhijklmn\r\n");
        assert!(r.is_ok());
    }
}
