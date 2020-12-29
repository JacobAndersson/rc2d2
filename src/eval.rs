use pleco::{Board, PieceType, Player};

pub fn piece_values(piece: PieceType) -> f32 {
    return match piece {
        PieceType::P => 100.0,
        PieceType::N => 280.0,
        PieceType::B => 320.0,
        PieceType::R => 479.0,
        PieceType::Q => 929.0,
        PieceType::K => 60_000.0,
        PieceType::None => 0.0,
        PieceType::All => 0.0,
    };
}

fn count_piece_material(board: &Board, player: Player, piece: PieceType) -> f32 {
    return piece_values(piece) * board.count_piece(player, piece) as f32;
}

fn count_material(board: &Board) -> f32 {
    let wp = count_piece_material(board, Player::White, PieceType::P);
    let wn = count_piece_material(board, Player::White, PieceType::N);
    let wb = count_piece_material(board, Player::White, PieceType::B);
    let wr = count_piece_material(board, Player::White, PieceType::R);
    let wq = count_piece_material(board, Player::White, PieceType::Q);
    let wk = count_piece_material(board, Player::White, PieceType::K);
    let white_material = wp + wn + wb + wr + wq + wk;

    let bp = count_piece_material(board, Player::Black, PieceType::P);
    let bn = count_piece_material(board, Player::Black, PieceType::N);
    let bb = count_piece_material(board, Player::Black, PieceType::B);
    let br = count_piece_material(board, Player::Black, PieceType::R);
    let bq = count_piece_material(board, Player::Black, PieceType::Q);
    let bk = count_piece_material(board, Player::Black, PieceType::K);
    let black_material = bp + bn + bb + br + bq + bk;

    let material = white_material - black_material;
    return material;
}

pub fn eval(board: &Board) -> f32 {
    if board.checkmate() {
        let turn: f32 = match &board.turn() {
            Player::White => 1.0,
            Player::Black => -1.0,
        };
        let score = -turn * (9999.0 - (100 * board.ply()) as f32);
        return score;
    }

    if board.stalemate() {
        return 0.0;
    }

    let material = count_material(board);
    let (middle, _) = board.psq().centipawns();
    let score = material + 0.01 * middle as f32;

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_pos() {
        assert_eq!(count_material(&Board::start_pos()), 0.0);
    }

    #[test]
    fn opening() {
        let board =
            Board::from_fen("rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 3")
                .unwrap();
        assert_eq!(count_material(&board), 0.0);
    }

    #[test]
    fn white_queen() {
        let board =
            Board::from_fen("rnb1kbnr/ppp2ppp/4p3/3p2Q1/3P4/4P3/PPP2PPP/RNB1KBNR b KQkq - 0 4")
                .unwrap();
        assert_eq!(count_material(&board), 929.0);
    }

    #[test]
    fn black_knight() {
        let board =
            Board::from_fen("rnb1kbnr/ppp2ppp/4p3/3p4/3P3Q/4Pq2/PPP2PPP/RNB1KB1R w KQkq - 0 6")
                .unwrap();
        assert_eq!(count_material(&board), -280.0);
    }

    #[test]
    fn multi() {
        let board1 =
            Board::from_fen("rnb1kbnr/ppp2ppQ/4p3/3p4/3P4/4Pq2/PPP2PPP/RNB1KB1R b KQkq - 0 6")
                .unwrap();
        assert_eq!(count_material(&board1), -180.0);

        let board2 =
            Board::from_fen("rnb1kbn1/ppp2ppr/4p3/3p4/3P4/4Pq2/PPP2PPP/RNB1KB1R w KQq - 0 7")
                .unwrap();
        assert_eq!(count_material(&board2), -1109.0);
    }
}
