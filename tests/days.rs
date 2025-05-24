use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_day_three() {
    let file = assert_fs::NamedTempFile::new("foobar.txt").unwrap();
    file.write_str("foobarbang     hello do don;t   do() mul(3,5) \n don't() mul(5,4) mul(   4,4)")
        .unwrap();

    let mut cmd = Command::cargo_bin("aoc24").unwrap();
    cmd.arg("day_three").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("35").and(predicates::str::contains("15")));

    file.close().unwrap();
}
