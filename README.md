# aki-xcat

*aki-xcat* is the program that concatenate files that are plain, gzip ... etc.

## Features

*aki-xcat*  concatenate files that are plain, gzip ... etc.

* command help

```text
aki-xcat --help
```

```
Usage:
  aki-xcat [options] [<file>...]

this is like a cat, zcat, xzcat and zstdcat.
with no <file> or when <file> is -, read standard input.
automatic discovery file type: plain, gz, xz and zst.

Options:
  -p, --pipe-in <num>   read from pipe <num> [unimplemented]

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Argument:
  <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd.

Examples:
  You can simple use. Just arrange the files.
    aki-xcat file1 file2.gz file3.xz file4.zst
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

#### Command line example 1

concatenate plain text file.
```
cat fixtures/plain.txt
```
result output :
```
abcdefg
hijklmn
```

concatenate gzip text file.
```
zcat fixtures/gztext.txt.gz
```
result output :
```
ABCDEFG
HIJKLMN
```

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

#### Command line example 2

concatenate plain text file, gzip text file, xz text file and zstd text file.
```
aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz fixtures/xztext.txt.xz  fixtures/zstext.txt.zst
```

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
