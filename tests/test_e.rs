const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
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

mod test_0_x_options_e {
    use exec_target::exec_target;
    use std::fs;
    use tempfile::tempdir;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(!oup.stdout.is_empty());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_base_dir() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "hello from base_dir\n").unwrap();
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "test_file.txt",
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello from base_dir\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-X", "base_dir=/non/existent/dir", "test_file.txt"],
        );
        #[cfg(not(windows))]
        assert!(oup.stderr.contains("No such file or directory"));
        #[cfg(windows)]
        assert!(oup
            .stderr
            .contains("The system cannot find the path specified."));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_file() {
        let temp_dir = tempdir().unwrap();
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "non_existent_file.txt",
            ],
        );
        #[cfg(not(windows))]
        assert!(oup.stderr.contains("No such file or directory"));
        #[cfg(windows)]
        assert!(oup
            .stderr
            .contains("The system cannot find the file specified"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_1_stdin_e {
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
    //
    #[test]
    fn test_stdin() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--", "-"], b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_stdin() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], b"" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_line_number_with_stdin() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-n", "-"], b"stdin line\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "     1\tstdin line\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_file_name_with_stdin() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-f", "-n", "-"],
            b"stdin line 1\nstdin line 2\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!("\"\"     1\tstdin line 1\n", "\"\"     2\tstdin line 2\n",)
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_path_name_with_stdin() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--path-name", "--", "-", fixture_plain!()],
            b"stdin line\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"\"\tstdin line\n",
                "\"fixtures/plain.txt\"\tabcdefg\n",
                "\"fixtures/plain.txt\"\thijklmn\n"
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_stdin_multiple() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--", "-", "-"], b"stdin line\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdin line\n");
        assert!(oup.status.success());
    }
    //
    /*
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--", "-"], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    */
}

