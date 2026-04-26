use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_smoke() {
    Command::cargo_bin("chyren")
        .expect("build chyren bin")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn status_json_smoke() {
    let out = Command::cargo_bin("chyren")
        .expect("build chyren bin")
        .args(["status", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let v: serde_json::Value =
        serde_json::from_slice(&out).expect("status --json should emit valid JSON");
    assert_eq!(v["ok"], true);
    assert_eq!(v["command"], "status");
    assert_eq!(v["status"], "SEALED");
}
