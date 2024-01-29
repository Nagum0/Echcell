use std::env;
use std::process::exit;
use echcell::error::CsvError;
use echcell::csv::{
    CSV,
    generate_output,
};

fn main() {
    // Arguments:
    let args: Vec<String> = env::args().map(String::from).collect();
    
    if args.len() == 1 {
        eprintln!("\n\t{}\n", CsvError::RunError("No CSV file was given...".to_string()));
        exit(1);
    }

    // Creating csv object:
    let csv = match CSV::new(args[1].clone()) {
        Ok(val) => val,
        Err(e)  => {
            eprintln!("\n\t{e}\n");
            exit(1);
        },
    }; 

    // Generating output:
    match generate_output(&csv) {
        Ok(_) => println!("\n\tOutput file successfully created!\n"),
        Err(err) => {
            eprintln!("\n\t{err}\n");
            exit(1);
        },
    }
}
