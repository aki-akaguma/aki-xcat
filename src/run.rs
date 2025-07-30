use crate::conf::CmdOptConf;
use crate::util::adapt_input;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::io::BufRead;
use std::path::Path;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    //
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
        |reader, path_s, line_num| -> anyhow::Result<usize> {
            let mut all_line_num = line_num;
            //
            if conf.flg_bin {
                // binary mode
                loop {
                    let buf = match reader.fill_buf() {
                        Ok(buf) => buf,
                        Err(err) => {
                            return Err(anyhow!("{}", err));
                        }
                    };
                    if buf.is_empty() {
                        break;
                    }
                    sioe.pout().lock().write_all(buf)?;
                    let len = buf.len();
                    reader.consume(len);
                }
            } else if !conf.flg_number && !conf.flg_file_name && !conf.flg_path_name {
                // text mode
                let mut buf: Vec<u8> = vec![];
                loop {
                    buf.clear();
                    match reader.read_until(b'\n', &mut buf) {
                        Ok(_sz) => (),
                        Err(err) => {
                            return Err(anyhow!("{}", err));
                        }
                    };
                    if buf.is_empty() {
                        break;
                    }
                    let line_ss = String::from_utf8_lossy(&buf);
                    #[cfg(not(windows))]
                    sioe.pout().lock().write_fmt(format_args!("{line_ss}"))?;
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
                /*
                // The following code is needed to check UTF8.
                for line in reader.lines() {
                    let line_s = line?;
                    let line_ss = line_s.as_str();
                    //let _line_len: usize = line_ss.len();
                    sioe.pout().lock().write_fmt(format_args!("{line_ss}\n"))?;
                }
                */
            } else {
                let mut curr_line_num: usize = 0;
                let file_nm = if conf.flg_file_name {
                    if let Some(os_s) = Path::new(path_s).file_name() {
                        os_s.to_string_lossy().to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                for line in reader.lines() {
                    let line_s = line?;
                    let line_ss = line_s.as_str();
                    //let _line_len: usize = line_ss.len();
                    //
                    let fmt_left = if conf.flg_path_name || conf.flg_file_name {
                        if conf.flg_path_name {
                            if conf.flg_number {
                                curr_line_num += 1;
                                format!("\"{path_s}\"{curr_line_num:>6}")
                            } else {
                                format!("\"{path_s}\"")
                            }
                        } else if conf.flg_file_name {
                            if conf.flg_number {
                                curr_line_num += 1;
                                format!("\"{file_nm}\"{curr_line_num:>6}")
                            } else {
                                format!("\"{file_nm}\"")
                            }
                        } else {
                            String::new()
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
            }
            Ok(all_line_num)
        },
    )?;
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
