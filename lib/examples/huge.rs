use sourcecfg_rs::formatters;

const REPEAT_AMOUNT: usize = 10_000;

fn main() {
    let unparsed_file = include_str!("cfg/gamemode_casual.cfg");
    let unparsed_file = unparsed_file.repeat(REPEAT_AMOUNT);

    println!("Prettifying data of size {} ...", unparsed_file.len());
    let _ = formatters::prettify(&unparsed_file).unwrap();
    println!("Done prettifying");
}
