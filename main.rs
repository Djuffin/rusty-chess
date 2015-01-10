#![crate_name = "rchess"]
#![crate_type = "bin"]
#![feature(int_uint)] 
#![allow(unstable)]
#![feature(box_syntax)]

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
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




fn main() {
  tables::init_tables();
  uci::UciEngine::new().main_loop();
}
