use std::fmt;
use std::iter::range_step;
use bitset::BitSet;


#[deriving(PartialEq, Clone, Show)]
pub enum Kind {
    Pawn = 0, Bishop = 1, Knight = 2, Rook = 3, Queen = 4, King = 5 
}

#[deriving(PartialEq, Clone)]
pub enum Color {
    White = 0, Black = 1
}

impl fmt::Show for Color {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            White => write!(f, "w"),
            Black => write!(f, "b")
        }
     }
}

#[deriving(PartialEq, Clone)]
pub struct Piece (pub Kind, pub Color);

impl fmt::Show for Piece {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Piece(Pawn,   White) => "P",
            Piece(Knight, White) => "N",
            Piece(Bishop, White) => "B",
            Piece(Rook,   White) => "R",
            Piece(King,   White) => "K",
            Piece(Queen,  White) => "Q",
            Piece(Pawn,   Black) => "p",
            Piece(Knight, Black) => "n",
            Piece(Bishop, Black) => "b",
            Piece(Rook,   Black) => "r",
            Piece(King,   Black) => "k",
            Piece(Queen,  Black) => "q"
        };
        write!(f, "{}", c)
     }
}


#[deriving(PartialEq, Clone)]
pub struct Square (pub u8); //file - 0..2 bits; rank - 3..5 bits. 0 based

impl Square {

    #[inline(always)]
    pub fn file_and_rank(self) -> u8 {
        let Square (v) = self;
        v
    }

    #[inline(always)]
    pub fn file(self) -> u8 {
        self.file_and_rank() & 7
    }

    #[inline(always)]
    pub fn rank(self) -> u8 {
        self.file_and_rank() >> 3
    }

    #[inline(always)]
    pub fn new(file: u8, rank:u8) -> Square { 
        assert!(file < 8);
        assert!(rank < 8);
        Square ((rank << 3) + file )
    }

}

impl fmt::Show for Square {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}{1}", 
            ('a' as u8 + self.file()) as char,
            ('1' as u8 + self.rank()) as char 
        )
     }
}


#[deriving(PartialEq, Clone, Show)]
pub enum CastlingRight {
    NoCastling, QueenCastling, KingCastling, BothCastling
}

#[deriving(PartialEq, Clone)]
pub struct OrdinalMoveInfo {
    pub from: Square,
    pub to : Square,
    pub piece : Piece,
    pub promotion : Option<Kind>
}

#[deriving(PartialEq, Clone)]
pub enum Move {
    OrdinalMove (OrdinalMoveInfo),
    CastleKingSide,
    CastleQueenSide
}

impl fmt::Show for Move {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            CastleKingSide => write!(f, "O-O"),
            CastleQueenSide => write!(f, "O-O-O"),
            OrdinalMove (ref of) =>  
                write!(f, "{0} {1}-{2}", of.piece, of.from, of.to)
        }
     }
}



#[deriving(PartialEq, Clone)]
pub struct Board {
    pub whites:  BitSet,
    pub blacks:  BitSet,
    pub pawns:   BitSet,
    pub bishops: BitSet,
    pub knights: BitSet,
    pub rooks:   BitSet,
    pub queens:  BitSet,
    pub kings:   BitSet 
}

impl Board {
    pub fn empty() -> Board {
        Board {
            whites:  BitSet::empty(),
            blacks:  BitSet::empty(),
            pawns:   BitSet::empty(),
            bishops: BitSet::empty(),
            knights: BitSet::empty(),
            rooks:   BitSet::empty(),
            queens:  BitSet::empty(),
            kings:   BitSet::empty()
        }
    }

    pub fn set_piece(&mut self, sq:Square, p:Piece) {
        let Piece(kind, color) = p;
        self.pawns  .set(sq, kind == Pawn);
        self.bishops.set(sq, kind == Bishop);
        self.knights.set(sq, kind == Knight);
        self.rooks  .set(sq, kind == Rook);
        self.queens .set(sq, kind == Queen);
        self.kings  .set(sq, kind == King);

        self.whites .set(sq, color == White);
        self.blacks .set(sq, color == Black);
    }

