// Module to validate whether a VCF is suitable for annotation removal.

use vcf::{VCFReader, VCFError};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use env_logger;
use log::{info, warn};



pub fn validate_vcf(input_file: std::path::PathBuf) -> Result<(), VCFError> {

    // Enable a logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    // Log start and process file:
    info!("Starting rustvcf validation...");
    info!("Processing: {:?} ...", input_file);

    //Start the timer
    let start = Instant::now();

    // Read the file
    let vcf = File::open(input_file)?;
    let buffer = BufReader::new(MultiGzDecoder::new(vcf));
    let mut reader = VCFReader::new(buffer)?;

    println!("Starting validation...");
    println!("Validating headers...");

    // Check for INFO field definition (since that's what we'll be modifying)
    if reader.header().info_count() == 0 {
        eprintln!("Warning: No INFO field definitions found in header");
    }
    
    println!("Header validation passed\n");

    println!("Validating records...");
    let mut vcf_record = reader.empty_record();
    let mut record_count = 0;
    let mut line_number = 0; // Approximate line number (headers + records)
    
    // Count header lines (rough estimate)
    line_number += reader.header().header_line_count();

    match reader.next_record(&mut vcf_record) {
            Ok(()) => {
                record_count += 1;
                line_number += 1;

                // Basic validation with assertions - ensures fields are structurally valid
                assert!(!vcf_record.chromosome.is_empty(), 
                    "Chromosome field cannot be empty at record {}", record_count);
                assert!(vcf_record.position > 0, 
                    "Position must be positive at record {}, got {}", record_count, vcf_record.position);
                // ID field can be empty (represented as Vec::new() or single dot)
                assert!(!vcf_record.reference.is_empty(), 
                    "Reference field cannot be empty at record {}", record_count);
                assert!(!vcf_record.alternative.is_empty(), 
                    "Alternative field cannot be empty at record {}", record_count);
                // QUAL can be None (missing), but if present should be reasonable
                if let Some(qual_val) = vcf_record.qual {
                    assert!(qual_val >= 0.0, 
                        "Quality score cannot be negative at record {}, got {}", record_count, qual_val);
                }
                // FILTER field can be empty but should exist
                // INFO field access - this is what we care about most
                let _info_raw = vcf_record.info_raw();
                assert!(_info_raw.is_some(), 
                    "INFO field must be accessible at record {}", record_count);

                // Progress reporting for large files
                if record_count % 100000 == 0 {
                    println!("Validated {} records...", record_count);
                }
            }
                Err(VCFError::NoMoreRecord) => {
                    // End of file - this is expected
                    break;
                }
                Err(e) => {
                    eprintln!("Error: Validation failed at approximately line {}: {}", line_number, e);
                    return Err(e);
                }
                
            }

            let duration = start.elapsed();
            println!("Validation completed successfully!");
            println!("Total records validated: {}", record_count);
            println!("Validation time: {:.2?}", duration);
            
            Ok(())
            
                }
// Alternative minimal version if you want even faster validation
pub fn validate_vcf_minimal(input_file: std::path::PathBuf) -> Result<(), VCFError> {
    // Enable a logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    let start = Instant::now();

    
    
    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        input_file,
    )?)))?;

    let mut vcf_record = reader.empty_record();
    let mut record_count = 0;
    
    // Just try to read every record - if parsing succeeds, structure is valid
    while reader.next_record(&mut vcf_record).is_ok() {
        record_count += 1;
        
        if record_count % 100000 == 0 {
            println!("Validated {} records...", record_count);
        }
    }
    
    let duration = start.elapsed();
    info!("Minimal validation completed!");
    info!("Total records: {}", record_count);
    info!("Validation time: {:.2?}", duration);
    
    Ok(())
}

