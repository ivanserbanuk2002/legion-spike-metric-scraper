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
