use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn cli() -> Command {
    Command::new(get_cargo_bin("sourcecfg_fmt"))
}

#[test]
fn test_prettified_snapshot_from_file() {
    assert_cmd_snapshot!(cli().arg("-i").arg("tests/fixtures/gamemode_casual.cfg"))
}

#[test]
fn test_minified_snapshot_from_file() {
    assert_cmd_snapshot!(
        cli()
            .arg("-i")
            .arg("tests/fixtures/gamemode_casual.cfg")
            .arg("-m")
    )
}

#[test]
fn test_prettified_snapshot_from_stdin() {
    assert_cmd_snapshot!(cli().pass_stdin(include_str!("fixtures/gamemode_casual.cfg")))
}

#[test]
fn test_minified_snapshot_from_stdin() {
    assert_cmd_snapshot!(
        cli()
            .arg("-m")
            .pass_stdin(include_str!("fixtures/gamemode_casual.cfg"))
    )
}
