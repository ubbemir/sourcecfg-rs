use sourcecfg_rs::{formatters, parser};

const REPEAT_AMOUNT: usize = 10_000;

fn main() {
    let unparsed_file = include_str!("cfg/gamemode_casual.cfg");
    let unparsed_file = unparsed_file.repeat(REPEAT_AMOUNT);

    let config = parser::parse(&unparsed_file).expect("Failed to parse CFG");

    println!("Prettifying data of size {} ...", unparsed_file.len());
    let _ = formatters::prettify(&config);
    println!("Done prettifying");
}
