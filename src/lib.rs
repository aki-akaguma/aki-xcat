//!
//! concatenate files that are plain, gzip, xz and zstd.
//!
//! ```text
//! Usage:
//!   aki-xcat [options] [<file>...]
//!
//! this is a like cat, zcat, xzcat and zstdcat.
//! with no <file> or when <file> is -, read standard input.
//! automatic discovery file type: plain, gz, xz and zst.
//!
//! Options:
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//! 
//! Argument:
//!   <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd.
//! 
//! Examples:
//!   You can simple use. Just arrange the files.
//!     aki-xcat file1 file2.gz file3.xz file4.zst
//! ```
//!
//! # Examples
//!
//! ### Command line example 1
//!
//! concatenate plain text file.
//! ```text
//! cat fixtures/plain.txt
//! ```
//! result output :
//! ```text
//! abcdefg
//! hijklmn
//! ```
//!
//! concatenate gzip text file.
//! ```text
//! zcat fixtures/gztext.txt.gz
//! ```
//! result output :
//! ```text
//! ABCDEFG
//! HIJKLMN
//! ```
//!
//! concatenate plain text file and gzip text file.
//! ```text
//! aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz
//! ```
//! result output :
//! ```text
//! abcdefg
//! hijklmn
//! ABCDEFG
//! HIJKLMN
//! ```
//!
//! ### Command line example 2
//!
//! concatenate plain text file, gzip text file, xz text file and zstd text file.
//! ```text
//! aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz fixtures/xztext.txt.xz  fixtures/zstext.txt.zst
//! ```
//!
//! ### Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
//!

#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute xcat
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "xcat"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_xcat::execute(&RunnelIoeBuilder::new().build(),
///     "xcat", &["file1", "file2.gz", "file3.xz", "file4.zst"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
