#![crate_name = "silly-chess"]
#![crate_type = "bin"]
#![feature(globs)]
use play::*;

pub mod play;
pub mod bitset;
pub mod fen;

pub fn main() {
    
    let mut x = Board::empty();
    let pieces = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
    for i in range(0, 8) {
        x.set_piece(Square::new(i as u8, 1), Piece(Pawn, White));
        x.set_piece(Square::new(i as u8, 0), Piece(pieces[i], White));
        x.set_piece(Square::new(i as u8, 6), Piece(Pawn, Black));
        x.set_piece(Square::new(i as u8, 7), Piece(pieces[i], Black));
    } 
    println!("{0}", x);

    let p = Position {
        board : x,
        en_passant : Some (Square::new(4,2)),
        half_moves_since_action : 0,
        full_moves : 10 ,
        next_to_move : White,
        white_castling : BothCastling,
        black_castling : BothCastling
    };
    println!("{0}", fen::render_fen(&p));

    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 134 231";
    match fen::parse_fen(fen) {
        Ok(p) => println!("{0}", fen::render_fen(&p)),
        Err(s) => println!("Error: {0}", s)        
    }
}
