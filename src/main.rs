use pleco::{Board};

mod search;
mod eval;

fn main() {
    let mut board = Board::start_pos();
    let mut color = 1;
    for _ in 0..20 {
        println!("{}", board);
        let (score, mv) = search::nega_max(board.shallow_clone(), 4, color, -10000.0, 10000.0, eval::eval);
        color = -color;
        board.apply_move(mv);
        println!("{}, {}", (color as f32) * score, mv);
    }
}
