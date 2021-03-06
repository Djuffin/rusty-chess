use std::fmt;
use bitset::{BitSet, SquareIter};
pub use self::Color::*;
pub use self::Kind::*;
pub use self::CastlingRight::*;
pub use self::Move::*;


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Kind {
    Pawn = 0, Bishop = 1, Knight = 2, Rook = 3, Queen = 4, King = 5
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Color {
    White = 0, Black = 1
}

impl Color {
    #[inline]
    pub fn inverse(self) -> Color {
        match self {
            White => Black,
            Black => White
        }
    }
}

impl fmt::Display for Kind {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Pawn   => "p",
            Knight => "n",
            Bishop => "b",
            Rook   => "r",
            King   => "k",
            Queen  => "q"
        };
        write!(f, "{}", c)
     }
}

impl fmt::Display for Color {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            White => write!(f, "w"),
            Black => write!(f, "b")
        }
     }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Piece (pub Kind, pub Color);

impl Piece {
    #[inline]
    pub fn color(self) -> Color {
        let Piece(_, color) = self;
        color
    }

    #[inline]
    pub fn kind(self) -> Kind {
        let Piece(kind, _) = self;
        kind
    }

}

impl fmt::Display for Piece {
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


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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
        debug_assert!(file < 8);
        debug_assert!(rank < 8);
        Square ((rank << 3) + file )
    }

}

impl fmt::Display for Square {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}{1}",
            ('a' as u8 + self.file()) as char,
            ('1' as u8 + self.rank()) as char
        )
     }
}


#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CastlingRight {
    NoCastling = 0, QueenCastling = 1, KingCastling = 2, BothCastling = 3
}

impl CastlingRight {
    fn remove(self, to_remove:CastlingRight) -> CastlingRight {
        match to_remove {
            KingCastling => match self {
                BothCastling => QueenCastling,
                KingCastling => NoCastling,
                _ => self
            },
            QueenCastling => match self {
                BothCastling => KingCastling,
                QueenCastling => NoCastling,
                _ => self
            },
            BothCastling => NoCastling,
            NoCastling => self
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct OrdinaryMoveInfo {
    pub from: Square,
    pub to : Square,
    pub kind : Kind,
    pub promotion : Option<Kind>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Move {
    OrdinaryMove (OrdinaryMoveInfo),
    CastleKingSide,
    CastleQueenSide,
    NullMove
}

impl Move {
    #[inline]
    pub fn new(kind:Kind, from:Square, to:Square, promo: Option<Kind>) -> Move {
        OrdinaryMove(OrdinaryMoveInfo{
            from: from,
            to: to,
            kind: kind,
            promotion: promo
        })
    }
}

impl fmt::Display for Move {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            CastleKingSide => write!(f, "O-O"),
            CastleQueenSide => write!(f, "O-O-O"),
            NullMove => write!(f, "null-move"),
            OrdinaryMove (ref of) => {
                match of.promotion {
                    Some(promo) =>
                        write!(f, "{0} {1}-{2}={3}", of.kind, of.from, of.to, promo),
                    None =>
                        write!(f, "{0} {1}-{2}", of.kind, of.from, of.to)
                }
            }
        }
     }
}



#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

    pub fn clear_square(&mut self, sq:Square) {
        let not_sq = !BitSet::from_one_square(sq);
        self.pawns   = self.pawns   & not_sq;
        self.bishops = self.bishops & not_sq;
        self.knights = self.knights & not_sq;
        self.rooks   = self.rooks   & not_sq;
        self.queens  = self.queens  & not_sq;
        self.kings   = self.kings   & not_sq;

        self.whites  = self.whites  & not_sq;
        self.blacks  = self.blacks  & not_sq;
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
    pub fn get_pieces(&self, kind: Kind, color: Color) -> SquareIter {
        let pieces_bitset = self.get_piece_bitset(kind);
        let color_bitset = self.get_color_bitset(color);
        (pieces_bitset & color_bitset).iter()
    }

    #[inline]
    pub fn get_color_bitset(self, color:Color) -> BitSet {
        match color {
            White => self.whites,
            Black => self.blacks
        }
    }

    #[inline]
    pub fn get_piece_bitset(self, kind:Kind) -> BitSet {
        match kind {
            Pawn   => self.pawns,
            Bishop => self.bishops,
            Knight => self.knights,
            Rook   => self.rooks,
            Queen  => self.queens,
            King   => self.kings,
        }
    }

}

impl fmt::Display for Board {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::new(file as u8, rank as u8);
                match self.get_piece(sq) {
                    Some (p) => write!(f, "{0}", p )?,
                    None => write!(f, ".")?
                }

            }
            write!(f, "\n")?;
        }
        Ok (())
     }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub board : Board,
    pub full_moves : u16,
    pub next_to_move : Color,
    pub white_castling : CastlingRight,
    pub black_castling : CastlingRight,
    pub en_passant : Option<Square>,
    pub half_moves_since_action : u8
}

