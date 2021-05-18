mod types;
mod engine;
mod feed;
mod score;

use crate::score::score::playback;
use crate::feed::feed::get_raw_feed;

fn main() {
    let mut flow = get_raw_feed();

    playback(&mut flow);
}
