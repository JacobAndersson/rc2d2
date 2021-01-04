use pleco::{BitMove, Board, Player};
use std::collections::HashMap;

use crate::eval;

#[derive(PartialEq, Debug)]
enum MoveType {
    Exact,
    UpperBound,
    LowerBound,
}

#[derive(Debug)]
pub struct TransitionEntry {
    value: f32,
    mv: BitMove,
    depth: u8,
    move_type: MoveType,
}

fn move_value(mv: &BitMove, board: &Board) -> u32 {
    let mut score: u32 = 0;
    if board.gives_check(*mv) {
        score += 50;
    } else if board.is_capture(*mv) {
        score += 10;
    }
    return score;
}

pub fn nega_max(
    mut board: Board,
    depth: u8,
    color: i8,
    mut alpha: f32,
    mut beta: f32,
    transition_table: &mut HashMap<u64, TransitionEntry>,
    root: bool,
    evaluator: fn(&Board) -> f32,
    do_null: bool,
) -> (f32, BitMove) {
    let alpha_original = alpha;
    let hash = board.zobrist();
    let mut skip_cache = false;
    let R = 2; //search depth reduction in null move pruning
    let mut moves = board.generate_moves().vec();

    if root {
        let mut temp_moves: Vec<BitMove> = Vec::new();
        for mv in &moves {
            board.apply_move(*mv);
            if board.checkmate() || !board.stalemate() {
                temp_moves.push(*mv);
            }
            board.undo_move();
        }
        if temp_moves.len() <= (&moves).len() && temp_moves.len() > 0 {
            moves = temp_moves;
            skip_cache = true;
        }
    }

    match transition_table.get(&hash) {
        Some(entry) => {
            if !root && !skip_cache && entry.depth >= depth {
                //use transition table value
                if entry.move_type == MoveType::Exact {
                    return (entry.value, entry.mv);
                } else if entry.move_type == MoveType::UpperBound {
                    beta = beta.min(entry.value);
                } else if entry.move_type == MoveType::LowerBound {
                    alpha = alpha.max(entry.value);
                }

                if alpha >= beta {
                    return (entry.value, entry.mv);
                }
            }
        }
        None => {}
    };

    moves.sort_by(|a, b| move_value(b, &board).cmp(&move_value(a, &board)));
    if depth == 0 || board.checkmate() || moves.is_empty() {
        return (
            quiesce(board, color, alpha, beta, 10, evaluator),
            BitMove::null(),
        );
    }

    //null move pruning
    //The apply_null_move and undo_null_move are unsafe operations
    unsafe {
        let curr_pl = match color {
            1 => Player::White,
            -1 => Player::Black,
            _ => panic!("In valid color"),
        };
        if do_null
            && !board.in_check()
            && board.ply() > 0
            && board.non_pawn_material(curr_pl) > 0
            && depth > 3
        {
            board.apply_null_move();
            let (mut score, _) = nega_max(
                board.shallow_clone(),
                depth - 1 - R,
                -color,
                -beta,
                -beta + 1.0,
                transition_table,
                false,
                evaluator,
                false,
            );
            score = -score;
            board.undo_null_move();

            if score > beta {
                return (beta, BitMove::null());
            }
        }
    }

    let mut best_score: f32 = -9999.0;
    let mut best_move: BitMove = BitMove::null();

    for mv in moves {
        board.apply_move(mv);
        let (mut score, _) = nega_max(
            board.shallow_clone(),
            depth - 1,
            -color,
            -beta,
            -alpha,
            transition_table,
            false,
            evaluator,
            true,
        );
        score = -score;

        board.undo_move();
        if score > best_score {
            best_score = score;
            best_move = mv;
        }

        if score > alpha {
            alpha = score;
        }

        if alpha > beta {
            break;
        }
    }

    let tp: MoveType;
    if best_score <= alpha_original {
        tp = MoveType::UpperBound;
    } else if best_score >= beta {
        tp = MoveType::LowerBound;
    } else {
        tp = MoveType::Exact;
    }

    let new_entry = TransitionEntry {
        value: best_score,
        mv: best_move,
        depth: depth,
        move_type: tp,
    };

    transition_table.insert(hash, new_entry);
    return (best_score, best_move);
}

