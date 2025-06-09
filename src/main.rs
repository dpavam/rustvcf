// Script to parse and remove annotations from vcf files
use bgzip::{ BGZFWriter, Compression };
use clap::Parser;
use env_logger;
use flate2::read::MultiGzDecoder;
use log::{info, warn};
use memchr::memchr_iter;
use std::fs::File;
use std::io::{ Write, BufReader, BufRead };

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
        // let bytes = line.as_bytes();
        // let mut position_7: usize = 0;
        // let mut position_8: usize = 0;
        // let mut tab_number_index: i32 = 0;

        // Use memchr to iterate over tabs quicker
        // let tab_positions: Vec<usize> = memchr_iter(b'\t', bytes).collect();

        let line_trimmed = line.trim_end();

        let mut fields: Vec<&str> = line_trimmed.split('\t').collect();
            
            if fields.len()>= 9 {
                fields[7] = "."; 

                let reconstructed = fields.join("\t");
                writer.write_all(reconstructed.as_bytes())?;
                writer.write_all(b"\n")?;
                processed_count+=1;
            }


        
        // Iterate over the tabs
        // if tab_positions.len() >= 8 {
        //     processed_count+=1;
        //     let position_7 = tab_positions[6];
        //     let position_8 = tab_positions[7];

        //     // Variables to store the the slices of the current line

        //     // Expected format: ...data6\tdata7\t + . + \tdata8

        //     // Reconstruct the line by replacing the info space (between position 7 and 8 with a dot.)
        //     writer.write_all(beginning.as_bytes())?;
        //     writer.write_all(b"\t.\t")?;
        //     writer.write_all(end.as_bytes())?;
        //     writes_count+=3;
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
    println!("Input lines read: {}", line_number);
    println!("Headers: {}, Processed: {}, Total writes: {}", header_count, processed_count, writes_count);
    writer.close()?;

    std::fs::write(&args.output, write_buffer)?;
    let duration = start.elapsed();
    info!("Done");
    info!("Execution time: {:.2?}", duration);
    Ok(())
}
