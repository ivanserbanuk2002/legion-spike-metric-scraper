mod cli;
mod output;
mod parser;

use clap::Parser;
use std::{error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let input = fs::read_to_string(&args.data)?;
    let mut metrics = parser::parse_metrics(&input)?;
    if let Some(prefix) = &args.name_prefix {
        metrics.retain(|metric| metric.name.starts_with(prefix));
    }
    if let Some(minimum) = args.min_value {
        metrics.retain(|metric| metric.value >= minimum);
    }
    metrics.sort_by(|left, right| right.value.total_cmp(&left.value));

    metrics.truncate(args.top);
    output::render(&metrics, args.format, io::stdout().lock())
}
