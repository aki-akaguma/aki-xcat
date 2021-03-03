#[cfg(feature = "flate2")]
use flate2::read::GzDecoder;

#[cfg(feature = "libflate")]
use libflate::gzip::Decoder;

#[cfg(feature = "inflate")]
use inflate::DeflateDecoder;

#[cfg(feature = "xz2")]
use xz2::read::XzDecoder;

#[cfg(feature = "zstd")]
use zstd::Decoder as ZstdDecoder;

#[cfg(feature = "lz4")]
use lz4::Decoder as Lz4Decoder;

use anyhow::Context;
use runnel::RunnelIoe;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub fn adapt_input<F>(sioe: &RunnelIoe, files: &[String], mut f: F) -> anyhow::Result<()>
where
    F: FnMut(&mut dyn BufRead) -> anyhow::Result<()>,
{
    if files.is_empty() {
        return f(&mut sioe.pin().lock());
    } else {
        for path_s in files {
            if path_s == "-" {
                return f(&mut sioe.pin().lock());
            }
            do_cat_proc_file(path_s, &mut f)?;
        }
    }
    Ok(())
}

fn do_cat_proc_file<F>(path_s: &str, f: &mut F) -> anyhow::Result<()>
where
    F: FnMut(&mut dyn BufRead) -> anyhow::Result<()>,
{
    let mut file = File::open(path_s).with_context(|| format!("can not open file: {}", path_s))?;
    //
    let mut buffer = [0; 4];
    match file.read(&mut buffer[..]) {
        Ok(sz) if sz >= 4 => {
            if buffer[0] == 0x1f && buffer[1] == 0x8b {
                // gzip file, at signature found
                file.seek(SeekFrom::Start(0))?;
                //
                #[cfg(feature = "flate2")]
                let gzd = GzDecoder::new(file);
                #[cfg(feature = "libflate")]
                let gzd = Decoder::new(file)?;
                #[cfg(feature = "inflate")]
                let gzd = DeflateDecoder::from_zlib(file);
                //
                let mut buf_reader = BufReader::new(gzd);
                let reader: &mut dyn BufRead = &mut buf_reader;
                return f(reader);
            } else if buffer[0] == 0xfd
                && buffer[1] == 0x37
                && buffer[2] == 0x7A
                && buffer[3] == 0x58
            {
                #[cfg(feature = "xz2")]
                {
                    // xz file, at signature found
                    file.seek(SeekFrom::Start(0))?;
                    //
                    let xzd = XzDecoder::new(file);
                    //
                    let mut buf_reader = BufReader::new(xzd);
                    let reader: &mut dyn BufRead = &mut buf_reader;
                    return f(reader);
                }
            } else if buffer[0] == 0x28
                && buffer[1] == 0xb5
                && buffer[2] == 0x2f
                && buffer[3] == 0xfd
            {
                #[cfg(feature = "zstd")]
                {
                    // zstd file, at signature found
                    file.seek(SeekFrom::Start(0))?;
                    //
                    let zsd = ZstdDecoder::new(file)?;
                    //
                    let mut buf_reader = BufReader::new(zsd);
                    let reader: &mut dyn BufRead = &mut buf_reader;
                    return f(reader);
                }
            } else if buffer[0] == 0x04
                && buffer[1] == 0x22
                && buffer[2] == 0x4D
                && buffer[3] == 0x18
            {
                #[cfg(feature = "lz4")]
                {
                    // lz4 file, at signature found
                    file.seek(SeekFrom::Start(0))?;
                    //
                    let lz4 = Lz4Decoder::new(file)?;
                    //
                    let mut buf_reader = BufReader::new(lz4);
                    let reader: &mut dyn BufRead = &mut buf_reader;
                    return f(reader);
                }
            } else if buffer[0] == 0x78
                && (buffer[1] == 0x01
                    || buffer[1] == 0x5e
                    || buffer[1] == 0x9c
                    || buffer[1] == 0xda
                    || buffer[1] == 0x20
                    || buffer[1] == 0x7d
                    || buffer[1] == 0xbb
                    || buffer[1] == 0xf9)
            {
                // zlib file, at signature found
                eprintln!("zlib file signature found.");
                unimplemented!();
            }
        }
        _ => {}
    };
    // plain file
    file.seek(SeekFrom::Start(0))?;
    let mut buf_reader = BufReader::new(file);
    let reader: &mut dyn BufRead = &mut buf_reader;
    f(reader).with_context(|| format!("Failed to read from '{}'", path_s))
}
/*
 * reference:
 *      https://en.wikipedia.org/wiki/List_of_file_signatures
*/
