#![crate_name = "rchess"]
#![crate_type = "bin"]
#![feature(globs)]
use types::*;

pub mod types;
pub mod bitset;
pub mod fen;
pub mod move_gen;
pub mod utils;
pub mod tables;
#[allow(dead_code)]
pub mod squares;

pub fn main() {
    tables::init_square_data();
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let p = fen::parse_fen(initial_fen).unwrap();
    for move in p.gen_moves() {
        let mut p1 = p;
        println!("Applying move: {}\n", move);
        p1.apply_move(&move);
        println!("Result:\n{}\n{}", p1.board, fen::render_fen(&p1));
    }
}
