use std::process::Command;
fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_swapboost"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn help_and_version() {
    assert!(run(&["--help"]).status.success());
    assert!(run(&["--version"]).status.success());
}
#[test]
fn rejects_invalid_and_mutating_commands() {
    for args in [
        vec!["monitor", "--interval", "0"],
        vec!["monitor", "--count", "0"],
        vec!["config", "apply"],
        vec!["benchmark"],
    ] {
        assert!(!run(&args).status.success());
    }
}
#[cfg(any(target_os = "linux", windows))]
#[test]
fn real_json_and_bounded_monitor() {
    let result = run(&["status", "--json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(value["snapshot"]["physical_total_bytes"].as_u64().unwrap() > 0);
    assert_eq!(value["changes_applied"], false);
    let result = run(&["monitor", "--count", "2", "--interval", "1", "--json"]);
    assert!(result.status.success());
    assert_eq!(String::from_utf8(result.stdout).unwrap().lines().count(), 2);
}
#[cfg(not(any(target_os = "linux", windows)))]
#[test]
fn unsupported_platform_fails_honestly() {
    let result = run(&["status"]);
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("not supported"));
}
