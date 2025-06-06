// Script to parse and remove annotations from vcf files


use clap::Parser;
// use vcf::{VCFReader, VCFError, VCFWriter};
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;

// Struct to specify the type of CLI arguments
#[derive(Parser)]
struct Cli {
    // Sets the tool/action to perform on the vcf
    #[arg(short, long)]
    tool: String,

    // Path to input vcf file
    #[arg(short, long)]
    input: std::path::PathBuf,

    // Path to output vcf file
    #[arg(short, long)]
    output: std::path::PathBuf,
}

// v=0.1.1
// This should treat the vcf as a text file and not a vcf
fn main() -> std::io::Result<()> {

    // Start the timer
    let start = Instant::now();
    
    let args = Cli::parse();

    // Read the file
    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(args.input)?)))?;
    
    // Create a writer
    let mut writer = VCFWriter::new(File::create(args.output)?, &reader.header().clone())?;

    // Create an empty record to read
    let mut vcf_record = reader.empty_record();

    // Loop over the reader and write everything except the other fields in the INFO column
    while reader.next_record(&mut vcf_record)? {

        vcf_record.info.clear();
        writer.write_record(&vcf_record)?;
    }
    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);
    Ok(())
}
