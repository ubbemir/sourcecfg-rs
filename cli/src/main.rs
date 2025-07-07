use sourcecfg_rs::formatters;
use clap::Parser;
use std::io::{self, Read};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Read CFG from a file instead of stdin
    #[arg(short, long)]
    input: Option<String>,

    /// Minify instead of prettifying
    #[arg(short, long)]
    minify: bool,
}
fn main() {
    let args = Args::parse();

    let content: String;
    if let Some(cfg_file) = args.input {
        content = fs::read_to_string(&cfg_file)
            .expect(&format!("Could not read file: {}", cfg_file));
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
        content = input;
    }

    if args.minify {
        println!("{}", formatters::minify(&content).unwrap());
    } else {
        println!("{}", formatters::prettify(&content).unwrap());
    }
}

// TODO: Make a separate function for taking an Args as input, reading input data based on how user chose to and returning it as a String. Make it purely functional with mockup IO possible to it is
// unit testable.