fn quiesce(
    mut board: Board,
    color: i8,
    mut alpha: f32,
    beta: f32,
    depth: u8,
    evaluator: fn(&Board) -> f32,
) -> f32 {
    let standpat = (color as f32) * evaluator(&board);
    if depth == 0 {
        return standpat;
    }
    if standpat >= beta {
        return beta;
    }

    if alpha < standpat {
        alpha = standpat;
    }

    let moves = board.generate_moves();
    for mv in moves {
        if !board.is_capture(mv)
            || standpat + eval::piece_values(board.piece_last_captured()) + 200.0 < alpha
        {
            continue;
        }

        board.apply_move(mv);
        let score = -quiesce(
            board.shallow_clone(),
            -color,
            -beta,
            -alpha,
            depth - 1,
            evaluator,
        );
        board.undo_move();
        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use pleco::{Board, Player};

    #[test]
    fn take_queen_white() {
        let fen = "rnb1kbnr/pppp1ppp/8/4p1q1/4P1Q1/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3";
        let board = Board::from_fen(fen).unwrap();
        let mut tt: HashMap<u64, TransitionEntry> = HashMap::new();

        let (_, mv) = nega_max(
            board,
            4,
            1,
            -9999.0,
            9999.0,
            &mut tt,
            true,
            eval::eval,
            true,
        );
        assert_eq!(
            mv.stringify(),
            "g4g5",
            "Did not play g4g5, played: {}, fen: {}",
            mv.stringify(),
            fen
        );
    }

    #[test]
    fn take_queen_black() {
        let board =
            Board::from_fen("rnb1kbnr/pppp1ppp/8/4p1q1/4P1Q1/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3")
                .unwrap();
        let mut tt: HashMap<u64, TransitionEntry> = HashMap::new();
        let (score, mv) = nega_max(
            board,
            4,
            -1,
            -9999.0,
            9999.0,
            &mut tt,
            true,
            eval::eval,
            true,
        );
        assert_ne!(mv.stringify(), "g4g5");
        assert_ne!(score, 0.0);
    }

    #[test]
    fn mate_in_one_1() {
        let mut board = Board::from_fen("k7/5R2/6R1/8/8/8/4K3/8 w - - 0 1").unwrap();
        let mut tt: HashMap<u64, TransitionEntry> = HashMap::new();

        for depth in 1..4 {
            let (_, mv) = nega_max(
                board.shallow_clone(),
                depth,
                1,
                -9999.0,
                9999.0,
                &mut tt,
                true,
                eval::eval,
                true,
            );
            board.apply_move(mv);
            assert!(
                board.checkmate(),
                "Played: {}, \n board after move: \n {}",
                mv.stringify(),
                board
            );
            board.undo_move();
        }
    }

    #[test]
    fn mate_in_one_2() {
        let fen = "1k6/8/8/8/8/3n4/6PR/6RK b Q - 0 1";
        let mut board = Board::from_fen(fen).unwrap();
        let mut tt: HashMap<u64, TransitionEntry> = HashMap::new();

        let color = -1;
        for depth in 1..4 {
            let (_, mv) = nega_max(
                board.shallow_clone(),
                depth,
                color,
                -9999.0,
                9999.0,
                &mut tt,
                true,
                eval::eval,
                true,
            );
            board.apply_move(mv);
            assert!(
                board.checkmate(),
                "Played: {}, depth: {}",
                mv.stringify(),
                &depth
            );
            board.undo_move()
        }
    }

    #[test]
    fn mate_in_two_1() {
        let fen = "k7/4R3/8/8/8/4R3/8/3K4 w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        let color = 1;
        let board = utils::play_x_moves(board, 3, 4, color, eval::eval);
        assert!(board.checkmate());
    }

    #[test]
    fn mate_in_two_2() {
        let board = Board::from_fen("k7/4R3/2p5/p7/1p6/2P1R2P/1P4P1/3K4 w - - 0 1").unwrap();
        let color = 1;
        let board = utils::play_x_moves(board, 3, 4, color, eval::eval);
        assert!(board.checkmate());
    }

    #[test]
    fn mate_in_two_3() {
        let board = Board::from_fen("r6k/6pp/p5r1/7R/5q2/3P3K/PPP1N1P1/2R1Q3 b - - 0 1").unwrap();
        let color = -1;
        let board = utils::play_x_moves(board, 3, 4, color, eval::eval);
        assert!(board.checkmate());
    }
}
