// file_reader.rs

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file_line_by_line(file_path: &str) -> io::Result<()> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(line_content) => {
                // Process or print the line content
                println!("{}", line_content);
            }
            Err(err) => {
                // Handle the error, e.g., print an error message
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    Ok(())
}

    
