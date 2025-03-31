use std::fs;
use sourcecfg_rs::formatters;

fn main() {
    let unparsed_file = fs::read_to_string("examples/test.cfg").unwrap();

    println!("{}", formatters::traverse(&unparsed_file).unwrap());
}