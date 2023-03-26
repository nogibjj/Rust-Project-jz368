extern crate clap;
use clap::Parser;
extern crate chinese_dictionary;
use chinese_dictionary::query;
use chinese_dictionary::{MeasureWord, WordEntry};
use chinese_dictionary::{tokenize};

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(short, long, default_value = "interesting")]
    text: String,
}

fn main() {
    let args = Args::parse();
    let text: String = args.text.to_string();
    println!("Original sentence: {}", text);

    let text_str = text.as_str();
    let results = tokenize(text_str);
    let mut result_str = "".to_string();
    for (pos, r) in results.iter().enumerate() {
        result_str.push_str(r);
        result_str.push_str(" ");
    }
    println!("Segmented sentence: {}", result_str);
}