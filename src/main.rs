//a command line tool to play the game Rock-Paper-Scissors with computer
use clap::Parser;
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Jinghuai Zhang")]
    Play {
        #[clap(short, long)]
        tgt: u32,
    },
}
// create the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { tgt }) => {
            println!("Hello, {}!", tgt);
            game::game(tgt);
        }
        None => println!("No command was used"),
    }
}