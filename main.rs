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


fn perft(p: &Position, depth:uint) -> u64 {
    if depth == 0 { return 1; }

    let mut result = 0;
    for move in p.gen_moves() {
        let mut p1 = *p;
        // if depth == 1 {
        //     println! ("{}", move);
        // }
        p1.apply_move(&move);
        let sub_result = perft(&p1, depth - 1);
        result += sub_result;
    }
    result
}

pub fn main() {
    tables::init_square_data();

    let initial_fen = "k7/7p/8/8/8/8/6P1/K7 w - - 0 1 ";
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
    let initial_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let p = fen::parse_fen(initial_fen).unwrap();

    println!("{}\n{}", fen::render_fen(&p), p.board);
    for depth in range(1, 7)
    //let depth = 1; 
    {
        let result = perft(&p, depth);
        println!("Depth:{} nodes:{}", depth, result);
    }
    //for move in p.gen_moves() {
    //    let mut p1 = p;
    //    println!("Applying move: {}\n", move);
    //    p1.apply_move(&move);
    //    println!("Result:\n{}\n{}", p1.board, fen::render_fen(&p1));
    //}
}
