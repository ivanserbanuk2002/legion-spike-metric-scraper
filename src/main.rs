mod cli;
mod output;
mod parser;

use clap::Parser;
use std::{collections::HashSet, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    if args
        .min_value
        .zip(args.max_value)
        .is_some_and(|(min, max)| min > max)
    {
        clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "--min-value must not exceed --max-value",
        )
        .exit();
    }
    let input = fs::read_to_string(&args.data)?;
    let mut metrics = parser::parse_metrics(&input)?;
    if let Some(prefix) = &args.name_prefix {
        metrics.retain(|metric| metric.name.starts_with(prefix));
    }
    if let Some(fragment) = &args.name_contains {
        metrics.retain(|metric| metric.name.contains(fragment));
    }
    if let Some(minimum) = args.min_value {
        metrics.retain(|metric| metric.value >= minimum);
    }
    if let Some(maximum) = args.max_value {
        metrics.retain(|metric| metric.value <= maximum);
    }
    if args.dedupe {
        let mut names = HashSet::new();
        metrics.retain(|metric| names.insert(metric.name.clone()));
    }
    metrics.sort_by(|left, right| {
        if args.reverse {
            left.value.total_cmp(&right.value)
        } else {
            right.value.total_cmp(&left.value)
        }
    });

    metrics.truncate(args.top);
    match args.output {
        Some(path) => {
            let file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)?;
            output::render(&metrics, args.format, file)
        }
        None => output::render(&metrics, args.format, io::stdout().lock()),
    }
}
