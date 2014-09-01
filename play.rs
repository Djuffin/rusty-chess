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
pub struct Square (u8); //file - 0..2 bits; rank - 3..5 bits. 0 based

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
        self.whites .set(sq, false);
        self.blacks .set(sq, false);
        self.pawns  .set(sq, false);
        self.bishops.set(sq, false);
        self.knights.set(sq, false);
        self.rooks  .set(sq, false);
        self.queens .set(sq, false);
        self.kings  .set(sq, false);
        
        let piece_set = match kind {
            Pawn   => &mut self.pawns,
            Bishop => &mut self.bishops,
            Knight => &mut self.knights,
            Rook   => &mut self.rooks,
            Queen  => &mut self.queens,
            King   => &mut self.kings,
        };
        piece_set.set(sq, true);

        let color_set = match color {
            White => &mut self.whites,
            Black => &mut self.blacks
        };
        color_set.set(sq, true);
    }

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

