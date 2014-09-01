use std::fmt;
use std::string::String;
use std::iter::range_step;
use play::*;

//Forsyth–Edwards Notation
//Serialization of position into something like this:
//rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
pub fn render_fen(p:&Position) -> String {
    //board
    let mut result =  String::with_capacity(80);
    for i in range_step(7i, -1, -1) {
        render_rank(&p.board, i as u8, &mut result);
        if i > 0 { result.push_char('/'); }
    }

    //next to move
    result.push_char(' ');
    result.push_char(match p.next_to_move {
        White => 'w', Black => 'b'
    });

    //castling rights
    result.push_char(' ');
    if p.white_castling == NoCastling && p.black_castling == NoCastling {
        result.push_char('-');
    } else {
        render_castling(White, p.white_castling, &mut result);
        render_castling(Black, p.black_castling, &mut result);
    }

    //En passant square
    result.push_char(' ');
    match p.en_passant {
        Some(s) => result.push_str(s.to_string().as_slice()),
        None => result.push_char('-')
    };

    //Halfmove clock
    result.push_char(' ');
    result.push_str(p.half_moves_since_action.to_string().as_slice());

    //Fullmove number
    result.push_char(' ');
    result.push_str(p.full_moves.to_string().as_slice());

    result
}

fn render_castling(color:Color, cr:CastlingRight, result: &mut String) {
    let s = match (color, cr) {
        (White, QueenCastling) => "K", 
        (White, KingCastling)  => "Q",
        (White, BothCastling)  => "KQ",
        (Black, QueenCastling) => "k", 
        (Black, KingCastling)  => "q",
        (Black, BothCastling)  => "kq",
        (_, NoCastling) => ""
    };
    result.push_str(s)
}

fn render_rank(b:&Board, rank:u8, result: &mut String) {
    let mut skip_number = 0u;
    for i in range(0, 8) {
        match b.get_piece(Square::new(i, rank)) {
            Some(p) => {
                if skip_number != 0 {
                    result.push_str(skip_number.to_string().as_slice());
                    skip_number = 0;    
                }
                result.push_str(p.to_string().as_slice());
            }
            None => skip_number += 1
        }
    }
    if skip_number != 0 {
        result.push_str(skip_number.to_string().as_slice());
    }
}

