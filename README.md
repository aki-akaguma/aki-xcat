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

cat and zcat by rust lang.
with no <file> or when <file> is -, read standard input.

Options:
  -H, --help     display this help and exit
  -V, --version  display version information and exit
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

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
