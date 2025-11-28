use clap::Parser;
use sourcecfg_rs::formatters;
use std::fs;
use std::io::{self, Read};

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

fn read_content<T: Read>(args: &Args, stdin: &mut T) -> io::Result<String> {
    let content: String;
    if let Some(cfg_file) = &args.input {
        content = fs::read_to_string(cfg_file)?;
    } else {
        let mut input = String::new();
        stdin.read_to_string(&mut input)?;
        content = input;
    }
    Ok(content)
}

fn main() {
    let args = Args::parse();

    let content = read_content(&args, &mut io::stdin()).unwrap();

    let output = if args.minify {
        formatters::minify(&content).unwrap()
    } else {
        formatters::prettify(&content).unwrap()
    };

    println!("{}", output);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs::File;
    use std::io::{Cursor, Write};
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};

    const FAKE_STDIN: &str = "Fake stdin input";
    const FAKE_FILECONTENT: &str = "Fake file content";

    struct MockEnv {
        fake_stdin: Cursor<&'static str>,
        _temp_file: File,
        _temp_dir: TempDir,
        temp_file_path: PathBuf,
    }

    fn setup_mock_env() -> MockEnv {
        let dir = tempdir().expect("TEST FAIL: Failed to create a temporary directory.");

        let file_path = dir.path().join("temp.cfg");
        let mut file =
            File::create(&file_path).expect("TEST FAIL: Failed to create a temporary file.");
        write!(file, "{}", FAKE_FILECONTENT)
            .expect("TEST FAILED: Failed to write to temporary file.");

        let fake_stdin_input = Cursor::new(FAKE_STDIN);

        MockEnv {
            fake_stdin: fake_stdin_input,
            _temp_file: file,
            _temp_dir: dir,
            temp_file_path: file_path,
        }
    }

    #[test]
    fn test_read_content_from_file() {
        let mut mock_env = setup_mock_env();

        // The heart of this test case, determining here that we should use an input file to read from.
        let args = Args {
            input: Some(mock_env.temp_file_path.to_str().unwrap().into()),
            minify: false, // does not matter for this test case
        };

        let content =
            read_content(&args, &mut mock_env.fake_stdin).expect("TEST FAIL: read_content failed");
        assert_eq!(content, FAKE_FILECONTENT);
        assert_ne!(content, FAKE_STDIN);
    }

    #[test]
    fn test_read_content_from_stdin() {
        let mut mock_env = setup_mock_env();

        let args = Args {
            input: None,
            minify: false, // does not matter for this test case
        };

        let content =
            read_content(&args, &mut mock_env.fake_stdin).expect("TEST FAIL: read_content failed");
        assert_eq!(content, FAKE_STDIN);
        assert_ne!(content, FAKE_FILECONTENT);
    }
}
