use crate::{eval, search};
use pleco::{Board, Player};
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
            true
        );
        board.apply_move(mv);
        color = -color;
    }
    return board;
}

pub fn play_match(depth: u8) {
    let mut board = Board::start_pos();
    let mut color = 1;
    let mut count = 0;
    let mut black_tt: HashMap<u64, search::TransitionEntry> = HashMap::new();
    let mut white_tt: HashMap<u64, search::TransitionEntry> = HashMap::new();
    while !board.generate_moves().is_empty() {
        println!("{}", board);
        count += 1;
        let transition_table = match color {
            1 => &mut white_tt,
            -1 => &mut black_tt,
            _ => &mut white_tt,
        };
        let (score, mv) = search::nega_max(
            board.shallow_clone(),
            depth,
            color,
            -10000.0,
            10000.0,
            transition_table,
            true,
            eval::eval,
            true
        );
        color = -color;
        let size = transition_table.keys().len();
        board.apply_move(mv);
    }
}

pub fn find_best_move(uci_moves: &str, depth: u8) -> String {
    let moves: Vec<&str> = uci_moves.split(" ").collect();
    let mut board = Board::start_pos();
    for mv in moves {
        board.apply_uci_move(mv);
    }

    let color = match board.turn() {
        Player::White => 1,
        Player::Black => -1
    };
    let mut tt: HashMap<u64, search::TransitionEntry> = HashMap::new();

    let (score, mv) = search::nega_max(
        board.shallow_clone(),
        depth,
        color,
        -9999.0,
        9999.0,
        &mut tt,
        true,
        eval::eval,
        true
    );
    println!("move: {}, score: {}", mv.stringify(), score);
    return mv.stringify();
} 
