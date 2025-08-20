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
        &conf.arg_params,
        //|sioe: &mut RunnelIoe, reader: Option<&mut dyn BufRead>, path_s: &str, line_num: usize| {
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

fn process_input<'a>(
    sioe: &'a RunnelIoe,
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

fn process_binary<'a>(sioe: &'a RunnelIoe, reader: Option<Box<dyn BufRead>>) -> anyhow::Result<()> {
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

fn process_text_simple<'a>(
    sioe: &'a RunnelIoe,
    reader: Option<Box<dyn BufRead>>,
) -> anyhow::Result<()> {
    let is_string_pipe_in = sioe.pg_in().is_line_pipe();
    if is_string_pipe_in {
        process_text_simple_string_pipe_in(sioe)
    } else {
        process_text_simple_byte_in(sioe, reader)
    }
}

fn process_text_simple_string_pipe_in(sioe: &RunnelIoe) -> anyhow::Result<()> {
    //let is_string_pipe_out = sioe.pg_out().is_line_pipe();
    for line in sioe.pg_in().lines() {
        let line = line?;
        /*
        if is_string_pipe_out {
            sioe.pg_out().write_line(line)?;
        } else {
            #[cfg(not(windows))]
            sioe.pg_out().lock().write_fmt(format_args!("{line}"))?;
            //
            #[cfg(windows)]
            {
                let line_ss = line;
                let ss = line_ss.as_bytes();
                let len = ss.len();
                if len >= 2 && ss[len - 2] == b'\r' && ss[len - 1] == b'\n' {
                    let ss = &ss[..(len - 2)];
                    let ssss = String::from_utf8_lossy(ss);
                    sioe.pg_out().lock().write_fmt(format_args!("{ssss}\n"))?;
                } else {
                    sioe.pg_out().lock().write_fmt(format_args!("{line_ss}"))?;
                }
            }
        }
        */
        sioe.pg_out().write_line(line)?;
    }
    Ok(())
}

fn process_text_simple_byte_in(
    sioe: &RunnelIoe,
    reader: Option<Box<dyn BufRead>>,
) -> anyhow::Result<()> {
    let is_string_pipe_out = sioe.pg_out().is_line_pipe();
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
        //
        if is_string_pipe_out {
            sioe.pg_out().write_line(line_ss.to_string())?;
        } else {
            #[cfg(not(windows))]
            sioe.pg_out()
                .lock()
                .write_fmt(format_args!("{line_ss}\n"))?;
            //
            #[cfg(windows)]
            {
                let line_ss = line_ss.to_string();
                let ss = line_ss.as_bytes();
                let len = ss.len();
                if len >= 2 && ss[len - 2] == b'\r' && ss[len - 1] == b'\n' {
                    let ss = &ss[..(len - 2)];
                    let ssss = String::from_utf8_lossy(ss);
                    sioe.pg_out().lock().write_fmt(format_args!("{ssss}\n"))?;
                } else {
                    sioe.pg_out().lock().write_fmt(format_args!("{line_ss}"))?;
                }
            }
        }
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
    let mut all_line_num = line_num;
    let mut curr_line_num: usize = 0;
    let file_nm = if conf.flg_file_name {
        Path::new(path_s)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default()
    } else {
        "".to_string()
    };
    for line in reader.lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //
        let fmt_left = if conf.flg_path_name {
            let prefix = format!("\"{path_s}\"");
            if conf.flg_number {
                curr_line_num += 1;
                format!("{prefix}{curr_line_num:>6}")
            } else {
                prefix
            }
        } else if conf.flg_file_name {
            let prefix = format!("\"{file_nm}\"");
            if conf.flg_number {
                curr_line_num += 1;
                format!("{prefix}{curr_line_num:>6}")
            } else {
                prefix
            }
        } else if conf.flg_number {
            all_line_num += 1;
            format!("{all_line_num:>6}")
        } else {
            String::new()
        };
        if is_string_pipe_out {
            sioe.pg_out().write_line(format!("{fmt_left}\t{line_ss}"))?;
        } else {
            sioe.pg_out()
                .lock()
                .write_fmt(format_args!("{fmt_left}\t{line_ss}\n"))?;
        }
    }
    Ok(all_line_num)
}
