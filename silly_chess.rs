#![crate_name = "silly-chess"]
#![crate_type = "bin"]
#![feature(globs)]
use types::*;

pub mod types;
pub mod bitset;
pub mod fen;
pub mod move_gen;
pub mod utils;
pub mod tables;

pub fn main() {
    tables::init_square_data();
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let sq = Square::new(4,5);
    let p = fen::parse_fen(initial_fen).unwrap();
    let moves = move_gen::gen_queen_moves(p.board, sq, Black);
    println!("{}", moves);
 
}
