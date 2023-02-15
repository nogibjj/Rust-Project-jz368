extern crate clap;
use clap::Parser;
use std::{process};
use csv::ReaderBuilder;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(short, long, default_value = "test.csv")]
    name: String,
}

fn example(name: String) -> Result<(), csv::Error> {
    let mut reader = ReaderBuilder::new().delimiter(b';').from_path(name)?;
    for result in reader.records() {
        let record = result?;
        for field in &record {
            println!("{:?}", field);
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(err) = example(args.name) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}