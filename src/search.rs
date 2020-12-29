use pleco::{BitMove, Board};

#[path = "./eval.rs"]
mod eval;

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
    beta: f32,
    evaluator: fn(&Board) -> f32,
) -> (f32, BitMove) {
    let mut moves = board.generate_moves();
    moves.sort_by(|a, b| move_value(a, &board).cmp(&move_value(b, &board)));

    if depth == 0 || board.checkmate() || moves.is_empty() {
        return (
            quiesce(board, color, alpha, beta, 5, evaluator),
            BitMove::null(),
        );
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
            evaluator,
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
        if !board.is_capture(mv) || standpat + eval::piece_values(board.piece_last_captured()) + 200.0 < alpha {
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
    use pleco::Board;

    #[test]
    fn take_queen_white() {
        let fen = "rnb1kbnr/pppp1ppp/8/4p1q1/4P1Q1/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3";
        let board = Board::from_fen(fen).unwrap();
        let (_, mv) = nega_max(board, 4, 1, -9999.0, 9999.0, eval::eval);
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
        let (score, mv) = nega_max(board, 4, -1, -9999.0, 9999.0, eval::eval);
        assert_ne!(mv.stringify(), "g4g5");
        assert_ne!(score, 0.0);
    }

    #[test]
    fn mate_in_one_1() {
        let mut board = Board::from_fen("k7/5R2/6R1/8/8/8/4K3/8 w - - 0 1").unwrap();
        for depth in 1..4 {
            let (_, mv) = nega_max(board.shallow_clone(), depth, 1, -9999.0, 9999.0, eval::eval);
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
        let color = -1;
        for depth in 1..4 {
            let (_, mv) = nega_max(
                board.shallow_clone(),
                depth,
                color,
                -9999.0,
                9999.0,
                eval::eval,
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
        let mut board = Board::from_fen(fen).unwrap();
        let mut color = 1;
        for _i in 0..3 {
            let (_, mv) = nega_max(board.shallow_clone(), 4, color, -9999.0, 9999.0, eval::eval);
            color = -color;
            board.apply_move(mv);
        }
        assert!(board.checkmate());
    }

    #[test]
    fn mate_in_two_2() {
        let mut board = Board::from_fen("k7/4R3/2p5/p7/1p6/2P1R2P/1P4P1/3K4 w - - 0 1").unwrap();
        let mut color = 1;
        for _i in 0..3 {
            let (_, mv) = nega_max(board.shallow_clone(), 4, color, -9999.0, 9999.0, eval::eval);
            color = -color;
            board.apply_move(mv);
        }
        assert!(board.checkmate());
    }

    #[test]
    fn mate_in_two_3() {
        let mut board =
            Board::from_fen("r6k/6pp/p5r1/7R/5q2/3P3K/PPP1N1P1/2R1Q3 b - - 0 1").unwrap();
        let mut color = -1;
        for _i in 0..3 {
            let (_, mv) = nega_max(board.shallow_clone(), 4, color, -9999.0, 9999.0, eval::eval);
            color = -color;
            board.apply_move(mv);
        }

        assert!(board.checkmate());
    }
}
