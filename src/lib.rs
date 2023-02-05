/*!
concatenate files that are plain, gzip, xz and zstd.

# Features

- concatenate files that are plain, gzip ... etc.
- input files are decompressed by auto.
- minimum support rustc 1.58.1 (db9d1b20b 2022-01-20)

# Command help

```text
aki-xcat --help
```

```text
Usage:
  aki-xcat [options] [<file>...]

this is like a cat, zcat, xzcat, zstdcat and lz4cat.
with no <file> or when <file> is -, read standard input.
automatic discovery file type: plain, gz, xz, zst and lz4.

Options:
  -n, --number          output line number for each lines
  -f, --file-name       output file name for each lines
      --path-name       output path name for each lines
  -p, --pipe-in <num>   read from pipe <num> [unimplemented]

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Argument:
  <file>         utf-8 encoded text file. A compressed file of it by gzip, xz, zstd, lz4.

Examples:
  You can simple use. Just arrange the files.
    aki-xcat file1 file2.gz file3.xz file4.zst file5.lz4
```

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-xcat
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

The input file used in this example looks like this:

```text
cat fixtures/plain.txt
```
result output :
```text
abcdefg
hijklmn
```

```text
zcat fixtures/gztext.txt.gz
```
result output :
```text
ABCDEFG
HIJKLMN
```

## Example 1: simple concatenate

concatenate plain text file and gzip text file.
```text
aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```text
abcdefg
hijklmn
ABCDEFG
HIJKLMN
```

## Example 2: multi file formats

concatenate plain text file, gzip text file, xz text file, zstd text file and lz4 text file.
```text
aki-xcat fixtures/plain.txt fixtures/gztext.txt.gz fixtures/xztext.txt.xz  fixtures/zstext.txt.zst fixtures/lz4text.txt.lz4
```

## Example 3: output all line number

concatenate plain text file and gzip text file.
```text
aki-xcat -n fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```text
     1  abcdefg
     2  hijklmn
     3  ABCDEFG
     4  HIJKLMN
```

## Example 4: file name and line number

concatenate plain text file and gzip text file.
```text
aki-xcat -n -f fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```text
"plain.txt"     1   abcdefg
"plain.txt"     2   hijklmn
"gztext.txt.gz"     1   ABCDEFG
"gztext.txt.gz"     2   HIJKLMN
```

## Example 5: path name and line number

concatenate plain text file and gzip text file.
```text
aki-xcat -n --path-name fixtures/plain.txt fixtures/gztext.txt.gz
```
result output :
```text
"fixtures/plain.txt"     1  abcdefg
"fixtures/plain.txt"     2  hijklmn
"fixtures/gztext.txt.gz"     1  ABCDEFG
"fixtures/gztext.txt.gz"     2  HIJKLMN
```

### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

*/
#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute xcat
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "xcat"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_xcat::execute(&RunnelIoeBuilder::new().build(),
///     "xcat", &["-n", "file1", "file2.gz", "file3.xz", "file4.zst"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{err}\n"));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
