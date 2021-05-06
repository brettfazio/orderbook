# Rust Order Book

`rust-quantcup` is a reimplementation of Quantcup.org's matching engine and order book in Rust. It attempts to make use of Rust language features at the implementation level as well as the project level (using `cargo test` and `cargo bench` for testing and scoring the matching engine).

`engine.rs` provides a baseline naive implementation of the machine engine akin to that of the original C quantcup code. The goal would be to optimize this matching engine. 

The baseline quantcup code from Quantcup.org is also provide (though it has been modified to use C++ instead of C).
