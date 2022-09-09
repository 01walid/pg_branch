use assert_cmd::{prelude::OutputAssertExt, Command};

mod tests {
    use super::*;

    #[test]
    fn cli_tests() {
        let mut cmd = Command::cargo_bin("pg_branch").unwrap();
        let output = cmd.arg("-V").output().unwrap();
        let msg = String::from_utf8_lossy(&output.stdout).to_string();
        insta::assert_snapshot!(msg);
        output.assert().success();
    }
}
