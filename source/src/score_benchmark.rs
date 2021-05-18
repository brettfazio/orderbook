use criterion::{criterion_group, criterion_main, Criterion};

mod types;
mod engine;
mod feed;
mod score;

use crate::score::score::playback;
use crate::feed::feed::get_raw_feed;

fn criterion_benchmark(c: &mut Criterion) {
    let mut flow = get_raw_feed();

    c.bench_function("score", |b| b.iter(|| playback(&mut flow)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);