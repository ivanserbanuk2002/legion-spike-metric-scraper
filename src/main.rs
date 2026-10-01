mod parser;

use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about = "Rank metrics from a local JSON file")]
struct Args {
    /// JSON array of objects with string name and numeric value fields.
    #[arg(long, default_value = "data/sample-metrics.json")]
    data: PathBuf,

    /// Maximum number of ranked metrics to print.
    #[arg(long, default_value_t = 5)]
    top: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = fs::read_to_string(&args.data)?;
    let mut metrics = parser::parse_metrics(&input)?;
    metrics.sort_by(|left, right| right.value.total_cmp(&left.value));

    let count = args.top.min(metrics.len());
    let name_width = metrics
        .iter()
        .take(count)
        .map(|metric| metric.name.chars().count())
        .max()
        .unwrap_or(4)
        .max(4);

    println!("{:>4}  {:name_width$}  {:>12}", "RANK", "NAME", "VALUE");
    for (index, metric) in metrics.iter().take(count).enumerate() {
        println!(
            "{:>4}  {:name_width$}  {:>12}",
            index + 1,
            metric.name,
            metric.value
        );
    }

    Ok(())
}
