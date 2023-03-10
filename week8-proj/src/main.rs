extern crate webster;
extern crate thesaurus;
extern crate clap;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(short, long, default_value = "interesting")]
    word: String,
}

fn main() {
    let args = Args::parse();
    let word: String = args.word.to_string();
    println!("{}", word);

    let definition = webster::dictionary(word.clone()).unwrap();
    println!("Definition of the word '{}': {}", word, definition);

    let synonyms = thesaurus::synonyms(&word);
    println!("{} synonyms for \"{word}\"...", synonyms.len());
    for x in &synonyms {
        println!("{x}");
    }
}