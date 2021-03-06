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
            if !conf.flg_number && !conf.flg_file_name && !conf.flg_path_name {
                // The following code is needed to check UTF8.
                for line in reader.lines() {
                    let line_s = line?;
                    let line_ss = line_s.as_str();
                    //let _line_len: usize = line_ss.len();
                    sioe.pout()
                        .lock()
                        .write_fmt(format_args!("{}\n", line_ss))?;
                }
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
                                format!("\"{}\"{:>6}", path_s, curr_line_num)
                            } else {
                                format!("\"{}\"", path_s)
                            }
                        } else if conf.flg_file_name {
                            if conf.flg_number {
                                curr_line_num += 1;
                                format!("\"{}\"{:>6}", file_nm, curr_line_num)
                            } else {
                                format!("\"{}\"", file_nm)
                            }
                        } else {
                            String::new()
                        }
                    } else if conf.flg_number {
                        all_line_num += 1;
                        format!("{:>6}", all_line_num)
                    } else {
                        String::new()
                    };
                    sioe.pout()
                        .lock()
                        .write_fmt(format_args!("{}\t{}\n", fmt_left, line_ss))?;
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
