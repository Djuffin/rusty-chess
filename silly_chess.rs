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
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let p = fen::parse_fen(initial_fen).unwrap();
    let moves = move_gen::gen_rook_moves(p.board, Square::new(3, 0), White);
    println!("{}", moves);


}
