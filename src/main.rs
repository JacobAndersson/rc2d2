use pleco::Board;

mod eval;
mod search;

fn main() {
    //let mut board = Board::from_fen("rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 3").unwrap();
    let mut board = Board::start_pos();
    let mut color = 1;
    println!("STARTED GAME");
    for _ in 0..20 {
        let (score, mv) = search::nega_max(
            board.shallow_clone(),
            4,
            color,
            -10000.0,
            10000.0,
            eval::eval,
        );
        color = -color;
        board.apply_move(mv);
        println!("{}, {}", score, mv);
        println!("{}", board);
    }
}
