// Script to parse and remove annotations from vcf files
use clap::Parser;
use env_logger;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{info, warn};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;
use crate::validate::validate_vcf_minimal;

mod validate;


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

// v=0.1.2
// This should treat the vcf as a text file and not a vcf
fn main() -> std::io::Result<()> {

    // Enable a logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Start the timer
    let start = Instant::now();
    // Parse cli arguments
    let args = Cli::parse();

    // Log start and process file:
    info!("Starting rustvcf annotation remover...");
    warn!("Warning: this method assumes all VCF fields are present in the data set (eg: INFO is always field number 7)");
    info!("Processing: {:?} ...", args.input);
    info!("Writing: {:?}", &args.output);

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

        // Split data by tabs
        // Get the bytes were data is stored in the line
        // Assing variable to store the bytes of position 7 and 8 (where INFO is) in indexing this would be 6 and 7!
        let bytes = line.as_bytes();
        let mut position_7: usize = 0;
        let mut position_8: usize = 0;
        let mut tab_number_index: i32 = 0;
        
        // Iterate over the bytes
        for (i, &item) in bytes.iter().enumerate() {
            if item == b'\t' {
                if tab_number_index < 6 {
                    tab_number_index +=1;
                } else if tab_number_index == 6 {
                    position_7 = i;
                    tab_number_index +=1;
                } else if tab_number_index == 7{
                    position_8 = i;
                    break;
            }
        }
    }
        // Variables to store the the slices of the current line
        let begining = &line[0..position_7];
        let end = &line[position_8..];
        // Expected format: ...data6\tdata7\t + . + \tdata8

        
        // Reconstruct the line by replacing the info space (between position 7 and 8 with a dot.)
        writeln!(writer, "{}\t.\t{}", begining, end)?;
        // println!("{}\t.\t{}", begining, end);
        }
       
        line.clear();
    }

    let duration = start.elapsed();
    info!("Done");
    info!("Execution time: {:.2?}", duration);
    Ok(())
}
