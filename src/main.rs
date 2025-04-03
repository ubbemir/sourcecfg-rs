use std::fs;
use sourcecfg_rs::formatters;

fn main() {
    let unparsed_file = fs::read_to_string("examples/test.cfg").unwrap();

    println!("Minified:\n{}", formatters::minify(&unparsed_file).unwrap());
    println!("");
    println!("Prettified:\n{}", formatters::prettify(&unparsed_file).unwrap());
}