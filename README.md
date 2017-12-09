# Backtrack
A simple Backtrack algorithm implementation in Rust.
Solves the 8 Queen Puzzle and Trunks problem as an example, and other puzzles
can be added by implementing the `Config` trait.

To build the example binaries:
```bash
cargo build --release
```

This will create `trunk-pack` and `8queens` in `target/release`.

If you want to run them without the overhead of cargo getting in the way, run
them directly from the `target/releases` directory.

Benchmarking (requires nightly Rust):
```bash
cargo bench
```

## Trunk Pack
This binary will take in an optional text file containing the starting config.
See the files under `data` for such files. If started without a test file,
it will use a default config.

Running:
```bash
cargo run --release --bin trunk-pack
```
or
```bash
./target/release/trunk-pack
```

## 8 Queens
This binary will take in a row and column for the starting config (where the 
first queen will be placed). If started without this data, it will use a default
config.

Running:
```bash
cargo run --release --bin 8queens
```
or
```bash
./target/release/8queens
```
