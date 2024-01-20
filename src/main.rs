use std::env;

mod csv;
use csv::{CSV, generate_output};

fn main() -> Result<(), std::io::Error> {
    // Arguments:
    let args: Vec<String> = env::args().map(String::from).collect();

    // Creating csv:
    let csv: CSV = CSV::new(args[1].clone())?;
    
    // Creating output:
    match generate_output(&csv) {
        Ok(_) => println!("[INFO] Success"),
        Err(err) => return Err(err),
    }

    Ok(())
}