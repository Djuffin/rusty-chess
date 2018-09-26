#![crate_name = "rchess"]
#![crate_type = "bin"]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
extern crate rand;

mod squares;

mod bitset;
mod types;
mod fen;
mod move_gen;
mod utils;
mod tables;
mod eval;
mod search;
mod hash;
pub mod uci;
#[cfg(test)]
mod perft_tests;
#[cfg(test)]
mod search_test;

fn main() {
  tables::init_tables();
  uci::UciEngine::new().std_main_loop();
}
