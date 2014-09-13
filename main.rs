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
#[cfg(test)]
mod perft_tests;


fn perft(p: &Position, depth:uint) -> u64 {
    if depth == 0 { return 1; }
    let mut iter = p.gen_moves();
    if depth == 1 {
        iter.count() as u64
    } else {
        let mut result = 0;
        for move in iter {
            let mut p1 = *p;
            p1.apply_move(&move);
            result += perft(&p1, depth - 1);
        }
        result
    }
}

pub fn main() {
    tables::init_square_data();

    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
    let p = fen::parse_fen(initial_fen).unwrap();

    println!("{}\n{}", fen::render_fen(&p), p.board);
    for depth in range(1, 7)
    {
        let result = perft(&p, depth);
        println!("Depth:{} nodes:{}", depth, result);
    }

    // let initial_fen = "4q3/3P2p1/5N1N/4p3/1Pp1p3/2K3P1/P7/8 b - b3 0 1";
    // let p = fen::parse_fen(initial_fen).unwrap();
    // for move in p.gen_moves() {
    //     let mut p1 = p;
    //     println!("Applying move: {}\n", move);
    //     p1.apply_move(&move);
    //     println!("Result:\n{}\n{}", p1.board, fen::render_fen(&p1));
    // }
}
