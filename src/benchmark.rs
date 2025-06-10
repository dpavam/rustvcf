// Module to execute bash commands for benchmarking against awk 

// Define a function that runs
// 1. The commands in awk and times them 
// 2. The commands with the in built logic and times them 
// 3. Writes a report in the format
/*
| Tool | File | Time |
|
 */

pub mod benchmarking_tools {
    use log::{info};
    use std::path::Path;
    use std::process::{Command};
    use std::time::Instant;

    // Main benchmark function to be ran in main.py
     pub fn benchmark(input: &Path, output: &Path) {
        info!("Starting benchmarking...");

        // Benchmark awk
        let start = Instant::now();
        let _ = awk_benchmarking::benchmark_awk(&input, &output);
        let duration = start.elapsed();
        info!("Done");
        info!("Awk time: {:.2?}", duration);
        clean_up(&output);

        // Benchmark self
        let start = Instant::now();
        let _ = self_benchmark::benchmark_self(&input, &output);
        let duration = start.elapsed();
        info!("Done");
        info!("rustvcf time: {:.2?}", duration);
        clean_up(&output);
     }


     mod awk_benchmarking {

        use super::Path;
        use super::info;
        use super::Command;
        
            // Functions of the awk benchmarking module
            pub fn benchmark_awk(input: &Path, output: &Path){

            info!("Benchmarking awk...");

            let awk_bash_command = format!(
            r#"bcftools view "{}" | \
            awk 'BEGIN {{ FS="\t"; OFS="\t" }}
            /^#/ {{ print; next }}
            {{
                split($8, info_parts, "\\|");
                $8 = info_parts[1];
                print;
            }}' | \
            bgzip -@ 1 > "{}" && \
            bcftools index "{}""#,
            input.display(),
            output.display(),
            output.display()
        );

        // Execute the command
        
        let awk_command_status = Command::new("bash")
            .arg("-c")
            .arg(&awk_bash_command)
            .status();

            info!("Command finished with status {:?}", awk_command_status);
        }
        
     }

     mod self_benchmark {

        use super::Path;
        use super::info;
        use crate::benchmark::benchmarking_tools::bcftools_index;
        use crate::remove_annotations;

        // Define function to run the self command
        pub fn benchmark_self(input: &Path, output: &Path) {
            info!("Benchmarking awk...");
            let _ = remove_annotations(&input, &output);
            bcftools_index(&output);
        }  
     }

    // bcftools indexing function
    fn bcftools_index(output: &Path){

        let bcf_index_formated_command = format!("bcftools index {}", &output.display());

        let bcf_index_command_status = Command::new("bash")
        .arg("-c")
        .arg(bcf_index_formated_command)
        .status();

        info!("Bcftools indexing finished with status {:?}", bcf_index_command_status);
    }
     
     


    // Clean up function
    fn clean_up(output: &Path) {
        info!("Cleaning up {}", &output.display());
        let formated_file_path = format!("{}*", &output.display());
        let clean_up_status = Command::new("bash")
        .arg("-c")
        .arg(format!("rm -r {}", formated_file_path))
        .status();

        info!("Clean up command finished with status {:?}", clean_up_status);
    }


}
