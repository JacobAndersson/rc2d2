use pleco::{Board, Player, SQ};

mod eval;
mod search;
mod utils;

fn main() {
    for _ in 0..5 {
        utils::play_match();
    }
}
