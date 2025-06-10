// Module to validate whether a VCF is suitable for annotation removal.



pub mod validation_tools {

    use vcf::{VCFReader, VCFError};
    use flate2::read::MultiGzDecoder;
    // use bgzip::{BGZFReader, BGZFError};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use log::{info};


    pub fn validate_vcf_minimal(input: &Path) -> Result<(), VCFError> {
    info!("Starting validation...");
    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        &input,
    )?)))?;

    let mut vcf_record = reader.empty_record();
    let mut record_count = 0;
    
    // Just try to read every record - if parsing succeeds, structure is valid
    while reader.next_record(&mut vcf_record).is_ok() {
        record_count += 1;
    }
    

    info!("Minimal validation completed!");
    info!("Total records: {}", record_count);
    
    Ok(())
}
}

// TODO: This works but it's taking too long. 
//1 CHECK how many records are expected
// 2 Check it actually goes through the expected number of records.