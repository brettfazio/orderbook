mod types;
mod engine;
mod feed;
mod score;

use crate::score::score::get_score;

fn main() {
    let feed = get_score();
}
