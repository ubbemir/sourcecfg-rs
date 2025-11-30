use sourcecfg_rs::formatters::prettify;

use pretty_assertions::assert_eq;
use sourcecfg_rs::parser;
use std::fs;
use std::path::Path;

const TEST_CASES_PATH: &str = "tests/formatting_cases";

#[test]
fn run_all_test_cases() {
    let test_dir = Path::new(TEST_CASES_PATH);

    for entry in fs::read_dir(test_dir)
        .unwrap_or_else(|_| panic!("Failed to read test cases dir '{}'", TEST_CASES_PATH))
    {
        let entry = entry.expect("Failed to read case dir");
        let path = entry.path();
        if path.is_dir() {
            let case_name = path.file_name().unwrap().to_string_lossy();

            let input_path = path.join("input.cfg");
            let expected_path = path.join("prettify.txt");

            let input = fs::read_to_string(&input_path).unwrap_or_else(|_| {
                panic!(
                    "Failed to read {} for case {}",
                    input_path.to_string_lossy(),
                    case_name
                )
            });
            let expected = fs::read_to_string(&expected_path).unwrap_or_else(|_| {
                panic!(
                    "Failed to read {} for case {}",
                    expected_path.to_string_lossy(),
                    case_name
                )
            });

            let config = parser::parse(&input)
                .unwrap_or_else(|_| panic!("Failed to parse input for case {}", case_name));
            let actual = prettify(&config).expect("Failed to run prettify");

            assert_eq!(
                actual, expected,
                "Test case '{}' failed:\nInput:\n{}\nExpected:\n{}\nGot:\n{}",
                case_name, input, expected, actual
            );
        }
    }
}
