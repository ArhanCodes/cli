
#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file, and display the lines containing the parsed pattern.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path for the file to read.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct NoClueMate(String);

fn main() -> Result<(), NoClueMate> {

    let args = Cli::parse();
    
    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| NoClueMate(format!("Error! Cannot read file. {}", err)))?;
    
    println!{"Content: \n{}", content}

    Ok(())    
}