    //retutns a piece located at a given square (if any)
    pub fn get_piece(&self, sq:Square) -> Option<Piece> {
        let sq_bits = BitSet::from_one_square(sq);
        let color = if !(self.whites & sq_bits).is_empty() {
            White
        } else if !(self.blacks & sq_bits).is_empty() {
            Black
        } else {
            return None;
        };

        if !(self.pawns & sq_bits).is_empty() {
            Some ( Piece(Pawn, color) )
        } else if !(self.bishops & sq_bits).is_empty() {
            Some ( Piece(Bishop, color) )
        } else if !(self.knights & sq_bits).is_empty() {
            Some ( Piece(Knight, color) )
        } else if !(self.rooks & sq_bits).is_empty() {
            Some ( Piece(Rook, color) )
        } else if !(self.queens & sq_bits).is_empty() {
            Some ( Piece(Queen, color) )
        } else if !(self.kings & sq_bits).is_empty() {
            Some ( Piece(King, color) )
        } else {
            unreachable!()
        }
    }

    //returns a list of squares containing pieces of given kind and color
    pub fn get_pieces(&self, kind: Kind, color: Color) -> Vec<Square> {
        let pieces_bitset = *self.get_piece_bitset(kind);
        let color_bitset = *self.get_color_bitset(color);
        (pieces_bitset & color_bitset).get_active_squares()
    }

    #[inline] 
    pub fn get_color_bitset<'a> (&'a self, color:Color) -> &'a BitSet {
        match color {
            White => &self.whites,
            Black => &self.blacks
        }
    }

    #[inline]
    pub fn get_piece_bitset<'a>(&'a self, kind:Kind) -> &'a BitSet {
        match kind {
            Pawn   => &self.pawns,
            Bishop => &self.bishops,
            Knight => &self.knights,
            Rook   => &self.rooks,
            Queen  => &self.queens,
            King   => &self.kings,
        }
    }

}

impl fmt::Show for Board {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        for rank in range_step(7i, -1, -1) {
            for file in range(0i, 8) {
                let sq = Square::new(file as u8, rank as u8);
                match self.get_piece(sq) {
                    Some (p) => try!(write!(f, "{0}", p )),
                    None => try!(write!(f, ".")) 
                }
                
            }
            try!(write!(f, "\n"));
        }
        Ok (())
     }
}


#[deriving(PartialEq, Clone)]
pub struct Position {
    pub board : Board,
    pub en_passant : Option<Square>,
    pub half_moves_since_action : u16,
    pub full_moves : u16 ,
    pub next_to_move : Color,
    pub white_castling : CastlingRight,
    pub black_castling : CastlingRight

}


#[cfg(test)]
mod tests {
use fen::parse_fen;
use types::*;

#[test]
fn get_pieces_test() {
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let initial_position = parse_fen(initial_fen).unwrap();

    let white_pawns = initial_position.board.get_pieces(Pawn, White);
    assert_eq!("[a2, b2, c2, d2, e2, f2, g2, h2]", white_pawns.to_string().as_slice());

    let black_knights = initial_position.board.get_pieces(Knight, Black);
    assert_eq!("[b8, g8]", black_knights.to_string().as_slice());

    for &color in [White, Black].iter() {
        for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
            let board = initial_position.board;
            let squares = board.get_pieces(kind, color);
            for &sq in squares.iter() {
                assert_eq!(board.get_piece(sq), Some(Piece(kind, color)));
            }
        }
    }

    let mut empty_fen = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut empty_position = parse_fen(empty_fen).unwrap();
    for &color in [White, Black].iter() {
        for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
            let empty_list = empty_position.board.get_pieces(kind, color);
            assert_eq!(empty_list.len(), 0);
        }
    }


} 

}