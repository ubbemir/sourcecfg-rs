use sourcecfg_rs::{formatters, parser};

fn main() {
    let unparsed_file = include_str!("cfg/gamemode_casual.cfg");

    let config = parser::parse(unparsed_file).expect("Failed to parse CFG");

    println!("Minified:\n{}", formatters::minify(&config).unwrap());
    println!();
    println!("Prettified:\n{}", formatters::prettify(&config).unwrap());
}