impl Position {
    pub fn gen_moves(&self) -> ::move_gen::LegalMovesIterator {
        ::move_gen::LegalMovesIterator::new(self)
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.gen_moves().count() == 0
    }

    pub fn is_check(&self) -> bool {
        let test_area = self.board.get_color_bitset(self.next_to_move) & self.board.kings;
        ::move_gen::is_under_attack(&self.board, self.next_to_move.inverse(), test_area)
    }

    pub fn apply_move(&mut self, mv:&Move) -> Option<Piece> {
        use squares::*;
        let color = self.next_to_move;
        match *mv {
            OrdinaryMove (ref mi) => {
                let mut captured_piece = self.board.get_piece(mi.to);
                debug_assert!(self.board.get_piece(mi.from).expect("src sq is empty").kind()
                    == mi.kind, "move piece is inconsistent with board piece");
                self.board.clear_square(mi.from);
                match mi.kind {
                    Queen | Bishop | Knight | Rook => {
                        self.board.set_piece(mi.to, Piece(mi.kind, color));
                        self.update_stats_after_move(captured_piece.is_some());
                        if mi.kind == Rook {
                            self.remove_rook_castling_right(mi.from, color);
                        }
                    },
                    Pawn => {
                        let piece_after_move = mi.promotion.unwrap_or(Pawn);
                        debug_assert!(mi.promotion.is_none() || mi.to.rank() == 7 || mi.to.rank() == 0,
                            "promotion before final rank");
                        self.board.set_piece(mi.to, Piece(piece_after_move, color));

                        //en passant capture
                        if Some(mi.to) == self.en_passant {
                            let jump_rank = match color {
                                White => 4, //inverse order since we'are capturing
                                Black => 3
                            };
                            let jump_sq = Square::new(mi.to.file(), jump_rank);
                            captured_piece = self.board.get_piece(jump_sq);
                            debug_assert!(captured_piece.expect("en passant capture of empty sq")
                                .kind() == Pawn, "en passant capture of not a pawn");
                            self.board.clear_square(jump_sq);
                        }

                        self.update_stats_after_move(true);
                        //en passant move
                        if color == Black && mi.from.rank() == 6 && mi.to.rank() == 4 {
                            self.en_passant = Some (Square::new(mi.to.file(), 5));
                        }
                        else if color == White && mi.from.rank() == 1 && mi.to.rank() == 3 {
                            self.en_passant = Some (Square::new(mi.to.file(), 2));
                        }
                    }
                    King => {
                        self.board.set_piece(mi.to, Piece(mi.kind, color));
                        self.update_stats_after_move(captured_piece.is_some());
                        self.remove_king_castling_right(color);
                    }
                }
                debug_assert!(captured_piece.is_none() ||
                    captured_piece.unwrap().color() == color.inverse(), "capturing friendly piece");
                match captured_piece {
                    Some(Piece(Rook, _)) =>
                        //rook is taken remove castling right for this rook
                        self.remove_rook_castling_right(mi.to, color.inverse()),
                    _ => {}
                }
                captured_piece
            }
            CastleQueenSide => {
                if color == White {
                    self.board.clear_square(a1);
                    self.board.clear_square(e1);
                    self.board.set_piece(c1, Piece(King, White));
                    self.board.set_piece(d1, Piece(Rook, White));
                } else {
                    self.board.clear_square(a8);
                    self.board.clear_square(e8);
                    self.board.set_piece(c8, Piece(King, Black));
                    self.board.set_piece(d8, Piece(Rook, Black));
                }
                self.update_stats_after_move(false);
                self.remove_king_castling_right(color);
                None
            }
            CastleKingSide => {
                if color == White {
                    self.board.clear_square(h1);
                    self.board.clear_square(e1);
                    self.board.set_piece(g1, Piece(King, White));
                    self.board.set_piece(f1, Piece(Rook, White));
                } else {
                    self.board.clear_square(h8);
                    self.board.clear_square(e8);
                    self.board.set_piece(g8, Piece(King, Black));
                    self.board.set_piece(f8, Piece(Rook, Black));
                }
                self.update_stats_after_move(false);
                self.remove_king_castling_right(color);
                None
            },
            NullMove => {
                self.update_stats_after_move(false);
                None
            }
        }
    }

