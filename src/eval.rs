use pleco::helper::Helper;
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

fn king_safety(board: &Board) -> f32 {
    let hlp = Helper::new();
    let wo = board.occupied_white(); //bitboard with occupency for white
    let bo = board.occupied_black();

    let wksq = board.king_sq(Player::White);
    let white_around = hlp.ring_distance(wksq, 0);
    let white_safety = (white_around & wo).count_bits() as f32;

    let bksq = board.king_sq(Player::Black);
    let black_around = hlp.ring_distance(bksq, 0);
    let black_safety = (black_around & bo).count_bits() as f32;
    return white_safety - black_safety;
}

fn pinned_pieces(board: &Board) -> f32 {
    let wp = board.pieces_pinned(Player::White).count_bits() as f32;
    let bp = board.pieces_pinned(Player::Black).count_bits() as f32;
    return bp - wp; //enemy pinned pieces are good
}

fn attacking_defending(board: &Board) -> (f32, f32) {
    let ao = board.occupied();
    let wo = board.occupied_white();
    let bo = board.occupied_black();

    let mut white_defenders: i8 = 0;
    let mut white_attackers: i8 = 0;
    let mut black_defenders: i8 = 0;
    let mut black_attackers: i8 = 0;

    let pieces = board.get_piece_locations();
    for (sq, pt) in pieces {
        let (player, _piece) = match pt.player_piece() {
            Some(x) => x,
            None => panic!("INVALID PIECETYPE"),
        };
        let relevant_pieces = board.attackers_to(sq, ao);
        if player == Player::White {
            black_attackers += (relevant_pieces & bo).count_bits() as i8;
            white_defenders += (relevant_pieces & wo).count_bits() as i8;
        } else if player == Player::Black {
            white_attackers += (relevant_pieces & wo).count_bits() as i8;
            black_defenders += (relevant_pieces & bo).count_bits() as i8;
        }
    }

    let attacking = white_attackers - black_attackers;
    let defending = white_defenders - black_defenders;
    return (attacking as f32, defending as f32);
}

fn num_big_pieces(board: &Board) -> u8{
    let pieces = board.occupied();
    let white_pawn = board.piece_bb(Player::White, PieceType::P);
    let black_pawn = board.piece_bb(Player::Black, PieceType::P);
    //let white_king = board.king_sq(Player::White).to_bb();
    //let black_king = board.king_sq(Player::Black).to_bb();
    let big_pieces = pieces&!white_pawn&!black_pawn;//&!white_king&!black_king;
    let num = big_pieces.count_bits();
    return num; 
}

pub fn eval(board: &Board) -> f32 {
    if board.checkmate() {
        let turn: f32 = match &board.turn() {
            Player::White => 1.0,
            Player::Black => -1.0,
        };
        let score = -turn * (9999.0 - board.ply() as f32);
        return score;
    }

    if board.stalemate() {
        return 0.0;
    }

    let material = count_material(board);
    let (middle, end) = board.psq().centipawns();
    let king_safety = king_safety(board);
    let pinned = pinned_pieces(board);
    let (attacking, defending) = attacking_defending(board);

    let mut psq = middle;
    if num_big_pieces(board) < 8 {
        psq = end;
    }

    let score = material
        + 0.01 * psq as f32
        + 20.0 * king_safety
        + 40.0 * pinned
        + 50.0 * attacking
        + 50.0 * defending;

    return score;
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

    #[test]
    fn safety() {
        let b1 = Board::from_fen("k7/8/8/8/8/8/1PPP4/2K5 w - - 0 1").unwrap();
        let k1 = king_safety(&b1);
        assert_eq!(k1, 3.0);

        let b2 = Board::from_fen("k7/pp6/8/8/8/8/1PPP4/2K5 w - - 0 1").unwrap();
        let k2 = king_safety(&b2);
        assert_eq!(k2, 1.0);

        let b3 = Board::from_fen("3k4/ppppp3/8/8/8/8/2P5/2K5 w - - 0 1").unwrap();
        let k3 = king_safety(&b3);
        assert_eq!(k3, -2.0);

        let b4 = Board::from_fen("2rk4/ppppp2n/8/P1r2Q2/1P5r/8/2PP4/2K5 w - - 0 1").unwrap();
        let k4 = king_safety(&b4);
        assert_eq!(k4, -2.0);
    }

    #[test]
    fn test_pinned_pieces() {
        let b1 = Board::from_fen("2k5/3p4/8/5B2/8/8/8/2K5 w - - 0 1").unwrap();
        let p1 = pinned_pieces(&b1);
        assert_eq!(p1, 1.0);
        let b2 = Board::from_fen("2k5/3p4/2r5/5B2/8/8/2P5/2K5 w - - 0 1").unwrap();
        let p2 = pinned_pieces(&b2);
        assert_eq!(p2, 0.0);

        let b3 = Board::from_fen("3k4/8/8/3r4/8/8/3P4/3K4 w - - 0 1").unwrap();
        let p3 = pinned_pieces(&b3);
        assert_eq!(p3, -1.0);

        let b4 = Board::from_fen("3k4/8/8/3r4/8/1b6/2PP4/3KN2q w - - 0 1").unwrap();
        let p4 = pinned_pieces(&b4);
        assert_eq!(p4, -3.0);
    }

    #[test]
    fn test_attacking_defending() {
        let b1 = Board::from_fen("1k6/8/8/8/8/8/NNN5/1K6 w - - 0 1").unwrap();
        let (a1, d1) = attacking_defending(&b1);
        assert_eq!(a1, 0.0);
        assert_eq!(d1, 3.0);

        let b2 = Board::from_fen("1k6/nnn5/8/8/5B2/8/NNN5/1K6 w - - 0 1").unwrap();
        let (a2, d2) = attacking_defending(&b2);
        assert_eq!(a2, 1.0);
        assert_eq!(d2, 0.0);

        let b3 = Board::from_fen("2k5/1ppp4/4B3/8/8/8/2PPP3/3K4 w - - 0 1").unwrap();
        let (a3, d3) = attacking_defending(&b3);
        assert_eq!(a3, 0.0);
        assert_eq!(d3, 0.0);
    }
}
