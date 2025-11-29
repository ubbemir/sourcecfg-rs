use sourcecfg_rs::formatters;

fn main() {
    let unparsed_file = include_str!("cfg/gamemode_casual.cfg");

    println!("Minified:\n{}", formatters::minify(unparsed_file).unwrap());
    println!();
    println!(
        "Prettified:\n{}",
        formatters::prettify(unparsed_file).unwrap()
    );
}
