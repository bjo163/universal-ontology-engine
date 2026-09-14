use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn engine() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ontology-engine"));
    command
        .arg("--registry")
        .arg(repo_root().join("specifications/universal-ontology-v1.0.json"));
    command
}

fn temp_root(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("ontology-cli-exit-{label}-{stamp}"))
}

#[test]
fn missing_workspace_is_exit_two() {
    let root = temp_root("missing");
    let output = engine().arg("discover").arg(&root).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("workspace preflight failed"));
    assert!(output.stdout.is_empty());
}

#[test]
fn invalid_level_is_exit_two() {
    let output = engine().arg("inspect").arg("50").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid level"));
}

#[test]
fn unsupported_parse_language_is_exit_two() {
    let root = temp_root("language");
    fs::create_dir_all(&root).unwrap();
    let file = root.join("demo.txt");
    fs::write(&file, "demo\n").unwrap();
    let output = engine()
        .arg("parse")
        .arg("not-a-language")
        .arg(&file)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported language"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn degraded_self_is_exit_one() {
    let root = temp_root("self");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
    let output = engine()
        .arg("self")
        .arg(&root)
        .arg("--json")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "degraded");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn structural_discovery_emits_versioned_json_only() {
    let fixture = repo_root().join("fixtures/mvp/typescript");
    let output = engine().arg("discover").arg(&fixture).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema"], "universal-ontology-engine/discovery-v1");
    assert_eq!(report["mode"], "structural");
    assert_eq!(report["read_only"], true);
}

#[cfg(unix)]
#[test]
fn symlink_workspace_is_exit_two() {
    use std::os::unix::fs::symlink;

    let root = temp_root("symlink-root");
    let link = temp_root("symlink-link");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("package.json"), "{\"name\":\"safe\"}\n").unwrap();
    symlink(&root, &link).unwrap();

    let output = engine().arg("discover").arg(&link).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("workspace symlink"));

    fs::remove_file(link).unwrap();
    fs::remove_dir_all(root).unwrap();
}
