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

    // Parse cli arguments
    let args = Cli::parse();

    // Let's parse the file with flate2 (compression) and BufReader (reading files)
    let vcf = File::open(args.input)?;
    let decoder = MultiGzDecoder::new(vcf);
    let mut reader = BufReader::new(decoder);

    //Prepare a writer for output file
    let output_file = File::create(args.output)?;
    let encoder = GzEncoder::new(output_file, Compression::fast());
    let mut writer = BufWriter::new(encoder);

    // Read lines and skip those with a #
    let mut line = String::new();
    // Read lines and skip those with a #
    while reader.read_line(&mut line)? > 0 {
        if line.starts_with('#') { 
            writeln!(writer, "{}", line.trim_end())?;
            // println!("{}", line.trim_end());
        } else {

        // Split line by tabs
        // TODO: instead of wiriting this to a vector, let's use string splicing
        let mut fields: Vec<&str> = line.trim_end().split('\t').collect();

        // let info_byte

        // let begining = &line[0..]
        
        // When at index 7 (INFO), replace by a dot
        if fields.len() > 7 {
            fields[7] = ".";
        }

         // Reconstruct the line
        let modified_line = fields.join("\t");
        
        writeln!(writer, "{}", &modified_line)?;
        // println!("{}", &modified_line);
        }
       
        line.clear();
    }

    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);
    Ok(())
}
