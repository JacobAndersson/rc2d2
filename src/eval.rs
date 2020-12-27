use pleco::{Board, Player, PieceType};

fn count_material(board: &Board) -> f32 {
    let wp = board.count_piece(Player::White, PieceType::P) as f32;
    let wn = board.count_piece(Player::White, PieceType::N) as f32;
    let wb = board.count_piece(Player::White, PieceType::B) as f32;
    let wr = board.count_piece(Player::White, PieceType::R) as f32;
    let wq = board.count_piece(Player::White, PieceType::Q) as f32;
    let wk = board.count_piece(Player::White, PieceType::K) as f32;
    let white_material = 100.0*wp + 280.0*wn + 320.0*wb + 479.0*wr + 929.0*wq + 60_000.0*wk;

    let bp = board.count_piece(Player::Black, PieceType::P) as f32;
    let bn = board.count_piece(Player::Black, PieceType::N) as f32;
    let bb = board.count_piece(Player::Black, PieceType::B) as f32;
    let br = board.count_piece(Player::Black, PieceType::R) as f32;
    let bq = board.count_piece(Player::Black, PieceType::Q) as f32;
    let bk = board.count_piece(Player::Black, PieceType::K) as f32;
    let black_material = 100.0*bp + 280.0*bn + 320.0*bb + 479.0*br + 929.0*bq + 60_000.0*bk;

    let material = white_material - black_material;
    return material;
}

pub fn eval(board: Board) -> f32 {
    if board.checkmate(){
        let turn: f32 = match &board.turn() {
            Player::White => 1.0,
            Player::Black => -1.0
        };
        let score = -turn * (9999.0 - (100 * board.ply()) as f32);
        return score;
    }

    if board.stalemate() {
        return 0.0;
    }

    let material = count_material(&board);
    let (middle, _) = board.psq().centipawns();
    let score = material + 0.05 * middle as f32;
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_pos(){
        assert_eq!(count_material(&Board::start_pos()), 0.0);
    }

    #[test]
    fn opening(){
        let board = Board::from_fen("rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 3").unwrap();
        assert_eq!(count_material(&board), 0.0);
    }

    #[test]
    fn white_queen(){
        let board = Board::from_fen("rnb1kbnr/ppp2ppp/4p3/3p2Q1/3P4/4P3/PPP2PPP/RNB1KBNR b KQkq - 0 4").unwrap();
        assert_eq!(count_material(&board), 929.0);
    }

    #[test]
    fn black_knight(){
        let board = Board::from_fen("rnb1kbnr/ppp2ppp/4p3/3p4/3P3Q/4Pq2/PPP2PPP/RNB1KB1R w KQkq - 0 6").unwrap();
        assert_eq!(count_material(&board), -280.0);
    }

    
    #[test]
    fn multi(){
        let board1 = Board::from_fen("rnb1kbnr/ppp2ppQ/4p3/3p4/3P4/4Pq2/PPP2PPP/RNB1KB1R b KQkq - 0 6").unwrap();
        assert_eq!(count_material(&board1), -180.0);

        let board2 = Board::from_fen("rnb1kbn1/ppp2ppr/4p3/3p4/3P4/4Pq2/PPP2PPP/RNB1KB1R w KQq - 0 7").unwrap();
        assert_eq!(count_material(&board2), -1109.0);
    }
}