    #[inline]
    fn remove_king_castling_right(&mut self, color:Color) {
        if color == White {
            self.white_castling = self.white_castling.remove(BothCastling);
        } else {
            self.black_castling = self.black_castling.remove(BothCastling);
        }
    }

    #[inline]
    fn remove_rook_castling_right(&mut self, rook_sq:Square, color:Color) {
        use squares::*;
        if color == White {
            if rook_sq == a1 {
                self.white_castling = self.white_castling.remove(QueenCastling);
            } else if rook_sq == h1 {
                self.white_castling = self.white_castling.remove(KingCastling);
            }
        } else {
            if rook_sq == a8 {
                self.black_castling = self.black_castling.remove(QueenCastling);
            } else if rook_sq == h8 {
                self.black_castling = self.black_castling.remove(KingCastling);
            }
        }
    }

    fn update_stats_after_move(&mut self, reset_move_counter:bool) {
        self.next_to_move = self.next_to_move.inverse();
        if self.next_to_move == White {
            self.full_moves += 1;
        }
        if reset_move_counter {
            self.half_moves_since_action = 0;
        } else {
            self.half_moves_since_action += 1;
        }
        self.en_passant = None;
    }
}


#[cfg(test)]
mod tests {
use fen::parse_fen;
use types::*;
use squares::*;

fn assert_squares(mut expected_sq:Vec<Square>, mut actual_sq:Vec<Square>) {
    expected_sq.sort();
    actual_sq.sort();
    assert_eq!(expected_sq, actual_sq);
}

#[test]
fn get_pieces_test() {
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let initial_position = parse_fen(initial_fen).unwrap();

    let white_pawns = initial_position.board.get_pieces(Pawn, White);
    assert_squares(vec![a2, b2, c2, d2, e2, f2, g2, h2], white_pawns.collect());

    let black_knights = initial_position.board.get_pieces(Knight, Black);
    assert_squares(vec![b8, g8], black_knights.collect());

    for &color in [White, Black].iter() {
        for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
            let board = initial_position.board;
            let squares = board.get_pieces(kind, color);
            for sq in squares {
                assert_eq!(board.get_piece(sq), Some(Piece(kind, color)));
            }
        }
    }

    let empty_fen = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let empty_position = parse_fen(empty_fen).unwrap();
    for &color in [White, Black].iter() {
        for &kind in [Pawn, Bishop, Knight, Rook, Queen, King].iter() {
            let mut empty_iter = empty_position.board.get_pieces(kind, color);
            assert_eq!(empty_iter.next(), None);
        }
    }
}

}
