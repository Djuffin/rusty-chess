//implementation of Zobrist Hashing
use types::*;
use tables::get_random_number;

pub fn calc_position_hash(position: &Position) -> u64 {
    let mut result:u64 = 0;
    let board = &position.board;
    for &color in [White, Black].iter() {
        for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
            let mut squares = board.get_pieces(kind, color);
            for sq in squares {
                result ^= piece_hash(sq, Piece(kind, color));
            }
        }
    } 
    result ^= castling_hash(position.white_castling, position.black_castling);
    result ^= en_passant_hash(position.en_passant);
    result ^= next_to_move_hash(position.next_to_move);
    result   

}

#[inline]
fn piece_hash(sq:Square, piece:Piece) -> u64 {
    //this function returns a random number for each (square, piece) combination
    //it returns first (2 * 6 * 64) 768 random numbers from #0 to #767   
    let index = (piece.kind() as uint << 7) + 
                (piece.color() as uint << 6) + 
                (sq.file_and_rank() as uint);
    get_random_number(index)
}

#[inline]
fn castling_hash(white:CastlingRight, black:CastlingRight) -> u64 {
    //this function returns random number for each possible black/white castling right combination
    //it returns 16 random numbers from #800 to #816
    let index = (white as uint) + (black as uint << 2);
    get_random_number(index + 800)
}

#[inline]
fn en_passant_hash(sq:Option<Square>) -> u64 {
    //this function returns a random number for each en pasant file
    //it returns random numbers from #820 to #827
    match sq {
        Some(s) => get_random_number(s.file() as uint + 820),
        None => 0
    }
}

#[inline]
fn next_to_move_hash(color: Color) -> u64 {
    //this function returns a random number if color == Black
    //it returns random numbers #830
    match color {
        Black => get_random_number(830),
        White => 0
    }
}



#[cfg(test)]
mod tests {
use hash::*;
use fen::{parse_fen, render_fen};
use types::*;
use std::str::{StrSlice};

#[test]
fn one_piece_positions() {
    ::tables::init_tables();
    let mut hashes:Vec<u64> = Vec::new();
    let empty_position = Position {
        board: Board::empty(),
        full_moves : 0,
        next_to_move : White,
        white_castling : BothCastling,
        black_castling : BothCastling,
        en_passant : None,
        half_moves_since_action : 0
    };
    for &w_castling in [BothCastling, QueenCastling, KingCastling, NoCastling].iter() {
        for &b_castling in [BothCastling, QueenCastling, KingCastling, NoCastling].iter() {
            for &color in [White, Black].iter() {
                for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
                    for sq in range(0u8, 64).map(|n| Square(n)) {
                        let mut p = empty_position;
                        p.white_castling = w_castling;
                        p.black_castling = b_castling;
                        p.board.set_piece(sq, Piece(kind, color));
                        hashes.push(calc_position_hash(&p));
                        p.next_to_move = Black;
                        hashes.push(calc_position_hash(&p));
                    }
                }
            }
       }
    }

    //checking that all hashes are unique
    hashes.sort();
    for i in range(1, hashes.len()) {
        assert!(hashes[i] != hashes[i - 1]);
    }
}

#[test]
fn position_tree_hashes() {
    ::tables::init_tables();
    let mut positions:Vec<String> = Vec::new();
    let initial_position = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    perft(&initial_position, 3, &mut positions);

    //positions with equal hashes should be queal
    positions.sort();
    for i in range(1, positions.len()) {
        if positions[i].as_slice().slice_to(16) == positions[i - 1].as_slice().slice_to(16) {
            assert_eq!(positions[i], positions[i - 1]);
        }
    }
}


fn perft(p: &Position, depth:uint, positions: &mut Vec<String>){
    let hash = calc_position_hash(p);
    positions.push(format!("{:016x} - {}", hash, render_fen(p)));
    if depth > 0 { 
        for mv in p.gen_moves() {
            let mut p1 = *p;
            p1.apply_move(&mv);
            //these 2 fields are not part of hash
            p1.half_moves_since_action = 0;
            p1.full_moves = 1;
            perft(&p1, depth - 1, positions);
        }
    }
}



}