# aki-xcat

*aki-xcat* is the program that concate files that are plain, gzip ... etc.

## Features

*aki-xcat*  concate files that are plain, gzip ... etc.

* example

command:
```
`aki-xcat` -H
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

```
aki-xcat text-file-1 gzip-file-2.gz gzip-file-3.gz | less
```
