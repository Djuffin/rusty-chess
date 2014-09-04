#![crate_name = "silly-chess"]
#![crate_type = "bin"]
#![feature(globs)]
use types::*;

pub mod types;
pub mod bitset;
pub mod fen;
pub mod move_gen;
pub mod utils;

pub fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let position = fen::parse_fen(fen).unwrap();

    println!("White pawns: {0}", position.board.get_pieces(Pawn, White));
    println!("Black knights: {0}", position.board.get_pieces(Knight, Black));
   
  
}
