use crate::{eval, search};
use pleco::{Board};
use std::collections::HashMap;

pub fn play_x_moves(
    mut board: Board,
    num_moves: u8,
    depth: u8,
    start_color: i8,
    evaluator: fn(&Board) -> f32,
) -> Board {
    let mut color: i8 = start_color;
    for i in 0..num_moves {
        let mut tt: HashMap<u64, search::TransitionEntry> = HashMap::new();

        let (score, mv) = search::nega_max(
            board.shallow_clone(),
            depth,
            color,
            -9999.0,
            9999.0,
            &mut tt,
            true,
            evaluator,
        );
        board.apply_move(mv);
        color = -color;
    }
    return board;
}

pub fn play_match() {
    let mut board = Board::start_pos();
    let mut color = 1;
    let mut count = 0;
    let mut black_tt: HashMap<u64, search::TransitionEntry> = HashMap::new();
    let mut white_tt: HashMap<u64, search::TransitionEntry> = HashMap::new();
    while !board.generate_moves().is_empty() {
        count += 1;
        let transition_table = match color {
            1 => &mut white_tt,
            -1 => &mut black_tt,
            _ => &mut white_tt,
        };
        let (score, mv) = search::nega_max(
            board.shallow_clone(),
            4,
            color,
            -10000.0,
            10000.0,
            transition_table,
            true,
            eval::eval,
        );
        color = -color;
        let size = transition_table.keys().len();
        board.apply_move(mv);
        println!(
            "{}. {}, {}, transition table: {} \n{}\r",
            count, mv, score, size, board
        );
    }
}
