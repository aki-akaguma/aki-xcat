use crate::util::{detect_file_type, FileType};

#[cfg(feature = "flate2")]
use flate2::read::GzDecoder;

#[cfg(feature = "xz2")]
use xz2::read::XzDecoder;

#[cfg(feature = "zstd")]
use zstd::Decoder as ZstdDecoder;

#[cfg(feature = "lz4")]
use lz4::Decoder as Lz4Decoder;

#[cfg(feature = "bzip2")]
use bzip2::read::BzDecoder;

use anyhow::Context;
use runnel::RunnelIoe;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn adapt_input<F>(sioe: &RunnelIoe, files: &[String], mut f: F) -> anyhow::Result<()>
where
    F: FnMut(&mut dyn BufRead, &str, usize) -> anyhow::Result<usize>,
{
    let mut line_num: usize = 0;
    if files.is_empty() {
        f(&mut sioe.pin().lock(), "", line_num)?;
    } else {
        for path_s in files {
            if path_s == "-" {
                line_num = f(&mut sioe.pin().lock(), "", line_num)?;
            } else {
                line_num = cat_process_file(path_s, line_num, &mut f)?;
            }
        }
    }
    Ok(())
}

fn cat_process_file<F>(path_s: &str, line_num: usize, f: &mut F) -> anyhow::Result<usize>
where
    F: FnMut(&mut dyn BufRead, &str, usize) -> anyhow::Result<usize>,
{
    let mut file = File::open(path_s).with_context(|| format!("Cannot open file: {path_s}"))?;
    let file_type = detect_file_type(&mut file)?;

    let mut reader: Box<dyn BufRead> = match file_type {
        FileType::Gzip => Box::new(BufReader::new(GzDecoder::new(file))),
        FileType::Xz => Box::new(BufReader::new(XzDecoder::new(file))),
        FileType::Zstd => Box::new(BufReader::new(ZstdDecoder::new(file)?)),
        FileType::Lz4 => Box::new(BufReader::new(Lz4Decoder::new(file)?)),
        FileType::Bzip2 => Box::new(BufReader::new(BzDecoder::new(file))),
        FileType::Plain => Box::new(BufReader::new(file)),
    };

    f(&mut reader, path_s, line_num).with_context(|| format!("Failed to read from '{path_s}'"))
}
