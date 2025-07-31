use std::fs::File;
use std::io::{Read, Result as IoResult, Seek, SeekFrom};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    Plain,
    #[cfg(feature = "flate2")]
    Gzip,
    #[cfg(feature = "xz2")]
    Xz,
    #[cfg(feature = "zstd")]
    Zstd,
    #[cfg(feature = "lz4")]
    Lz4,
    #[cfg(feature = "bzip2")]
    Bzip2,
}

pub fn detect_file_type(file: &mut File) -> IoResult<FileType> {
    let mut magic_bytes = [0; 4];
    let is_mini = if let Err(err) = file.read_exact(&mut magic_bytes) {
        if err.kind() != std::io::ErrorKind::UnexpectedEof {
            return Err(err);
        }
        true
    } else {
        false
    };
    file.seek(SeekFrom::Start(0))?; // Rewind after reading

    Ok(if is_mini {
        FileType::Plain
    } else {
        match magic_bytes {
            #[cfg(feature = "flate2")]
            [0x1f, 0x8b, _, _] => FileType::Gzip,
            #[cfg(feature = "xz2")]
            [0xfd, 0x37, 0x7a, 0x58] => FileType::Xz,
            #[cfg(feature = "zstd")]
            [0x28, 0xb5, 0x2f, 0xfd] => FileType::Zstd,
            #[cfg(feature = "lz4")]
            [0x04, 0x22, 0x4d, 0x18] => FileType::Lz4,
            #[cfg(feature = "bzip2")]
            [0x42, 0x5a, 0x68, _] => FileType::Bzip2,
            _ => FileType::Plain,
        }
    })
}
/*
 * reference:
 *   https://en.wikipedia.org/wiki/List_of_file_signatures
*/
