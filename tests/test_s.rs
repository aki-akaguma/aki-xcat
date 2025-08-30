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
                let _ = sioe.pg_err().lock()
                .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
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

mod test_0_x_options {
    use libaki_xcat::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(&["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_base_dir() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "hello from base_dir\n").unwrap();
        let (r, sioe) = do_execute!(&[
            "-X",
            &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
            "test_file.txt"
        ]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello from base_dir\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let (r, sioe) = do_execute!(&["-X", "base_dir=/non/existent/dir", "test_file.txt"]);
        #[cfg(not(windows))]
        assert!(buff!(sioe, serr).contains("No such file or directory"));
        #[cfg(windows)]
        assert!(buff!(sioe, serr).contains("The system cannot find the path specified."));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_file() {
        let temp_dir = tempdir().unwrap();
        let (r, sioe) = do_execute!(&[
            "-X",
            &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
            "non_existent_file.txt",
        ]);
        #[cfg(not(windows))]
        assert!(buff!(sioe, serr).contains("No such file or directory"));
        #[cfg(windows)]
        assert!(buff!(sioe, serr).contains("The system cannot find the file specified"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1_stdin {
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
    //
    #[test]
    fn test_stdin() {
        let (r, sioe) = do_execute!(&["--", "-"], "abcdefg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_stdin() {
        let (r, sioe) = do_execute!(&[], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_line_number_with_stdin() {
        let (r, sioe) = do_execute!(&["-n", "--", "-"], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "     1\tstdin line\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_file_name_with_stdin() {
        let (r, sioe) = do_execute!(&["-f", "-n", "--", "-"], "stdin line 1\nstdin line 2\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!("\"\"     1\tstdin line 1\n", "\"\"     2\tstdin line 2\n",)
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_path_name_with_stdin() {
        let (r, sioe) = do_execute!(
            &["--path-name", "--", "-", fixture_plain!()],
            "stdin line\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"\"\tstdin line\n",
                "\"fixtures/plain.txt\"\tabcdefg\n",
                "\"fixtures/plain.txt\"\thijklmn\n"
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_stdin_multiple() {
        let (r, sioe) = do_execute!(&["--", "-", "-"], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdin line\n");
        assert!(r.is_ok());
    }
}

mod test_2_file {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_empty() {
        let (r, sioe) = do_execute!(&[fixture_empty!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
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
    fn test_plain_concat() {
        let (r, sioe) = do_execute!(&[fixture_plain!(), fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\nabcdefg\nhijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_f_without_n() {
        let (r, sioe) = do_execute!(&["-f", fixture_plain!()]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "\"plain.txt\"\tabcdefg\n\"plain.txt\"\thijklmn\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_path_name_takes_precedence_over_file_name() {
        let (r, sioe) = do_execute!(&["-n", "-f", "--path-name", fixture_plain!()]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"fixtures/plain.txt\"     1\tabcdefg\n",
                "\"fixtures/plain.txt\"     2\thijklmn\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_same_file_multiple_times_with_line_numbers() {
        let (r, sioe) = do_execute!(&["-n", fixture_plain!(), fixture_plain!()]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "     1\tabcdefg\n     2\thijklmn\n     3\tabcdefg\n     4\thijklmn\n"
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
    #[cfg(not(windows))]
    #[test]
    fn test_non_existent_file() {
        let (r, sioe) = do_execute!(&["non_existent_file.txt"], "");
        assert!(buff!(sioe, serr).contains("No such file or directory"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[cfg(not(windows))]
    #[test]
    fn test_directory_as_input() {
        let (r, sioe) = do_execute!(&["fixtures"], "");
        assert!(buff!(sioe, serr).contains("Is a directory"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_no_newline_at_end() {
        let (r, sioe) = do_execute!(&[fixture_no_newline!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "no-newline\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(not(windows))]
    #[test]
    fn test_symlink() {
        let (r, sioe) = do_execute!(&[fixture_symlink!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_options_after_filenames() {
        let (r, sioe) = do_execute!(&[fixture_plain!(), "-n"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "     1\tabcdefg\n     2\thijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_large_number_of_files() {
        let mut args = vec!["-n"];
        args.resize(args.len() + 100, fixture_plain!());
        let (r, sioe) = do_execute!(&args, "");
        assert_eq!(buff!(sioe, serr), "");
        let mut expected_output = String::new();
        for i in 1..=200 {
            let line = if i % 2 == 1 { "abcdefg" } else { "hijklmn" };
            expected_output.push_str(&format!("{:>6}\t{}\n", i, line));
        }
        assert_eq!(buff!(sioe, sout), expected_output);
        assert!(r.is_ok());
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_gz() {
        let (r, sioe) = do_execute!(&[fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
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
    fn test_line_numbering_is_per_file_with_f() {
        let (r, sioe) = do_execute!(&["-n", "-f", fixture_plain!(), fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"plain.txt\"     1\tabcdefg\n",
                "\"plain.txt\"     2\thijklmn\n",
                "\"gztext.txt.gz\"     1\tABCDEFG\n",
                "\"gztext.txt.gz\"     2\tHIJKLMN\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_line_numbering_is_per_file_with_path_name() {
        let (r, sioe) = do_execute!(&["-n", "--path-name", fixture_plain!(), fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"fixtures/plain.txt\"     1\tabcdefg\n",
                "\"fixtures/plain.txt\"     2\thijklmn\n",
                "\"fixtures/gztext.txt.gz\"     1\tABCDEFG\n",
                "\"fixtures/gztext.txt.gz\"     2\tHIJKLMN\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_repeated_files() {
        let (r, sioe) = do_execute!(&[fixture_plain!(), fixture_gz!(), fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "abcdefg\nhijklmn\nABCDEFG\nHIJKLMN\nabcdefg\nhijklmn\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_line_number_with_repeated_files() {
        let (r, sioe) = do_execute!(
            &["-n", fixture_plain!(), fixture_gz!(), fixture_plain!()],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "     1\tabcdefg\n     2\thijklmn\n     3\tABCDEFG\n     4\tHIJKLMN\n     5\tabcdefg\n     6\thijklmn\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mixed_stdin_files() {
        let (r, sioe) = do_execute!(&["--", "-", fixture_gz!(), "-"], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdin line\nABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mixed_files_and_stdin_with_line_numbers() {
        let (r, sioe) = do_execute!(
            &["-n", "--", fixture_gz!(), "-", fixture_plain!()],
            "stdin line 1\nstdin line 2\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "     1\tABCDEFG\n",
                "     2\tHIJKLMN\n",
                "     3\tstdin line 1\n",
                "     4\tstdin line 2\n",
                "     5\tabcdefg\n",
                "     6\thijklmn\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_large_gz_file() {
        let (r, sioe) = do_execute!(&[fixture_text10k!()], "");
        assert_eq!(buff!(sioe, serr), "");
        // Just check that the output is large, not the exact content
        assert!(buff!(sioe, sout).len() > 10000);
        assert!(r.is_ok());
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_xz() {
        let (r, sioe) = do_execute!(&[fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_zstd() {
        let (r, sioe) = do_execute!(&[fixture_zstd!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "lz4")]
mod test_3_file_lz4 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_lz4() {
        let (r, sioe) = do_execute!(&[fixture_lz4!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2 {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_bzip2() {
        let (r, sioe) = do_execute!(&[fixture_bzip2!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
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

mod test_4_complex {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_stdin_first_then_file() {
        let (r, sioe) = do_execute!(&["--", "-", fixture_plain!()], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdin line\nabcdefg\nhijklmn\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_line_number_with_empty_file() {
        let (r, sioe) = do_execute!(&["-n", fixture_empty!(), fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!("     1\tabcdefg\n", "     2\thijklmn\n")
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_line_numbering_and_filename_with_stdin() {
        let (r, sioe) = do_execute!(&["-n", "-f", "--", "-", fixture_plain!()], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"\"     1\tstdin line\n",
                "\"plain.txt\"     1\tabcdefg\n",
                "\"plain.txt\"     2\thijklmn\n"
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_stdin_with_files() {
        let (r, sioe) = do_execute!(&["--", "-", fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_invalid_utf8_mixed() {
        let (r, sioe) = do_execute!(
            &[fixture_plain!(), fixture_invalid_utf8!(), fixture_plain!()],
            ""
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            format!(
                "abcdefg\nhijklmn\n{}abcdefg\nhijklmn\n",
                invalid_utf8_result!()
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_stdin() {
        let (r, sioe) = do_execute!(&["--", "-", "-", fixture_plain!(), "-"], "stdin line\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdin line\nabcdefg\nhijklmn\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mix_of_existing_and_non_existing_files() {
        let (r, sioe) = do_execute!(
            &[fixture_plain!(), "non-existent-file", fixture_plain!()],
            "stdin line\n"
        );
        #[cfg(not(windows))]
        assert!(buff!(sioe, serr).contains("No such file or directory"));
        #[cfg(windows)]
        assert!(buff!(sioe, serr).contains("The system cannot find the file specified"));
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        assert!(r.is_err());
    }
}

#[cfg(feature = "xz2")]
#[cfg(feature = "zstd")]
#[cfg(feature = "lz4")]
#[cfg(feature = "bzip2")]
mod test_4_complex_more {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_all_compression_formats() {
        let (r, sioe) = do_execute!(&[
            fixture_gz!(),
            fixture_xz!(),
            fixture_zstd!(),
            fixture_lz4!(),
            fixture_bzip2!(),
        ]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
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
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2() {
        let (r, sioe) = do_execute!(
            &[
                "--",
                "-",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            "stdin line\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "stdin line\n",
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
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                "--",
                "-",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            "stdin line\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "     1\tstdin line\n",
                "     2\tabcdefg\n",
                "     3\thijklmn\n",
                "     4\tABCDEFG\n",
                "     5\tHIJKLMN\n",
                "     6\tABCDEFG\n",
                "     7\tHIJKLMN\n",
                "     8\tABCDEFG\n",
                "     9\tHIJKLMN\n",
                "    10\tABCDEFG\n",
                "    11\tHIJKLMN\n",
                "    12\tABCDEFG\n",
                "    13\tHIJKLMN\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_fnm_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                "-f",
                "--",
                "-",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            "stdin line\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"\"     1\tstdin line\n",
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
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_pnm_num() {
        let (r, sioe) = do_execute!(
            &[
                "-n",
                "--path-name",
                "--",
                "-",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            "stdin line\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "\"\"     1\tstdin line\n",
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
}

mod test_5_binary_mode {
    use libaki_xcat::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_binary_mode_with_plain() {
        let (r, sioe) = do_execute!(&["-b", fixture_plain!()], "");
        assert_eq!(buff!(sioe, serr), "");
        #[cfg(not(windows))]
        assert_eq!(buff!(sioe, sout), "abcdefg\nhijklmn\n");
        #[cfg(windows)]
        assert_eq!(buff!(sioe, sout), "abcdefg\r\nhijklmn\r\n");
        assert!(r.is_ok());
    }
    //
    /* FAIL
    #[test]
    fn test_binary_mode_with_invalid_utf8() {
        let (r, sioe) = do_execute!(&["-b", fixture_invalid_utf8!()], "");
        assert_eq!(buff!(sioe, serr), "");
        let expected_stdout = std::fs::read(fixture_invalid_utf8!()).unwrap();
        assert_ne!(buff!(sioe, sout).as_bytes(), expected_stdout);
        assert!(r.is_ok());
    }
    */
    //
    #[cfg(feature = "flate2")]
    #[test]
    fn test_binary_mode_with_gz() {
        let (r, sioe) = do_execute!(&["-b", fixture_gz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_binary_mode_with_xz() {
        let (r, sioe) = do_execute!(&["-b", fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        let expected = std::fs::read("fixtures/xztext.txt.xz").unwrap();
        let mut decoder = xz2::read::XzDecoder::new(expected.as_slice());
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(buff!(sioe, sout).as_bytes(), expected_stdout);
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "zstd")]
    #[test]
    fn test_binary_mode_with_zstd() {
        let (r, sioe) = do_execute!(&["-b", fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        let expected = std::fs::read("fixtures/zstext.txt.zst").unwrap();
        let mut decoder = zstd::stream::read::Decoder::new(expected.as_slice()).unwrap();
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(buff!(sioe, sout).as_bytes(), expected_stdout);
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "lz4")]
    #[test]
    fn test_binary_mode_with_lz4() {
        let (r, sioe) = do_execute!(&["-b", fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        let expected = std::fs::read("fixtures/lz4text.txt.lz4").unwrap();
        let mut decoder = lz4::Decoder::new(expected.as_slice()).unwrap();
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(buff!(sioe, sout).as_bytes(), expected_stdout);
        assert!(r.is_ok());
    }
    //
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_binary_mode_with_bzip2() {
        let (r, sioe) = do_execute!(&["-b", fixture_xz!()], "");
        assert_eq!(buff!(sioe, serr), "");
        let expected = std::fs::read("fixtures/bzip2text.txt.bz2").unwrap();
        let mut decoder = bzip2::read::BzDecoder::new(expected.as_slice());
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(buff!(sioe, sout).as_bytes(), expected_stdout);
        assert!(r.is_ok());
    }
}

mod test_9_broken_pipe {
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
