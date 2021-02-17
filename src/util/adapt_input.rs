#[cfg(feature = "flate2")]
extern crate flate2;
#[cfg(feature = "flate2")]
use self::flate2::read::GzDecoder;

#[cfg(feature = "libflate")]
extern crate libflate;
#[cfg(feature = "libflate")]
use self::libflate::gzip::Decoder;

#[cfg(feature = "inflate")]
extern crate inflate;
#[cfg(feature = "inflate")]
use self::inflate::DeflateDecoder;

use runnel::StreamIoe;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn adapt_input<F>(sioe: &StreamIoe, files: &[String], mut f: F) -> anyhow::Result<()>
where
    F: FnMut(&mut dyn Read) -> anyhow::Result<()>,
{
    if files.is_empty() {
        return f(&mut sioe.pin.lock());
    } else {
        for path_s in files {
            if path_s == "-" {
                return f(&mut sioe.pin.lock());
            }
            do_cat_proc_file(path_s, &mut f)?;
        }
    }
    Ok(())
}

fn do_cat_proc_file<F>(path_s: &str, f: &mut F) -> anyhow::Result<()>
where
    F: FnMut(&mut dyn Read) -> anyhow::Result<()>,
{
    let mut file = File::open(path_s)?;
    //
    let mut buffer = [0; 2];
    match file.read(&mut buffer[..]) {
        Ok(sz) if sz >= 2 => {
            if buffer[0] == 0x1f && buffer[1] == 0x8b {
                // gzip file, at signature found
                file.seek(SeekFrom::Start(0))?;
                //
                #[cfg(feature = "flate2")]
                let gzd = GzDecoder::new(file);
                #[cfg(feature = "inflate")]
                let gzd = DeflateDecoder::from_zlib(file);
                #[cfg(feature = "libflate")]
                let gzd = Decoder::new(file)?;
                //
                let mut buf_reader = BufReader::new(gzd);
                let reader: &mut dyn Read = &mut buf_reader;
                return f(reader);
            } else if buffer[0] == 0x78 && buffer[1] == 0x9c {
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
    let reader: &mut dyn Read = &mut buf_reader;
    f(reader)
}
