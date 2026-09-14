use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run_fixture(name: &str, mode: &str) -> Vec<u8> {
    let fixture = repo_root().join("fixtures/mvp").join(name);
    let output = Command::new(env!("CARGO_BIN_EXE_ontology-engine"))
        .arg("discover")
        .arg(&fixture)
        .arg("--include-files")
        .arg(mode)
        .arg("--max-depth")
        .arg("4")
        .output()
        .expect("ontology-engine should execute");

    assert!(
        output.status.success(),
        "fixture {name} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn count_level(report: &Value, level: u8) -> u64 {
    let prefix = format!("{level:02}_");
    report["nodes_by_level"]
        .as_object()
        .expect("nodes_by_level object")
        .iter()
        .filter(|(key, _)| key.starts_with(&prefix))
        .map(|(_, value)| value.as_u64().expect("level count"))
        .sum()
}

fn assert_common_invariants(report: &Value) {
    assert_eq!(report["ontology"], "1.0.0");
    assert_eq!(report["read_only"], true);
    assert!(count_level(report, 22) > 0, "REPOSITORY must exist");
    assert!(count_level(report, 23) > 0, "SOURCE must exist");
    assert_eq!(count_level(report, 24), 0, "UNIT must remain unmaterialized");
    assert_eq!(count_level(report, 25), 0, "MODULE must remain unmaterialized");
    assert!(count_level(report, 27) > 0, "COMPONENT must exist");
    assert!(count_level(report, 28) > 0, "ELEMENT must exist");

    let semantic_levels = [30_u8, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46];
    let semantic_count: u64 = semantic_levels
        .into_iter()
        .map(|level| count_level(report, level))
        .sum();
    assert!(semantic_count > 0, "fixture must project semantic evidence");
}

fn assert_byte_deterministic(name: &str, mode: &str) {
    let first = run_fixture(name, mode);
    let second = run_fixture(name, mode);
    assert_eq!(first, second, "repeated scans must be byte-identical for {name}");

    let report: Value = serde_json::from_slice(&first).expect("valid discovery JSON");
    assert_common_invariants(&report);
}

#[test]
fn rust_fixture_is_byte_deterministic() {
    assert_byte_deterministic("rust", "--rust-ast");
}

#[test]
fn typescript_fixture_is_byte_deterministic() {
    assert_byte_deterministic("typescript", "--syntax");
}

#[test]
fn python_fixture_is_byte_deterministic() {
    assert_byte_deterministic("python", "--syntax");
}
