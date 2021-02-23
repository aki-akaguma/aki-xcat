macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-xcat [options] [<file>...]\n",
            "\n",
            "this is a like cat, zcat, xzcat and zstdcat.\n",
            "with no <file> or when <file> is -, read standard input.\n",
            "automatic discovery file type: plain, gz, xz and zst.\n",
            "\n",
            "Options:\n",
            "  -H, --help     display this help and exit\n",
            "  -V, --version  display version information and exit\n",
            "\n",
            "Argument:\n",
            "  <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd.\n",
            "\n",
            "Examples:\n",
            "  You can simple use. Just arrange the files.\n",
            "    aki-xcat file1 file2.gz file3.xz file4.zst\n",
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
/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt.gz"
    };
}
*/
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[rustfmt::skip]
macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
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
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
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
        assert_eq!(r.is_err(), true);
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
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_stdin() {
        let (r, sioe) = do_execute!(&["--", "-"], "abcdefg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\n");
        assert_eq!(r.is_ok(), true);
    }
}

mod test_2 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_plain() {
        let (r, sioe) = do_execute!(&[fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_gz() {
        let (r, sioe) = do_execute!(&[fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_xz() {
        let (r, sioe) = do_execute!(&[fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        let (r, sioe) = do_execute!(&[fixture_zstd!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[test]
    fn test_plain_gz_xz() {
        let (r, sioe) = do_execute!(
            &[
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!()
            ],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "abcdefg\nhijklmn\nABCDEFG\nHIJKLMN\nABCDEFG\nHIJKLMN\nABCDEFG\nHIJKLMN\n"
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let (r, sioe) = do_execute!(&[fixture_invalid_utf8!()], "");
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": Failed to read from \'",
                fixture_invalid_utf8!(),
                "\': stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
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
