use std::fs::File;
use std::io::{Read, Result as IoResult, Seek, SeekFrom};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    Plain,
    Gzip,
    Xz,
    Zstd,
    Lz4,
    Bzip2,
    #[default]
    Other,
}

// --- Magic Numbers Reference ---
// Gzip:  1F 8B (2 bytes)
// Xz:    FD 37 7A 58 (First 4 bytes of 6-byte header: FD 37 7A 58 5A 00)
// Zstd:  28 B5 2F FD (4 bytes)
// Lz4:   04 22 4D 18 (4 bytes)
// Bzip2: 42 5A 68 ("BZh", 3 bytes)
// -------------------------------

const MAGIC_GZIP: [u8; 2] = [0x1f, 0x8b];
const MAGIC_XZ: [u8; 4] = [0xfd, 0x37, 0x7a, 0x58];
const MAGIC_ZSTD: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];
const MAGIC_LZ4: [u8; 4] = [0x04, 0x22, 0x4d, 0x18];
const MAGIC_BZIP2: [u8; 3] = [0x42, 0x5a, 0x68];

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
    } else if magic_bytes.starts_with(&MAGIC_GZIP) {
        FileType::Gzip
    } else if magic_bytes.starts_with(&MAGIC_XZ) {
        FileType::Xz
    } else if magic_bytes.starts_with(&MAGIC_ZSTD) {
        FileType::Zstd
    } else if magic_bytes.starts_with(&MAGIC_LZ4) {
        FileType::Lz4
    } else if magic_bytes.starts_with(&MAGIC_BZIP2) {
        FileType::Bzip2
    } else {
        FileType::Plain
    })
}
/*
 * reference:
 *   https://en.wikipedia.org/wiki/List_of_file_signatures
*/
