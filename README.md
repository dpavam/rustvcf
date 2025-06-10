# rustvcf
## Purpose
This is a rust CLI tool intended to remove **INFO** annotations from VCF files and replace them with ".".
The objective is to beat `awk` in a current project which takes ~25mins per sample. 

## Performance
### From the 1000 genomes project

| File           | Size | Time |
| :---------------- | :------: | ----: |
| ALL.chr14.shapeit2_integrated_snvindels_v2a_27022019.GRCh38.phased.vcf.gz       |   163MB   | ~40s |
| ALL.chr22.shapeit2_integrated_snvindels_v2a_27022019.GRCh38.phased.vcf.gz          |   383MB   | ~116s |


## Install
To install make sure you have cargo v1.87 at least.
To produce the executable run:
```cargo build --release```

## Usage

### Compile
```cargo build --release``` 

### For deannotation:
``` ./target/release/rustvcf -t deannotate -i <path/to/input.vcf.gz> -o <path/to/output.vcf.gz> ```

### For validation of the deannotated file (experimental/slow):
``` ./target/release/rustvcf -t validate -i <path/to/deannotated_vcf.gz> -o None ```
