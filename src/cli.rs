use crate::output::Format;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Rank metrics from a local JSON file")]
pub struct Args {
    /// JSON array of objects with string name and numeric value fields.
    #[arg(long, default_value = "data/sample-metrics.json")]
    pub data: PathBuf,

    /// Maximum number of ranked metrics to print.
    #[arg(long, default_value_t = 5)]
    pub top: usize,

    /// Keep only metrics whose names start with this case-sensitive prefix.
    #[arg(long)]
    pub name_prefix: Option<String>,

    /// Keep names containing this case-sensitive substring.
    #[arg(long)]
    pub name_contains: Option<String>,

    /// Keep values greater than or equal to this finite threshold.
    #[arg(long, value_parser = finite_value, allow_hyphen_values = true)]
    pub min_value: Option<f64>,

    /// Rank smallest values first instead of largest.
    #[arg(long)]
    pub reverse: bool,

    /// Render a readable table or a JSON array of ranked records.
    #[arg(long, value_enum, default_value = "table")]
    pub format: Format,

    /// Write to a new file instead of stdout; existing files are never replaced.
    #[arg(long)]
    pub output: Option<PathBuf>,
}

fn finite_value(value: &str) -> Result<f64, String> {
    let value: f64 = value.parse().map_err(|_| "expected a finite number")?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err("expected a finite number".into())
    }
}
