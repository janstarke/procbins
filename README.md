![Crates.io](https://img.shields.io/crates/v/procbins)

# procbins
compresses all process binaries into a zip file

This tool is intended to be used for forensic triage.

## Usage
```
Usage:
  procbins ZIPFILE

compresses all process binaries into a zip file

Positional arguments:
  zipfile               name of the destination zip file

Optional arguments:
  -h,--help             Show this help message and exit
```

## Features

* applies the original folder structure into the zip file, to prevent problems with duplicate file names
* writes log messages to `messages.log`, which will also be part of the zip file
* writes SHA1 hashes to `sha1_hashes.csv`, which will also be part of the zip file

## Download

| OS | Version | Hash |
|----|---------|-----|
|[Windows (x64)](https://github.com/teeshop/procbins/releases/download/0.4.0/procbins_0.4.0_x86_64-pc-windows-gnu.zip) | `0.4.0`  | `56eab09e1f46408f9b7d7dd357e602cb84335de89e68abf1b856ae089a27e740` |
|[Linux (x64)](https://github.com/teeshop/procbins/releases/download/0.4.0/procbins_0.4.0_x86_64-unknown-linux-musl.zip) | `0.4.0`  | `20a73d025c013fef185a0117607bdffd1f4f5ee4c99b19be63f998cf302dafd5` |
