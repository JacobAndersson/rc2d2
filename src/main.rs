use pleco::{Board, Player, SQ};

mod eval;
mod search;
mod utils;
mod tablebase;

fn main() {
    /*
    for _ in 0..5 {
        utils::play_match();
    }
    */
    utils::play_match(5);
}
