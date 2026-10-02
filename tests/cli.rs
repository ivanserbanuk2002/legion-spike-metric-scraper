use std::process::{Command, Output};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_legion-spike-metric-scraper"))
        .args(["--data", "data/sample-metrics.json"])
        .args(args)
        .output()
        .expect("CLI executes")
}

#[test]
fn name_prefix_filters_before_top_limit() {
    let output = run(&["--name-prefix", "fake-e", "--top", "1"]);
    assert!(output.status.success(), "{:?}", output);
    let table = String::from_utf8(output.stdout).unwrap();
    assert_eq!(table.lines().count(), 2);
    assert!(table.contains("fake-epsilon"));
    assert!(!table.contains("fake-theta"));

    let empty = run(&["--name-prefix", "absent"]);
    assert!(empty.status.success());
    assert_eq!(String::from_utf8(empty.stdout).unwrap().lines().count(), 1);
}

#[test]
fn minimum_is_inclusive_and_combines_with_prefix() {
    let output = run(&["--name-prefix", "fake-e", "--min-value", "12", "--top", "8"]);
    assert!(output.status.success());
    let table = String::from_utf8(output.stdout).unwrap();
    assert_eq!(table.lines().count(), 3);
    assert!(table.contains("fake-epsilon"));
    assert!(table.contains("fake-eta"));

    let none = run(&["--min-value", "100"]);
    assert!(none.status.success());
    assert_eq!(String::from_utf8(none.stdout).unwrap().lines().count(), 1);
}

#[test]
fn minimum_rejects_non_finite_or_invalid_numbers() {
    for value in ["NaN", "inf", "-inf", "word"] {
        let output = run(&["--min-value", value]);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn json_output_has_filtered_ranked_records_and_empty_array() {
    let output = run(&[
        "--format",
        "json",
        "--name-prefix",
        "fake-e",
        "--min-value",
        "-1",
        "--top",
        "1",
    ]);
    assert!(output.status.success());
    let rows: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        rows,
        serde_json::json!([
            {"rank": 1, "name": "fake-epsilon", "value": 31.75}
        ])
    );

    let empty = run(&["--format", "json", "--top", "0"]);
    assert!(empty.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&empty.stdout).unwrap(),
        serde_json::json!([])
    );
    assert_eq!(run(&["--format", "xml"]).status.code(), Some(2));
}

#[test]
fn output_file_contains_result_without_stdout_or_overwriting_existing_file() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    for format in ["json", "table"] {
        let path = std::env::temp_dir().join(format!(
            "legion-metrics-{}-{nonce}-{format}",
            std::process::id()
        ));
        let filename = path.to_str().unwrap();
        let args = ["--format", format, "--top", "2"];
        let expected = run(&args);
        assert!(expected.status.success());
        let written = run(&["--format", format, "--top", "2", "--output", filename]);
        assert!(written.status.success(), "{:?}", written);
        assert!(written.stdout.is_empty());
        assert_eq!(fs::read(&path).unwrap(), expected.stdout);

        let refused = run(&["--output", filename]);
        assert!(!refused.status.success());
        assert_eq!(fs::read(&path).unwrap(), expected.stdout);
        fs::remove_file(path).unwrap();
    }
}

#[test]
fn substring_filter_combines_with_prefix_and_top() {
    let output = run(&[
        "--name-contains",
        "eta",
        "--name-prefix",
        "fake-",
        "--top",
        "2",
        "--format",
        "json",
    ]);
    assert!(output.status.success());
    let rows: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(rows[0]["name"], "fake-theta");
    assert_eq!(rows[1]["name"], "fake-zeta");
    assert_eq!(rows.as_array().unwrap().len(), 2);
    let empty = run(&["--name-contains", "NO-MATCH", "--format", "json"]);
    assert!(empty.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&empty.stdout).unwrap(),
        serde_json::json!([])
    );
}

#[test]
fn reverse_sorts_ascending_before_applying_top() {
    let output = run(&["--reverse", "--top", "2", "--format", "json"]);
    assert!(output.status.success());
    let rows: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(rows[0]["name"], "fake-gamma");
    assert_eq!(rows[0]["rank"], 1);
    assert_eq!(rows[1]["name"], "fake-eta");
}

#[test]
fn csv_output_contains_header_and_ranked_rows() {
    let output = run(&["--format", "csv", "--top", "1"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "rank,name,value\n1,\"fake-theta\",88.25\n"
    );
    let empty = run(&["--format", "csv", "--top", "0"]);
    assert!(empty.status.success());
    assert_eq!(
        String::from_utf8(empty.stdout).unwrap(),
        "rank,name,value\n"
    );
}
