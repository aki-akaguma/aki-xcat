use crate::conf::CmdOptConf;
use crate::util::adapt_input;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;

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
        std::io::copy(reader, &mut sioe.pout().lock())?;
        Ok(())
    })?;
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
