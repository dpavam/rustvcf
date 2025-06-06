# rustvcf
## Purpose
This is a rust CLI tool intended to remove **INFO** annotations from VCF files and replace them with ".".
The objective is to beat `awk` in a current project which takes ~25mins per sample. 

## NOTE:
**WE ARE NOT THERE YET, BUT WE WILL.**


## Install
To install make sure you have cargo v1.87 at least.
To produce the executable run:
```cargo build --release```

## Usage
To use run:
``` ./target/release/rustvcf -t none -i <path/to/input.vcf.gz> -o <path/to/output.vcf.gz> ```
