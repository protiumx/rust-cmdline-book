use assert_cmd::Command;

#[test]
fn runs_true() {
    Command::cargo_bin("true").unwrap().assert().success();
}

#[test]
fn runs_false() {
    Command::cargo_bin("false").unwrap().assert().failure();
}
