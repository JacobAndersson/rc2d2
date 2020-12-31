use pleco::Board;
use std::collections::HashMap;

mod eval;
mod search;
mod utils;

fn main() {
    for _ in 0..5 {
        utils::play_match();
    }
}
