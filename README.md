# legion-spike-metric-scraper

A small Rust spike that reads local JSON metrics and prints a ranked top-N table.
**Status:** spike complete, pending review for ci-tooling integration.

Requires Rust 1.75.0; the toolchain and dependency lockfile are pinned.
Run `cargo run -- --data data/sample-metrics.json --top 5`.
`--top` defaults to 5; 0 prints the header, and larger limits print all available rows.
Input is a JSON array of `{"name":"example","value":12.5}`; the sample contains eight fake rows.
Values sort descending; equal values retain their input order.
Use `--name-prefix fake-e --top 1` to filter names before ranking; matching is case-sensitive.
An unmatched prefix prints only the table header.
`--min-value 12` keeps values >= 12; combine it with the prefix filter before applying `--top`.
The threshold accepts negative numbers but rejects NaN and infinity; JSON metric values must also be finite.
ADR: Rust was chosen over Python for a standalone binary and a typed data model.
Supplied planning note: Python option was rejected in planning (2025-06-12).
`cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked` run on every push.
