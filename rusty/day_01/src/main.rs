use ::std ::io ::BufReader;
use std::fs::File;
use std ::io ::prelude ::* ;


fn main () -> std ::io ::Result<()>{
    let file = File ::open("input.txt")?;
    let  buf_reader = BufReader::new(file);

    // iterating over the contents of the file

    for line in buf_reader.lines(){
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading line: {}", e)
        }
    }
    Ok(())

}
