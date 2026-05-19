use crate::conf::CmdOptConf;
use crate::util::adapt_input;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};
use std::path::Path;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let r = run_0(sioe, conf);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    adapt_input(
        sioe,
        conf.base_dir(),
        &conf.arg_params,
        |sioe, reader, path_s, line_num| process_input(sioe, conf, reader, path_s, line_num),
    )?;
    let is_string_pipe_out = sioe.pg_out().is_line_pipe();
    if is_string_pipe_out {
        sioe.pg_out().flush_line()?;
    } else {
        sioe.pg_out().lock().flush()?;
    }
    Ok(())
}

fn process_input(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    reader: Option<Box<dyn BufRead>>,
    path_s: &str,
    line_num: usize,
) -> anyhow::Result<usize> {
    if conf.flg_bin {
        process_binary(sioe, reader)?;
        Ok(line_num)
    } else if !conf.flg_number && !conf.flg_file_name && !conf.flg_path_name {
        process_text_simple(sioe, reader)?;
        Ok(line_num)
    } else {
        process_text_decorated(sioe, reader, conf, path_s, line_num)
    }
}

fn process_binary(sioe: &RunnelIoe, reader: Option<Box<dyn BufRead>>) -> anyhow::Result<()> {
    let mut reader = match reader {
        Some(rd) => rd,
        None => sioe.pg_in().lock_bufread(),
    };
    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        sioe.pg_out().lock().write_all(buf)?;
        let len = buf.len();
        reader.consume(len);
    }
    Ok(())
}

fn process_text_simple(sioe: &RunnelIoe, reader: Option<Box<dyn BufRead>>) -> anyhow::Result<()> {
    let is_string_pipe_in = sioe.pg_in().is_line_pipe();
    if is_string_pipe_in {
        process_text_simple_string_pipe_in(sioe)
    } else {
        process_text_simple_byte_in(sioe, reader)
    }
}

fn process_text_simple_string_pipe_in(sioe: &RunnelIoe) -> anyhow::Result<()> {
    for line in sioe.pg_in().lines() {
        let line = line?;
        sioe.pg_out().write_line(line)?;
    }
    Ok(())
}

fn process_text_simple_byte_in(
    sioe: &RunnelIoe,
    reader: Option<Box<dyn BufRead>>,
) -> anyhow::Result<()> {
    let mut reader = match reader {
        Some(rd) => rd,
        None => sioe.pg_in().lock_bufread(),
    };
    let mut buf: Vec<u8> = vec![];
    loop {
        buf.clear();
        reader.read_until(b'\n', &mut buf)?;
        if buf.is_empty() {
            break;
        }
        let buf_s = if buf[buf.len() - 1] == b'\n' {
            if buf.len() >= 2 && buf[buf.len() - 2] == b'\r' {
                &buf[..(buf.len() - 2)]
            } else {
                &buf[..(buf.len() - 1)]
            }
        } else {
            &buf[..]
        };
        let line_ss = String::from_utf8_lossy(buf_s);
        sioe.pg_out().write_line(line_ss.to_string())?;
    }
    Ok(())
}

fn process_text_decorated(
    sioe: &RunnelIoe,
    reader: Option<Box<dyn BufRead>>,
    conf: &CmdOptConf,
    path_s: &str,
    line_num: usize,
) -> anyhow::Result<usize> {
    let is_string_pipe_out = sioe.pg_out().is_line_pipe();
    let reader = match reader {
        Some(rd) => rd,
        None => sioe.pg_in().lock_bufread(),
    };

    let mut decorator = TextDecorator::new(conf, path_s, line_num);

    for line in reader.lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let fmt_left = decorator.next_prefix();

        if is_string_pipe_out {
            sioe.pg_out().write_line(format!("{fmt_left}\t{line_ss}"))?;
        } else {
            sioe.pg_out()
                .lock()
                .write_fmt(format_args!("{fmt_left}\t{line_ss}\n"))?;
        }
    }
    Ok(decorator.all_line_num)
}

struct TextDecorator<'a> {
    conf: &'a CmdOptConf,
    path_s: &'a str,
    file_nm: String,
    pub all_line_num: usize,
    curr_line_num: usize,
}

impl<'a> TextDecorator<'a> {
    fn new(conf: &'a CmdOptConf, path_s: &'a str, all_line_num: usize) -> Self {
        let file_nm = if conf.flg_file_name {
            Path::new(path_s)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            String::new()
        };
        Self {
            conf,
            path_s,
            file_nm,
            all_line_num,
            curr_line_num: 0,
        }
    }

    fn next_prefix(&mut self) -> String {
        if self.conf.flg_path_name {
            let prefix = format!("\"{}\"", self.path_s);
            if self.conf.flg_number {
                self.curr_line_num += 1;
                format!("{prefix}{:>6}", self.curr_line_num)
            } else {
                prefix
            }
        } else if self.conf.flg_file_name {
            let prefix = format!("\"{}\"", self.file_nm);
            if self.conf.flg_number {
                self.curr_line_num += 1;
                format!("{prefix}{:>6}", self.curr_line_num)
            } else {
                prefix
            }
        } else if self.conf.flg_number {
            self.all_line_num += 1;
            format!("{:>6}", self.all_line_num)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator_numbering() {
        let conf = CmdOptConf {
            flg_number: true,
            ..Default::default()
        };

        let mut decorator = TextDecorator::new(&conf, "test.txt", 0);
        assert_eq!(decorator.next_prefix(), "     1");
        assert_eq!(decorator.next_prefix(), "     2");
    }

    #[test]
    fn test_decorator_filename_and_number() {
        let conf = CmdOptConf {
            flg_file_name: true,
            flg_number: true,
            ..Default::default()
        };

        let mut decorator = TextDecorator::new(&conf, "dir/test.txt", 0);
        assert_eq!(decorator.next_prefix(), "\"test.txt\"     1");
        assert_eq!(decorator.next_prefix(), "\"test.txt\"     2");
    }

    #[test]
    fn test_decorator_pathname_and_number() {
        let conf = CmdOptConf {
            flg_path_name: true,
            flg_number: true,
            ..Default::default()
        };

        let mut decorator = TextDecorator::new(&conf, "dir/test.txt", 10);
        assert_eq!(decorator.next_prefix(), "\"dir/test.txt\"     1");
    }

    #[test]
    fn test_decorator_only_filename() {
        let conf = CmdOptConf {
            flg_file_name: true,
            ..Default::default()
        };

        let mut decorator = TextDecorator::new(&conf, "dir/test.txt", 0);
        assert_eq!(decorator.next_prefix(), "\"test.txt\"");
    }

    #[test]
    fn test_decorator_continuous_numbering() {
        let conf = CmdOptConf {
            flg_number: true,
            ..Default::default()
        };

        let mut dec1 = TextDecorator::new(&conf, "f1.txt", 0);
        dec1.next_prefix();
        let last_num = dec1.all_line_num;

        let mut dec2 = TextDecorator::new(&conf, "f2.txt", last_num);
        assert_eq!(dec2.next_prefix(), "     2");
    }
}
