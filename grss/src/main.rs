use std::io::{BufRead, BufReader};
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let f = File::open(args.path).expect("wrong path");
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            },
            Err(error) => println!("error occured while reading file lines: {:?}", error)
        };
    }

}