mod test_2_file_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_empty() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_empty!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
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
    fn test_plain_concat() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_plain!(), fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\nabcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_f_without_n() {
        let oup = exec_target(TARGET_EXE_PATH, ["-f", fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "\"plain.txt\"\tabcdefg\n\"plain.txt\"\thijklmn\n",
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_path_name_takes_precedence_over_file_name() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-n", "-f", "--path-name", fixture_plain!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"fixtures/plain.txt\"     1\tabcdefg\n",
                "\"fixtures/plain.txt\"     2\thijklmn\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_same_file_multiple_times_with_line_numbers() {
        let oup = exec_target(TARGET_EXE_PATH, ["-n", fixture_plain!(), fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "     1\tabcdefg\n     2\thijklmn\n     3\tabcdefg\n     4\thijklmn\n"
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
    #[cfg(not(windows))]
    #[test]
    fn test_non_existent_file() {
        let oup = exec_target(TARGET_EXE_PATH, ["non_existent_file.txt"]);
        assert!(oup.stderr.contains("No such file or directory"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[cfg(not(windows))]
    #[test]
    fn test_directory_as_input() {
        let oup = exec_target(TARGET_EXE_PATH, ["fixtures"]);
        assert!(oup.stderr.contains("Is a directory"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_no_newline_at_end() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_no_newline!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "no-newline\n");
        assert!(oup.status.success());
    }
    //
    #[cfg(not(windows))]
    #[test]
    fn test_symlink() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_symlink!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_options_after_filenames() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_plain!(), "-n"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "     1\tabcdefg\n     2\thijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_large_number_of_files() {
        let mut args = vec!["-n"];
        args.resize(args.len() + 100, fixture_plain!());
        let oup = exec_target(TARGET_EXE_PATH, args);
        assert_eq!(oup.stderr, "");
        let mut expected_output = String::new();
        for i in 1..=200 {
            let line = if i % 2 == 1 { "abcdefg" } else { "hijklmn" };
            expected_output.push_str(&format!("{:>6}\t{}\n", i, line));
        }
        assert_eq!(oup.stdout, expected_output);
        assert!(oup.status.success());
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz_e {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_gz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_gz!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
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
    fn test_line_numbering_is_per_file_with_f() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-n", "-f", fixture_plain!(), fixture_gz!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"plain.txt\"     1\tabcdefg\n",
                "\"plain.txt\"     2\thijklmn\n",
                "\"gztext.txt.gz\"     1\tABCDEFG\n",
                "\"gztext.txt.gz\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_line_numbering_is_per_file_with_path_name() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-n", "--path-name", fixture_plain!(), fixture_gz!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"fixtures/plain.txt\"     1\tabcdefg\n",
                "\"fixtures/plain.txt\"     2\thijklmn\n",
                "\"fixtures/gztext.txt.gz\"     1\tABCDEFG\n",
                "\"fixtures/gztext.txt.gz\"     2\tHIJKLMN\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_repeated_files() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [fixture_plain!(), fixture_gz!(), fixture_plain!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "abcdefg\nhijklmn\nABCDEFG\nHIJKLMN\nabcdefg\nhijklmn\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_line_number_with_repeated_files() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-n", fixture_plain!(), fixture_gz!(), fixture_plain!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "     1\tabcdefg\n     2\thijklmn\n     3\tABCDEFG\n     4\tHIJKLMN\n     5\tabcdefg\n     6\thijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mixed_stdin_files() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--", "-", fixture_gz!(), "-"],
            b"stdin line\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdin line\nABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mixed_files_and_stdin_with_line_numbers() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-n", "--", fixture_gz!(), "-", fixture_plain!()],
            b"stdin line 1\nstdin line 2\n",
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "     1\tABCDEFG\n",
                "     2\tHIJKLMN\n",
                "     3\tstdin line 1\n",
                "     4\tstdin line 2\n",
                "     5\tabcdefg\n",
                "     6\thijklmn\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_large_gz_file() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_text10k!()]);
        assert_eq!(oup.stderr, "");
        // Just check that the output is large, not the exact content
        assert!(oup.stdout.len() > 10000);
        assert!(oup.status.success());
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_xz() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_xz!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_zstd() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_zstd!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "lz4")]
mod test_3_file_lz4_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_lz4() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_lz4!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
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
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_bzip2() {
        let oup = exec_target(TARGET_EXE_PATH, [fixture_bzip2!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
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

mod test_4_complex_e {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_stdin_first_then_file() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--", "-", fixture_plain!()],
            b"stdin line\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdin line\nabcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_line_number_with_empty_file() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-n", fixture_empty!(), fixture_plain!()],
            b"" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!("     1\tabcdefg\n", "     2\thijklmn\n",)
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_line_numbering_and_filename_with_stdin() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-n", "-f", "--", "-", fixture_plain!()],
            b"stdin line\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "\"\"     1\tstdin line\n",
                "\"plain.txt\"     1\tabcdefg\n",
                "\"plain.txt\"     2\thijklmn\n"
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_stdin_with_files() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--", "-", fixture_plain!()], b"" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8_mixed() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [fixture_plain!(), fixture_invalid_utf8!(), fixture_plain!()],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            format!(
                "abcdefg\nhijklmn\n{}abcdefg\nhijklmn\n",
                invalid_utf8_result!()
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_stdin() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--", "-", "-", fixture_plain!(), "-"],
            b"stdin\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdin\nabcdefg\nhijklmn\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mix_of_existing_and_non_existing_files() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [fixture_plain!(), "non-existent-file", fixture_plain!()],
        );
        #[cfg(not(windows))]
        assert!(oup.stderr.contains("No such file or directory"));
        #[cfg(windows)]
        assert!(oup
            .stderr
            .contains("The system cannot find the file specified"));
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        assert!(!oup.status.success());
    }
}

#[cfg(feature = "xz2")]
#[cfg(feature = "zstd")]
#[cfg(feature = "lz4")]
#[cfg(feature = "bzip2")]
mod test_4_complex_more_e {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_all_compression_formats() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
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
    #[test]
    fn test_mixed_stdin_compressed_uncompressed() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "--",
                "-",
                fixture_plain!(),
                fixture_gz!(),
                fixture_xz!(),
                fixture_zstd!(),
                fixture_lz4!(),
                fixture_bzip2!(),
            ],
            b"stdin line\n",
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mixed_with_line_numbers() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
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
            b"stdin line\n",
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_fnm_num() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
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
            b"stdin line\n",
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_plain_gz_xz_zst_lz4_bzip2_pnm_num() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
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
            b"stdin line\n",
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert!(oup.status.success());
    }
}

mod test_5_binary_mode_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_binary_mode_with_plain() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_plain!()]);
        assert_eq!(oup.stderr, "");
        #[cfg(not(windows))]
        assert_eq!(oup.stdout, "abcdefg\nhijklmn\n");
        #[cfg(windows)]
        assert_eq!(oup.stdout, "abcdefg\r\nhijklmn\r\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_binary_mode_with_invalid_utf8() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_invalid_utf8!()]);
        assert_eq!(oup.stderr, "");
        let expected_stdout = std::fs::read(fixture_invalid_utf8!()).unwrap();
        assert_ne!(oup.stdout.as_bytes(), expected_stdout);
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "flate2")]
    #[test]
    fn test_binary_mode_with_gz() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_gz!()]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_binary_mode_with_xz() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_xz!()]);
        assert_eq!(oup.stderr, "");
        let expected = std::fs::read("fixtures/xztext.txt.xz").unwrap();
        let mut decoder = xz2::read::XzDecoder::new(expected.as_slice());
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(oup.stdout.as_bytes(), expected_stdout);
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "zstd")]
    #[test]
    fn test_binary_mode_with_zstd() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_zstd!()]);
        assert_eq!(oup.stderr, "");
        let expected = std::fs::read("fixtures/zstext.txt.zst").unwrap();
        let mut decoder = zstd::stream::read::Decoder::new(expected.as_slice()).unwrap();
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(oup.stdout.as_bytes(), expected_stdout);
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "lz4")]
    #[test]
    fn test_binary_mode_with_lz4() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_lz4!()]);
        assert_eq!(oup.stderr, "");
        let expected = std::fs::read("fixtures/lz4text.txt.lz4").unwrap();
        let mut decoder = lz4::Decoder::new(expected.as_slice()).unwrap();
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(oup.stdout.as_bytes(), expected_stdout);
        assert!(oup.status.success());
    }
    //
    #[cfg(feature = "bzip2")]
    #[test]
    fn test_binary_mode_with_bzip2() {
        let oup = exec_target(TARGET_EXE_PATH, ["-b", fixture_bzip2!()]);
        assert_eq!(oup.stderr, "");
        let expected = std::fs::read("fixtures/bzip2text.txt.bz2").unwrap();
        let mut decoder = bzip2::read::BzDecoder::new(expected.as_slice());
        let mut expected_stdout = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut expected_stdout).unwrap();
        assert_eq!(oup.stdout.as_bytes(), expected_stdout);
        assert!(oup.status.success());
    }
}

#[cfg(feature = "flate2")]
mod test_9_broken_pipe_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[cfg(feature = "flate2")]
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
