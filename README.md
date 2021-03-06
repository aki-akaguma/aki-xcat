# aki-xcat

*aki-xcat* is the program that concatenate files that are plain, gzip ... etc.

## Features

*aki-xcat*  concatenate files that are plain, gzip ... etc.

* command help

```text
aki-xcat --help
```

```
//! Usage:
//!   aki-xcat [options] [<file>...]
//! 
//! this is like a cat, zcat, xzcat, zstdcat and lz4cat.
//! with no <file> or when <file> is -, read standard input.
//! automatic discovery file type: plain, gz, xz, zst and lz4.
//! 
//! Options:
//!   -n, --number          output line number for each lines
//!   -f, --file-name       output file name for each lines
//!       --path-name       output path name for each lines
//!   -p, --pipe-in <num>   read from pipe <num> [unimplemented]
//! 
//!   -H, --help        display this help and exit
//!   -V, --version     display version information and exit
//! 
//! Argument:
//!   <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd, lz4.
//! 
//! Examples:
//!   You can simple use. Just arrange the files.
//!     aki-xcat file1 file2.gz file3.xz file4.zst file5.lz4
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-xcat
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

The input file used in this example looks like this:

```
cat fixtures/plain.txt
```
result output :
```
abcdefg
hijklmn
```

```
zcat fixtures/gztext.txt.gz
```
result output :
```
ABCDEFG
HIJKLMN
```

### Example 1: simple concatenate

concatenate plain text file and gzip text file.
```
aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```
abcdefg
hijklmn
ABCDEFG
HIJKLMN
```

### Example 2: multi file formats

concatenate plain text file, gzip text file, xz text file, zstd text file and lz4 text file.
```
aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz fixtures/xztext.txt.xz  fixtures/zstext.txt.zst fixtures/lz4text.txt.lz4
```

### Example 3: output all line number

concatenate plain text file and gzip text file.
```
aki-xcat -n fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```
     1  abcdefg
     2  hijklmn
     3  ABCDEFG
     4  HIJKLMN
```

### Example 4: file name and line number

concatenate plain text file and gzip text file.
```
aki-xcat -n -f fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```
"plain.txt"     1   abcdefg
"plain.txt"     2   hijklmn
"gztext.txt.gz"     1   ABCDEFG
"gztext.txt.gz"     2   HIJKLMN
```

### Example 5: path name and line number

concatenate plain text file and gzip text file.
```
aki-xcat -n --path-name fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```
"fixtures/plain.txt"     1  abcdefg
"fixtures/plain.txt"     2  hijklmn
"fixtures/gztext.txt.gz"     1  ABCDEFG
"fixtures/gztext.txt.gz"     2  HIJKLMN
```

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-xcat/blob/main/CHANGELOG.md)
