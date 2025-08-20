use std::fs::File;
use std::io::{Read, Result as IoResult, Seek, SeekFrom};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    Plain,
    Gzip,
    Xz,
    Zstd,
    Lz4,
    Bzip2,
    Other,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Other
    }
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
            [0x1f, 0x8b, _, _] => FileType::Gzip,
            [0xfd, 0x37, 0x7a, 0x58] => FileType::Xz,
            [0x28, 0xb5, 0x2f, 0xfd] => FileType::Zstd,
            [0x04, 0x22, 0x4d, 0x18] => FileType::Lz4,
            [0x42, 0x5a, 0x68, _] => FileType::Bzip2,
            _ => FileType::Plain,
        }
    })
}
/*
 * reference:
 *   https://en.wikipedia.org/wiki/List_of_file_signatures
*/
