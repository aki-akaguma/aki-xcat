use crate::conf::CmdOptConf;
use crate::util::adapt_input;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::io::BufRead;
use std::io::BufReader;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    //
    let r = run_0(sioe, &conf.arg_params);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn run_0(sioe: &RunnelIoe, files: &[String]) -> anyhow::Result<()> {
    adapt_input(sioe, files, |reader| -> anyhow::Result<()> {
        //std::io::copy(reader, &mut sioe.pout().lock())?;
        //
        // The following code is needed to check UTF8.
        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            let line_s = line?;
            let line_ss = line_s.as_str();
            let _line_len: usize = line_ss.len();
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
        Ok(())
    })?;
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
