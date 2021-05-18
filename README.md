# Rust Order Book

`rust-quantcup` is a reimplementation of Quantcup.org's matching engine and order book in Rust. It attempts to make use of Rust language features at the implementation level as well as the project level (using `cargo test` and `cargo bench` for testing and scoring the matching engine).

`engine.rs` provides a baseline naive implementation of the machine engine akin to that of the original C quantcup code. The goal would be to optimize this matching engine.

The baseline quantcup code from Quantcup.org is also provide (though it has been modified to use C++ instead of C).

While there is no official leaderboard or competition like the original QuantCup, a way to participate here would be to fork the repo, edit `engine.rs` and try to get the best bench score possible.

## Modification

`engine.rs` should be the main location for modification and optimization.

## Testing

To test your matching engine simply run:

```
cd source
cargo test
```

This will run the unit tests in `src/engine/test.rs`.

## Scoring

To score your program go into the `source` directory like before and run

```
cargo bench
```

This will benchmark your code using the `score_feed.csv` as the order flow and the matching engine implemented in `engine.rs`.
