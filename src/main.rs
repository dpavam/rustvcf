// Script to parse and remove annotations from vcf files
use bgzip::{ BGZFWriter, Compression };
use clap::Parser;
use env_logger;
use flate2::read::MultiGzDecoder;
use log::{info, warn, debug, error};
use std::fs::File;
use std::io::{ Write, BufReader, BufRead };
use std::path::Path;
use std::time::Instant;
use crate::validate::validation_tools;
use crate::benchmark::benchmarking_tools;
mod validate;
mod benchmark;


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

    // Log passed files
    info!("Processing file: {:?}", &args.input);
    info!("Output file: {:?}", &args.output);

    // Logic to execute appropriate flag
    match args.tool.as_str() {
        "deannotate" => {
            remove_annotations(&args.input, &args.output)?
        },
        "validate" => {
            validation_tools::validate_vcf_minimal(&args.input)
            .expect("VCF validation failed");
        },
        "benchmark" => {
            benchmarking_tools::benchmark(&args.input, &args.output);
        },
        // Error handling?
        _ => {
            error!("Error: Unexpected tool specified {}", args.tool);
            panic!();
        }
    }

    let duration = start.elapsed();
    info!("Done");
    info!("Execution time: {:.2?}", duration);

    Ok(())
}


pub fn remove_annotations(input: &Path, output: &Path) -> std::io::Result<()> {
    // Log start and process file:
    info!("Starting deannotation...");
    warn!("Warning: this method assumes all VCF fields are present in the data set (eg: INFO is always field number 7)");

    // Let's parse the file with flate2 (compression) and BufReader (reading files)
    let vcf = File::open(input)?;
    let decoder = MultiGzDecoder::new(vcf);
    let mut reader = BufReader::new(decoder);

    //Prepare a writer for output file
    let mut write_buffer = Vec::new();
    let mut writer = BGZFWriter::new(&mut write_buffer, Compression::fast());

    // Read lines and skip those with a #
    let mut line = String::new();

    // Keep track of line number for debugging purposes
    let mut line_number = 0;
    let mut header_count = 0;
    let mut processed_count = 0;
    let mut writes_count = 0; // Track total writes to the file


    // Read lines and skip those with a #
    while reader.read_line(&mut line)? > 0 {
        // Register the line number
        line_number += 1;
        // Check the begining of a line
        if line.starts_with('#') { 
            header_count +=1;
            writer.write_all(line.as_bytes())?;
            writes_count+=1;
            // println!("{}", line.trim_end());
        } else {

        // Split data by tabs
        // Get the bytes were data is stored in the line
        // Assing variable to store the bytes of position 7 and 8 (where INFO is) in indexing this would be 6 and 7!
        let line_trimmed = line.trim_end();

        let mut fields: Vec<&str> = line_trimmed.split('\t').collect();
            
            if fields.len()>= 9 {
                fields[7] = "."; 

                let reconstructed = fields.join("\t");
                writer.write_all(reconstructed.as_bytes())?;
                writer.write_all(b"\n")?;
                processed_count+=1;
            }


        else {
            // handle malformed cases
            // write as is
            writer.write_all(line.as_bytes())?;
            writes_count+=1;
            info!("Line at {} is malformed. Written as is.", line_number);
        }
        // println!("{}\t.\t{}", begining, end);
        }
       
        line.clear();
        
    }
    // Debug statements
    debug!("Input lines read: {}", line_number);
    debug!("Headers: {}, Processed: {}, Total writes: {}", header_count, processed_count, writes_count);
    writer.close()?;

    std::fs::write(output, write_buffer)?;
    Ok(())
}