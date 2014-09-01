use std::str::{Chars, StrSlice};
use std::string::String;
use std::iter::range_step;
use play::*;

//Forsyth–Edwards Notation
//Serialization of position into something like this:
//rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
pub fn render_fen(p:&Position) -> String {
    //rendering board rank by rank
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

    //en passant square
    result.push_char(' ');
    match p.en_passant {
        Some(s) => result.push_str(s.to_string().as_slice()),
        None => result.push_char('-')
    };

    //halfmove clock
    result.push_char(' ');
    result.push_str(p.half_moves_since_action.to_string().as_slice());

    //fullmove number
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

pub fn parse_fen(input:&str) -> Result<Position, String> {
    let mut iter = input.chars();

    //read board
    let board = try!(parse_board(&mut iter));
    try!(expect_char(&mut iter, ' ', "Space is expected after board description".to_string()));

    //read next to move
    let next_to_move = match iter.next() {
        Some('w') => White,
        Some('b') => Black,
        c => { return Err(format!("'b' or 'w' is expected for next to move instead of {0}", c)) }
    };
    try!(expect_char(&mut iter, ' ', "Space is expected after next to move color".to_string()));

    //read castlings
    let (white_castling, black_castling) = try!(parse_castlings(&mut iter));

    //read en passant
    let en_passant = try!(parse_en_passant(&mut iter));
    try!(expect_char(&mut iter, ' ', "Space is expected after the en passant value".to_string()));

    //halfmove clock
    let halfmove = try!(parse_int(&mut iter));

    //fullmove number
    let full_moves = try!(parse_int(&mut iter));

    Ok (Position {
        board: board,
        en_passant : en_passant,
        half_moves_since_action : halfmove,
        full_moves : full_moves,
        next_to_move : next_to_move,
        white_castling : white_castling,
        black_castling : black_castling        
    })
}

fn parse_int(iter: &mut Chars) -> Result<u16 , String> {
    let mut result = 0u16;
    loop {
        match iter.next() {
            Some(c) if c >= '0' && c <= '9' => {
                result *= 10;
                result += ((c as u32) - ('0' as u32)) as u16;
            }
            None | Some(' ') => break,
            c => return Err(format!("Expected integer move value instead of {0}", c))
        }
    }
    Ok (result)
}

fn parse_en_passant(iter: &mut Chars) -> Result< Option<Square> , String> {
    let file = match iter.next() {
        Some(c) if c >= 'a' && c <= 'h' => (c as u32) - ('a' as u32),
        Some('-') => return Ok(None),
        c => return Err(format!("Unexpected en passant value: {0}", c))
    };
    let rank = match iter.next() {
        Some(c) if c >= '1' && c <= '8' => (c as u32) - ('1' as u32),
        c => return Err(format!("Unexpected en passant value: {0}", c))
    };
    Ok(Some(Square::new(file as u8, rank as u8))) 
}

fn parse_castlings(iter: &mut Chars) -> Result<(CastlingRight, CastlingRight), String> {
    let (mut white_king, mut white_queen, mut black_king, mut black_queen) 
        = (false, false, false, false);
    let mut n = 0i;
    loop {
        match iter.next() {
            Some('k') => black_king = true,
            Some('K') => white_king = true,
            Some('q') => black_queen = true,
            Some('Q') => white_queen = true,
            Some('-') if !(white_king && white_queen && black_queen && black_king) => 
                { return Ok ((NoCastling, NoCastling)) }
            Some(' ') => break,
            c => { return Err(format!("Unexpected castling configuration {0}", c)) } 
        };
        n += 1;
        if n > 4 {
            return Err("Castling configuration is too long".to_string());  
        }
    }
    let bools_to_castling = |king:bool, queen:bool| {
        match (king, queen) {
            (true, false)  => KingCastling,
            (false, true)  => QueenCastling,
            (true, true)   => BothCastling,
            (false, false) => NoCastling
        } 
    };

    let white_castling = bools_to_castling(white_king, white_queen);
    let black_castling = bools_to_castling(black_king, black_queen);
    Ok ((white_castling, black_castling))
}

fn parse_board(iter: &mut Chars) -> Result<Board, String> {
    let mut rank = 7i;
    let mut board = Board::empty();
    while rank >= 0 {
        let mut file = 0i;
        while file < 8 {
            let sq = Square::new(file as u8, rank as u8);
            let c = try!(read_char(iter, format!("Can't read square {0}", sq)));
            match parse_empty_squares(c) {
                Some(n) => {
                    if n + file > 8 {
                        return Err(format!("Can't put {0} empty squares at rank:{1} file:{2}", n, rank + 1, file + 1));
                    }
                    file += n;
                },
                None => {
                    let piece = try!(parse_piece(c));
                    board.set_piece(sq, piece);
                    file += 1;
                }
            };
        }
        if rank != 0 {
            try!(expect_char(iter, '/', format!("Rank delimiter / is expected after rank {0}", rank + 1)));
        }
        rank -= 1;
    }
    Ok (board)   
}

fn parse_empty_squares(c: char) -> Option<int> {
    let n = c as u32;
    let zero = '0' as u32;
    if n <= zero {
        None
    } else if (n - zero) > 8 {
        None
    } else {
        Some ((n - zero) as int)
    }
}

fn parse_piece(c: char) -> Result<Piece, String> {
    match c {
        'p' => Ok( Piece(Pawn, Black) ),
        'P' => Ok( Piece(Pawn, White) ),
        'b' => Ok( Piece(Bishop, Black) ),
        'B' => Ok( Piece(Bishop, White) ),
        'n' => Ok( Piece(Knight, Black) ),
        'N' => Ok( Piece(Knight, White) ),
        'r' => Ok( Piece(Rook, Black) ),
        'R' => Ok( Piece(Rook, White) ),
        'q' => Ok( Piece(Queen, Black) ),
        'Q' => Ok( Piece(Queen, White) ),
        'k' => Ok( Piece(King, Black) ),
        'K' => Ok( Piece(King, White) ),
        c => Err( format! ("Can't parse {} as a piece", c))
    }
}

fn expect_char(iter :&mut Chars, expected:char, err_msg:String) -> Result<(), String> {
    match iter.next() {
        Some(c) if c == expected => Ok (()),
        _ => Err(err_msg) 
    }    
}

fn read_char(iter :&mut Chars, err_msg:String) -> Result<char, String> {
    match iter.next() {
        Some(c) => Ok(c),
        None => Err(err_msg)
    }
}

