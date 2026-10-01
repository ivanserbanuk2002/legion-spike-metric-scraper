use std::process::{Command, Output};

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
