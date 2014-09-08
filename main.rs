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
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1";
    let p = fen::parse_fen(initial_fen).unwrap();
    let moves = move_gen::MovesIterator::new(p);
    let moves_vec:Vec<Move> = FromIterator::from_iter(moves);
    println!("{}", moves_vec);
}
