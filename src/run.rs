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
        |reader: &mut dyn BufRead, path_s: &str, line_num: usize| {
            process_input(sioe, conf, reader, path_s, line_num)
        },
    )?;
    sioe.pout().lock().flush()?;
    Ok(())
}

fn process_input(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    reader: &mut dyn BufRead,
    path_s: &str,
    line_num: usize,
) -> anyhow::Result<usize> {
    if conf.flg_bin {
        process_binary(reader, sioe)?;
        Ok(line_num)
    } else if !conf.flg_number && !conf.flg_file_name && !conf.flg_path_name {
        process_text_simple(reader, sioe)?;
        Ok(line_num)
    } else {
        process_text_decorated(reader, sioe, conf, path_s, line_num)
    }
}

fn process_binary(reader: &mut dyn BufRead, sioe: &RunnelIoe) -> anyhow::Result<()> {
    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        sioe.pout().lock().write_all(buf)?;
        let len = buf.len();
        reader.consume(len);
    }
    Ok(())
}

fn process_text_simple(reader: &mut dyn BufRead, sioe: &RunnelIoe) -> anyhow::Result<()> {
    let mut buf: Vec<u8> = vec![];
    loop {
        buf.clear();
        reader.read_until(b'\n', &mut buf)?;
        if buf.is_empty() {
            break;
        }
        let line_ss = String::from_utf8_lossy(&buf);
        //
        #[cfg(not(windows))]
        sioe.pout().lock().write_fmt(format_args!("{line_ss}"))?;
        //
        #[cfg(windows)]
        {
            let line_ss = line_ss.to_string();
            let ss = line_ss.as_bytes();
            let len = ss.len();
            if len >= 2 && ss[len - 2] == b'\r' && ss[len - 1] == b'\n' {
                let ss = &ss[..(len - 2)];
                let ssss = String::from_utf8_lossy(ss);
                sioe.pout().lock().write_fmt(format_args!("{ssss}\n"))?;
            } else {
                sioe.pout().lock().write_fmt(format_args!("{line_ss}"))?;
            }
        }
    }
    Ok(())
}

fn process_text_decorated(
    reader: &mut dyn BufRead,
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    path_s: &str,
    line_num: usize,
) -> anyhow::Result<usize> {
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
        sioe.pout()
            .lock()
            .write_fmt(format_args!("{fmt_left}\t{line_ss}\n"))?;
    }
    Ok(all_line_num)
}
