use assert_cmd::Command;

#[test]
fn test_cli_default() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    // In automated tests, stdout is a pipe (not a terminal), so by default no trailing newline
    let assert = cmd.assert().success();
    let output = assert.get_output();
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    assert_eq!(stdout.len(), 32);
    assert!(!stdout.ends_with('\n'));
}

#[test]
fn test_cli_force_newline() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.arg("-N");
    let assert = cmd.assert().success();
    let output = assert.get_output();
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    assert!(stdout.ends_with('\n'));
    assert_eq!(stdout.trim().len(), 32);
}

#[test]
fn test_cli_uuid() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["uuid"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout.len(), 36);
    assert_eq!(stdout.chars().filter(|&c| c == '-').count(), 4);

    let mut cmd_v7 = Command::cargo_bin("rrand").unwrap();
    cmd_v7.args(["uuid", "--v7", "--simple"]);
    let assert_v7 = cmd_v7.assert().success();
    let stdout_v7 = String::from_utf8(assert_v7.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout_v7.len(), 32);
    assert!(!stdout_v7.contains('-'));
}

#[test]
fn test_cli_ulid() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["ulid"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout.len(), 26);
    assert_eq!(stdout, stdout.to_uppercase());

    let mut cmd_lower = Command::cargo_bin("rrand").unwrap();
    cmd_lower.args(["ulid", "--lower"]);
    let assert_lower = cmd_lower.assert().success();
    let stdout_lower = String::from_utf8(assert_lower.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout_lower.len(), 26);
    assert_eq!(stdout_lower, stdout_lower.to_lowercase());
}

#[test]
fn test_cli_base36() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["base36", "-l", "16", "--upper"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout.len(), 16);
    assert_eq!(stdout, stdout.to_uppercase());
}

#[test]
fn test_cli_base62_and_base58() {
    let mut cmd62 = Command::cargo_bin("rrand").unwrap();
    cmd62.args(["base62", "-l", "20"]);
    let assert62 = cmd62.assert().success();
    let stdout62 = String::from_utf8(assert62.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout62.len(), 20);

    let mut cmd58 = Command::cargo_bin("rrand").unwrap();
    cmd58.args(["base58", "-l", "20"]);
    let assert58 = cmd58.assert().success();
    let stdout58 = String::from_utf8(assert58.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout58.len(), 20);
    assert!(!stdout58.contains('0'));
    assert!(!stdout58.contains('O'));
    assert!(!stdout58.contains('I'));
    assert!(!stdout58.contains('l'));
}

#[test]
fn test_cli_bytes_hex_and_base64() {
    let mut cmd_hex = Command::cargo_bin("rrand").unwrap();
    cmd_hex.args(["bytes", "16", "--hex"]);
    let assert_hex = cmd_hex.assert().success();
    let stdout_hex = String::from_utf8(assert_hex.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout_hex.len(), 32); // 16 bytes = 32 hex chars

    let mut cmd_b64 = Command::cargo_bin("rrand").unwrap();
    cmd_b64.args(["bytes", "16", "--base64"]);
    let assert_b64 = cmd_b64.assert().success();
    let stdout_b64 = String::from_utf8(assert_b64.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout_b64.len(), 24); // 16 bytes in standard base64 is 24 chars
}

#[test]
fn test_cli_int() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["int", "--min", "10", "--max", "20", "-c", "5", "-d", ","]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let nums: Vec<i64> = stdout.split(',').map(|s| s.parse().unwrap()).collect();
    assert_eq!(nums.len(), 5);
    for n in nums {
        assert!((10..=20).contains(&n));
    }
}

#[test]
fn test_cli_coin_and_dice() {
    let mut cmd_coin = Command::cargo_bin("rrand").unwrap();
    cmd_coin.args(["coin"]);
    let assert_coin = cmd_coin.assert().success();
    let stdout_coin = String::from_utf8(assert_coin.get_output().stdout.clone()).unwrap();
    assert!(stdout_coin == "Heads" || stdout_coin == "Tails");

    let mut cmd_dice = Command::cargo_bin("rrand").unwrap();
    cmd_dice.args(["dice", "2d6", "-d", "+"]);
    let assert_dice = cmd_dice.assert().success();
    let stdout_dice = String::from_utf8(assert_dice.get_output().stdout.clone()).unwrap();
    let rolls: Vec<u32> = stdout_dice.split('+').map(|s| s.parse().unwrap()).collect();
    assert_eq!(rolls.len(), 2);
    for r in rolls {
        assert!((1..=6).contains(&r));
    }
}

#[test]
fn test_cli_passphrase() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["passphrase", "-w", "5", "-s", "."]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let parts: Vec<&str> = stdout.split('.').collect();
    assert_eq!(parts.len(), 5);
}

#[test]
fn test_cli_pick_args_and_stdin() {
    let mut cmd_args = Command::cargo_bin("rrand").unwrap();
    cmd_args.args(["pick", "apple", "banana", "cherry"]);
    let assert_args = cmd_args.assert().success();
    let stdout_args = String::from_utf8(assert_args.get_output().stdout.clone()).unwrap();
    assert!(["apple", "banana", "cherry"].contains(&stdout_args.as_str()));

    let mut cmd_stdin = Command::cargo_bin("rrand").unwrap();
    cmd_stdin.arg("pick");
    cmd_stdin.write_stdin("dog\ncat\nbird\n");
    let assert_stdin = cmd_stdin.assert().success();
    let stdout_stdin = String::from_utf8(assert_stdin.get_output().stdout.clone()).unwrap();
    assert!(["dog", "cat", "bird"].contains(&stdout_stdin.as_str()));
}

#[test]
fn test_cli_lorem() {
    let mut cmd = Command::cargo_bin("rrand").unwrap();
    cmd.args(["lorem", "-w", "15"]);
    let assert = cmd.assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout.split_whitespace().count(), 15);
}
