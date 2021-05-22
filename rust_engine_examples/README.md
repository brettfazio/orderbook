# Rust Order Book Engine Examples

This directory contains the baseline engine implementation as well as some more performant re-implementations.

## Baseline

`baseline_engine.rs` is the baseline engine in the source code at `src/engine/engine.rs`. Is it a reimplementation of the baseline Quantcup.org matching engine.

## Winning

`winning_engine.rs` is a Rust reimplementation of the winning engine from Quantcup 1. On my computer it has a roughly 50% speed improvement over the baseline engine.

C and Go implementations of the winning engine use linked lists among other 'unsafe' structures harder to represent in Rust so the built in standard collections were just used instead.

### References

[C-code for winning engine](https://gist.github.com/druska/d6ce3f2bac74db08ee9007cdf98106ef)
[Go code for winning engine](https://github.com/rdingwall/go-quantcup)
