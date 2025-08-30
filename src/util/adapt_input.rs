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

pub fn adapt_input<F>(
    sioe: &RunnelIoe,
    base_dir: String,
    files: &[String],
    mut f: F,
) -> anyhow::Result<()>
where
    F: FnMut(&RunnelIoe, Option<Box<dyn BufRead>>, &str, usize) -> anyhow::Result<usize>,
{
    let mut line_num: usize = 0;
    if files.is_empty() {
        f(sioe, None, "", line_num)?;
    } else {
        for path_s in files {
            if path_s == "-" {
                line_num = f(sioe, None, "", line_num)?;
            } else {
                line_num = if base_dir.is_empty() {
                    cat_process_file(sioe, path_s, line_num, &mut f)?
                } else {
                    let s = format!("{base_dir}/{path_s}");
                    cat_process_file(sioe, &s, line_num, &mut f)?
                };
            }
        }
    }
    Ok(())
}

fn cat_process_file<F>(
    sioe: &RunnelIoe,
    path_s: &str,
    line_num: usize,
    f: &mut F,
) -> anyhow::Result<usize>
where
    F: FnMut(&RunnelIoe, Option<Box<dyn BufRead>>, &str, usize) -> anyhow::Result<usize>,
{
    let mut file = File::open(path_s).with_context(|| format!("Cannot open file: {path_s}"))?;
    let file_type = detect_file_type(&mut file)?;

    let reader: Box<dyn BufRead> = match file_type {
        #[cfg(feature = "flate2")]
        FileType::Gzip => Box::new(BufReader::new(GzDecoder::new(file))),
        #[cfg(feature = "xz2")]
        FileType::Xz => Box::new(BufReader::new(XzDecoder::new(file))),
        #[cfg(feature = "zstd")]
        FileType::Zstd => Box::new(BufReader::new(ZstdDecoder::new(file)?)),
        #[cfg(feature = "lz4")]
        FileType::Lz4 => Box::new(BufReader::new(Lz4Decoder::new(file)?)),
        #[cfg(feature = "bzip2")]
        FileType::Bzip2 => Box::new(BufReader::new(BzDecoder::new(file))),
        FileType::Plain => Box::new(BufReader::new(file)),
        _ => return Err(anyhow!("Not support file type: {:?}", file_type)),
    };

    f(sioe, Some(reader), path_s, line_num)
        .with_context(|| format!("Failed to read from '{path_s}'"))
}
