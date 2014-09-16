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
pub mod uci;
#[allow(dead_code)]
pub mod squares;
#[cfg(test)]
mod perft_tests;
mod eval;


pub fn main() {
    tables::init_tables();
    uci::UciEngine::new().main_loop();
}
