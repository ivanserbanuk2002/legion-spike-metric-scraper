use crate::parser::Metric;
use clap::ValueEnum;
use serde::Serialize;
use std::{error::Error, io::Write};

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Table,
    Json,
}

#[derive(Serialize)]
struct RankedMetric<'a> {
    rank: usize,
    name: &'a str,
    value: f64,
}

pub fn render(
    metrics: &[Metric],
    format: Format,
    mut writer: impl Write,
) -> Result<(), Box<dyn Error>> {
    match format {
        Format::Json => {
            let rows: Vec<_> = metrics
                .iter()
                .enumerate()
                .map(|(index, metric)| RankedMetric {
                    rank: index + 1,
                    name: &metric.name,
                    value: metric.value,
                })
                .collect();
            serde_json::to_writer_pretty(&mut writer, &rows)?;
            writeln!(writer)?;
        }
        Format::Table => {
            let name_width = metrics
                .iter()
                .map(|metric| metric.name.chars().count())
                .max()
                .unwrap_or(4)
                .max(4);
            writeln!(
                writer,
                "{:>4}  {:name_width$}  {:>12}",
                "RANK", "NAME", "VALUE"
            )?;
            for (index, metric) in metrics.iter().enumerate() {
                writeln!(
                    writer,
                    "{:>4}  {:name_width$}  {:>12}",
                    index + 1,
                    metric.name,
                    metric.value
                )?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}
