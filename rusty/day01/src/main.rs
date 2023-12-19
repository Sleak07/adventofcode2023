// main.rs or another module

mod file_reader; // Assuming file_reader.rs is in the same directory

use file_reader::read_file_line_by_line;

fn main() {
    if let Err(err) = read_file_line_by_line("input.txt") {
        eprintln!("Error: {}", err);
    }
}


