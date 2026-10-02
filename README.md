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
`--max-value 32` keeps values <= 32, including the threshold itself.
Use `--min-value 12 --max-value 32 --name-prefix fake-e --top 2` to rank only the matching range.
Both thresholds accept negative numbers but reject NaN and infinity; JSON metric values must also be finite.
Reversed bounds (`--min-value 32 --max-value 12`) exit with code 2 before reading input or creating output.
`--dedupe` keeps the first qualifying input row for each exact, case-sensitive name.
It runs after all name/value filters and before stable sorting and `--top`, so a filtered-out
row does not reserve its name and a later duplicate with a larger value does not replace the first.
For example, `--name-prefix fake- --min-value 12 --max-value 64.5 --dedupe --top 3 --format json`
filters the input, keeps unique names, then ranks the first three. Without `--dedupe`, duplicates remain.
`--format json` prints an array of `{ "rank": 1, "name": "fake-theta", "value": 88.25 }` records.
Ranks start at 1 after filtering and sorting; an empty result is `[]`. The default format is `table`.
`--output summary.json --format json` writes to a new file and leaves stdout empty.
Existing files are never replaced. Missing parent directories and write failures return a nonzero exit code;
a write failure may leave a partial new file. Input parsing happens before the destination is created.
ADR: Rust was chosen over Python for a standalone binary and a typed data model.
Supplied planning note: Python option was rejected in planning (2025-06-12).
`cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked` run on every push.

Design reference: Mikko Ohtamaa's [finite-value JSON export fix](https://github.com/tradingstrategy-ai/web3-ethereum-defi/commit/382dbe6623bc79a6ed350139d3750ef75c09eb0b).
That export maps invalid optional metadata to null; this CLI rejects non-finite required numbers.
Filtering, ranked JSON and file output are our own additions; no upstream code was copied.

`--name-contains eta` filters by a case-sensitive substring. It combines with
`--name-prefix` and `--min-value` before sorting and applying the top limit.

`--reverse --top 3` ranks the three smallest values first. Equal values keep
input order in either direction, and output ranks still start at 1.

`--format csv` exports `rank,name,value` with a header, quoted names and doubled
embedded quotes. Empty results contain only the header. It works with `--output`.
CSV preserves raw metric names and values rather than altering them for spreadsheets.
