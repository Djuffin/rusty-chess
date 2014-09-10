use types::*;
use bitset::BitSet;

//Dirty bit tricks are used in move_gen 
//see more here:
//https://chessprogramming.wikispaces.com/Efficient+Generation+of+Sliding+Piece+Attacks
//https://chessprogramming.wikispaces.com/Hyperbola+Quintessence


pub struct MovesIterator {
    position: Position,
    moves_cache: Vec<Move>,
    occupied_set: BitSet,
    friendly_set: BitSet,
    next_kind: Kind,
    can_gen_more: bool
}

impl Iterator<Move> for MovesIterator {
    fn next(&mut self) -> Option<Move> {
        while self.moves_cache.is_empty() {
            if !self.can_gen_more {
                return None
            }
            self.can_gen_more = self.gen_more_moves();
        } 
        self.moves_cache.pop()
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        (self.moves_cache.len(), None)    
    }  
}

impl MovesIterator {
    pub fn new(pos: &Position) -> MovesIterator {
        MovesIterator {
            position : *pos,
            moves_cache : Vec::with_capacity(32),
            next_kind : Queen,
            occupied_set : pos.board.whites | pos.board.blacks,
            friendly_set : pos.board.get_color_bitset(pos.next_to_move),
            can_gen_more : true
        }
    }

    fn gen_more_moves(&mut self) -> bool {
        let result = &mut self.moves_cache;
        let occupied_set = self.occupied_set;
        let friendly_set = self.friendly_set;
        let pos = &self.position;
        let board = &pos.board;
        let color = pos.next_to_move;
        match self.next_kind {
           Queen => {
                for from_sq in board.get_pieces(Queen, color) {
                    let moves_set = gen_queen_moves(occupied_set, friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Queen, color, from_sq, to_sq));
                    }
                } 
                self.next_kind = Rook;
                true           
           } 
           Rook => {
                for from_sq in board.get_pieces(Rook, color) {
                    let moves_set = gen_rook_moves(occupied_set, friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Rook, color, from_sq, to_sq));
                    }
                }
                self.next_kind = Bishop;
                true
           }
           Bishop => {
                for from_sq in board.get_pieces(Bishop, color) {
                    let moves_set = gen_bishop_moves(occupied_set, friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Bishop, color, from_sq, to_sq));
                    }
                }            
                self.next_kind = Knight;
                true
           }
           Knight => {
                for from_sq in board.get_pieces(Knight, color) {
                    let moves_set = gen_knight_moves(friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Knight, color, from_sq, to_sq));
                    }
                }
                self.next_kind = Pawn;
                true
           }
           Pawn => {
                let en_passant_set = match pos.en_passant {
                                        Some(s) => BitSet::from_one_square(s),
                                        None => BitSet::empty()
                                     };
                 
                if color == White {
                    //free_set is a set of squares a pawn can go to.
                    //We copy all occupied squares on 3rd rank to 4th rank in order 
                    //to account for the fact that if pawn can't step one square 
                    //it also can't step two squares at a time. 
                    let free_set = !(occupied_set | BitSet::new(occupied_set.get_rank(2) as u64) << 8 * 3);
                    let pawn_enemy_set = board.get_color_bitset(Black) | en_passant_set;        
                    for from_sq in board.get_pieces(Pawn, color) {
                        let moves_set = gen_white_pawn_moves(free_set, pawn_enemy_set, from_sq);
                        add_pawn_moves(result, color, from_sq, moves_set);
                    }
                } else {
                    //see comment for whites
                    let free_set = !(occupied_set | BitSet::new(occupied_set.get_rank(5) as u64) << 8 * 4);        
                    let pawn_enemy_set = board.get_color_bitset(White) | en_passant_set;
                    for from_sq in board.get_pieces(Pawn, color) {
                        let moves_set = gen_black_pawn_moves(free_set, pawn_enemy_set, from_sq);
                        add_pawn_moves(result, color, from_sq, moves_set);
                    }
                }            
                self.next_kind = King;
                true
           },
           King => {
                for from_sq in board.get_pieces(King, color) {
                    let moves_set = gen_king_moves(friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(King, color, from_sq, to_sq));
                    }
                }

                //castling
                let (castle_rank, queen_castle_allowed, king_castle_allowed) = 
                    match (color, pos.white_castling, pos.black_castling) {
                    (White, QueenCastling, _) => (0, true,  false), 
                    (White, KingCastling, _)  => (0, false, true),
                    (White, BothCastling, _)  => (0, true,  true),
                    (Black, _, QueenCastling) => (7, true,  false),   
                    (Black, _, KingCastling)  => (7, false, true),  
                    (Black, _, BothCastling)  => (7, true,  true),
                    (White, NoCastling, _)| (Black, _, NoCastling)  => (3, false, false)
                };

                //here we assume that if castling right is specified king and rook
                //are on the castling ready position
                if king_castle_allowed {
                    if occupied_set.get_rank(castle_rank) & 0b01100000u8 == 0 {
                        result.push(CastleKingSide);
                    }
                }
                if queen_castle_allowed {
                    if occupied_set.get_rank(castle_rank) & 0b00001110u8 == 0 {
                        result.push(CastleQueenSide);
                    }
                }
                false
           }
        }
    }
}


#[inline]
fn add_pawn_moves(list:&mut Vec<Move>, color: Color, from:Square, moves:BitSet) {
    for to in moves.iter() {
        if to.rank() == 7 || to.rank() == 1 {
            for &p in [Queen, Rook, Bishop, Knight].iter() {
                list.push( Move::new(Piece(Pawn, color), from, to, Some(p)) )
            }
        } else {
            list.push( Move::new(Piece(Pawn, color), from, to, None) );
        }
    }
}

#[inline]
fn squares_to_move(kind:Kind, color: Color, from:Square, to:Square) -> Move {
    Move::new(Piece(kind, color), from, to, None)
}

fn gen_white_pawn_moves(free_set:BitSet, enemy_set:BitSet, sq:Square) -> BitSet {
    use tables::{get_white_pawn_moves_mask, get_white_pawn_attacks_mask};
    (get_white_pawn_moves_mask(sq) & free_set) 
        | (get_white_pawn_attacks_mask(sq) & enemy_set) 
}

fn gen_black_pawn_moves(free_set:BitSet, enemy_set:BitSet, sq:Square) -> BitSet {
    use tables::{get_black_pawn_moves_mask, get_black_pawn_attacks_mask};
    (get_black_pawn_moves_mask(sq) & free_set) 
        | (get_black_pawn_attacks_mask(sq) & enemy_set) 
}

//generate rook moves on a given board from a given square 
//we don't check that rook is acutally there
fn gen_rook_moves(occupied_set:BitSet, friendly_set:BitSet, sq:Square) -> BitSet {
    let rank_move_set = gen_rank_sliding_moves(occupied_set, sq);
    let file_move_set = gen_file_sliding_moves(occupied_set, sq);

    (rank_move_set | file_move_set) & !friendly_set
}

//generate bishop moves on a given board from a given square 
//we don't check that bishop is acutally there
fn gen_bishop_moves(occupied_set:BitSet, friendly_set:BitSet, sq:Square) -> BitSet {
    let diag_move_set     = gen_diagonal_sliding_moves(occupied_set, sq);
    let antidiag_move_set = gen_antidiagonal_sliding_moves(occupied_set, sq);

    (diag_move_set | antidiag_move_set) & !friendly_set
}

//generate queen moves on a given board from a given square 
//we don't check that queen is acutally there
fn gen_queen_moves(occupied_set:BitSet, friendly_set:BitSet, sq:Square) -> BitSet {
    let diag_move_set     = gen_diagonal_sliding_moves(occupied_set, sq);
    let antidiag_move_set = gen_antidiagonal_sliding_moves(occupied_set, sq);
    let rank_move_set     = gen_rank_sliding_moves(occupied_set, sq);
    let file_move_set     = gen_file_sliding_moves(occupied_set, sq);

    (diag_move_set | antidiag_move_set | rank_move_set | file_move_set) 
        & !friendly_set
}

fn gen_king_moves(friendly_set:BitSet, sq:Square) -> BitSet {
    use tables::get_king_moves_mask;
    let raw_moves = get_king_moves_mask(sq);
    raw_moves & !friendly_set
}

fn gen_knight_moves(friendly_set:BitSet, sq:Square) -> BitSet {
    use tables::get_knight_moves_mask;
    let raw_moves = get_knight_moves_mask(sq);
    raw_moves & !friendly_set
}

#[inline]
fn gen_rank_sliding_moves(occupied_set:BitSet, sq:Square) -> BitSet {
    use utils::reverse;
    let rank_blockers_mask:u8 = occupied_set.get_rank(sq.rank());
    let piece_mask:u8 = 1u8 << sq.file() as uint;
    let rank_attack = (rank_blockers_mask - (piece_mask << 1)) ^ 
                      reverse(reverse(rank_blockers_mask) - (reverse(piece_mask) << 1));
    BitSet::new( (rank_attack as u64) << (sq.rank() * 8) as uint )
}

#[inline]
fn gen_file_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
    use tables::get_file_mask;
    let file_mask = get_file_mask(sq);
    let piece_mask = BitSet::from_one_square(sq);
    let occupied_file = occupied_set & file_mask;
    let ray_up = occupied_file - piece_mask;
    let ray_down = (occupied_file.swap() - piece_mask.swap()).swap();
    let changes = ray_up ^ ray_down;
    changes & file_mask
}

#[inline]
fn gen_diagonal_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
    use tables::get_diagonal_mask;
    let diag_mask = get_diagonal_mask(sq);
    let piece_mask = BitSet::from_one_square(sq);
    let occupied_diag = occupied_set & diag_mask;
    let ray_up = occupied_diag - piece_mask;
    let ray_down = (occupied_diag.swap() - piece_mask.swap()).swap();
    let changes = ray_up ^ ray_down;
    changes & diag_mask
}

#[inline]
fn gen_antidiagonal_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
    use tables::get_antidiagonal_mask;
    let antidiag_mask = get_antidiagonal_mask(sq);
    let piece_mask = BitSet::from_one_square(sq);
    let occupied_diag = occupied_set & antidiag_mask;
    let ray_up = occupied_diag - piece_mask;
    let ray_down = (occupied_diag.swap() - piece_mask.swap()).swap();
    let changes = ray_up ^ ray_down;
    changes & antidiag_mask
}

#[cfg(test)]
mod tests {
use fen::parse_fen;
use types::*;
use move_gen::MovesIterator;
use move_gen::tests::squares::*;

#[allow(dead_code)]
mod squares {
    use types::{Square};
    pub static a1:Square = Square(0 * 8 + 0);
    pub static a2:Square = Square(1 * 8 + 0);
    pub static a3:Square = Square(2 * 8 + 0);
    pub static a4:Square = Square(3 * 8 + 0);
    pub static a5:Square = Square(4 * 8 + 0);
    pub static a6:Square = Square(5 * 8 + 0);
    pub static a7:Square = Square(6 * 8 + 0);
    pub static a8:Square = Square(7 * 8 + 0);

    pub static b1:Square = Square(0 * 8 + 1);
    pub static b2:Square = Square(1 * 8 + 1);
    pub static b3:Square = Square(2 * 8 + 1);
    pub static b4:Square = Square(3 * 8 + 1);
    pub static b5:Square = Square(4 * 8 + 1);
    pub static b6:Square = Square(5 * 8 + 1);
    pub static b7:Square = Square(6 * 8 + 1);
    pub static b8:Square = Square(7 * 8 + 1);

    pub static c1:Square = Square(0 * 8 + 2);
    pub static c2:Square = Square(1 * 8 + 2);
    pub static c3:Square = Square(2 * 8 + 2);
    pub static c4:Square = Square(3 * 8 + 2);
    pub static c5:Square = Square(4 * 8 + 2);
    pub static c6:Square = Square(5 * 8 + 2);
    pub static c7:Square = Square(6 * 8 + 2);
    pub static c8:Square = Square(7 * 8 + 2);

    pub static d1:Square = Square(0 * 8 + 3);
    pub static d2:Square = Square(1 * 8 + 3);
    pub static d3:Square = Square(2 * 8 + 3);
    pub static d4:Square = Square(3 * 8 + 3);
    pub static d5:Square = Square(4 * 8 + 3);
    pub static d6:Square = Square(5 * 8 + 3);
    pub static d7:Square = Square(6 * 8 + 3);
    pub static d8:Square = Square(7 * 8 + 3);

    pub static e1:Square = Square(0 * 8 + 4);
    pub static e2:Square = Square(1 * 8 + 4);
    pub static e3:Square = Square(2 * 8 + 4);
    pub static e4:Square = Square(3 * 8 + 4);
    pub static e5:Square = Square(4 * 8 + 4);
    pub static e6:Square = Square(5 * 8 + 4);
    pub static e7:Square = Square(6 * 8 + 4);
    pub static e8:Square = Square(7 * 8 + 4);

    pub static f1:Square = Square(0 * 8 + 5);
    pub static f2:Square = Square(1 * 8 + 5);
    pub static f3:Square = Square(2 * 8 + 5);
    pub static f4:Square = Square(3 * 8 + 5);
    pub static f5:Square = Square(4 * 8 + 5);
    pub static f6:Square = Square(5 * 8 + 5);
    pub static f7:Square = Square(6 * 8 + 5);
    pub static f8:Square = Square(7 * 8 + 5);

    pub static g1:Square = Square(0 * 8 + 6);
    pub static g2:Square = Square(1 * 8 + 6);
    pub static g3:Square = Square(2 * 8 + 6);
    pub static g4:Square = Square(3 * 8 + 6);
    pub static g5:Square = Square(4 * 8 + 6);
    pub static g6:Square = Square(5 * 8 + 6);
    pub static g7:Square = Square(6 * 8 + 6);
    pub static g8:Square = Square(7 * 8 + 6);

    pub static h1:Square = Square(0 * 8 + 7);
    pub static h2:Square = Square(1 * 8 + 7);
    pub static h3:Square = Square(2 * 8 + 7);
    pub static h4:Square = Square(3 * 8 + 7);
    pub static h5:Square = Square(4 * 8 + 7);
    pub static h6:Square = Square(5 * 8 + 7);
    pub static h7:Square = Square(6 * 8 + 7);
    pub static h8:Square = Square(7 * 8 + 7);
}

fn from_square(sq:Square, it:MovesIterator) -> Vec<Move>{
    let filter_it = it.filter_map(|m| {
        match m {
            OrdinalMove(mi) if mi.from == sq => Some(m), 
            _ => None
        }
    });
    let mut result:Vec<Move> = FromIterator::from_iter(filter_it);
    result.sort();
    result    
}

fn prepare_moves(piece: Piece, from:Square, squares:&[Square]) -> Vec<Move> {
    let it = squares.iter().map(|to_sq| Move::new(piece, from, *to_sq, None));
    let mut result:Vec<Move> = FromIterator::from_iter(it);
    result.sort();
    result
}

fn assert_moves(fen:&str, from:Square, expected_moves:&[Move]) {
    let pos = parse_fen(fen).unwrap();
    let it = MovesIterator::new(&pos);
    let generated_moves = from_square(from, it);
    let mut expected_moves = Vec::from_slice(expected_moves);
    expected_moves.sort();
    assert_eq!(generated_moves, expected_moves);    
}

fn assert_squares(fen:&str, from:Square, squares:&[Square]) {
    let pos = parse_fen(fen).unwrap();
    let it = MovesIterator::new(&pos);
    let generated_moves = from_square(from, it);
    let piece = pos.board.get_piece(from).unwrap();
    let expected_moves = prepare_moves(piece, from, squares);
    assert_eq!(generated_moves, expected_moves);    
}

#[test]
fn rook_moves_test() {
    ::tables::init_square_data();
    let fen = "R6R/8/8/3rr3/3RR3/8/8/r6r b - - 0 1"; 
    
    //moves of black rook a1
    assert_squares(fen, a1, [a2, a3, a4, a5, a6, a7, a8, b1, c1, d1, e1, g1, f1]);

    //moves of black rook d5
    assert_squares(fen, d5, [a5, b5, c5, d4, d6, d7, d8]);

    //moves of black rook e5
    assert_squares(fen, e5, [f5, g5, h5, e6, e7, e8, e4]);

    //moves of black rook h1
    assert_squares(fen, h1, [b1, c1, d1, e1, f1, g1, h2, h3, h4, h5, h6, h7, h8]);

    //same but white to move
    let fen = "R6R/8/8/3rr3/3RR3/8/8/r6r w - - 0 1"; 
    
    //moves of white rook a8
    assert_squares(fen, a8, [a1, a2, a3, a4, a5, a6, a7, b8, c8, d8, e8, f8, g8]);

    //moves of white rook d4
    assert_squares(fen, d4, [a4, b4, c4, d5, d3, d2, d1]);

    //moves of white rook e4
    assert_squares(fen, e4, [f4, g4, h4, e5, e3, e2, e1]);

    //moves of white rook h8
    assert_squares(fen, h8, [b8, c8, d8, e8, f8, g8, h1, h2, h3, h4, h5, h6, h7]);
}

#[test]
fn bishop_moves_test() {
    ::tables::init_square_data();
    let fen = "b7/8/8/8/2bB4/8/pP2Pp2/7B w - - 0 1"; 

    //moves of white bishop d4
    assert_squares(fen, d4, [c3, e5, f6, g7, h8, a7, b6, c5, e3, f2]);

    //moves of white bishop h1
    assert_squares(fen, h1, [a8, b7, c6, d5, e4, f3, g2]);

    let fen = "b7/8/8/8/2bB4/8/pP2Pp2/7B b - - 0 1";

    //moves of black bishop a8
    assert_squares(fen, a8, [b7, c6, d5, e4, f3, g2, h1]);

    //moves of black bishop c4
    assert_squares(fen, c4, [a6, b5, d3, e2, b3, d5, e6, f7, g8]);
}

#[test]
fn pawn_moves_test() {
    ::tables::init_square_data();
    let fen = "4q3/3P2p1/5N1N/4p3/1Pp1p3/2K5/P7/8 w - - 0 1";

    assert_squares(fen, a2, [a3, a4]);    
    assert_squares(fen, b4, [b5]);
    assert_moves(fen, d7, [
        Move::new(Piece(Pawn, White), d7, d8, Some(Queen)),
        Move::new(Piece(Pawn, White), d7, d8, Some(Rook)),
        Move::new(Piece(Pawn, White), d7, d8, Some(Bishop)),
        Move::new(Piece(Pawn, White), d7, d8, Some(Knight)),
        Move::new(Piece(Pawn, White), d7, e8, Some(Queen)),
        Move::new(Piece(Pawn, White), d7, e8, Some(Rook)),
        Move::new(Piece(Pawn, White), d7, e8, Some(Bishop)),
        Move::new(Piece(Pawn, White), d7, e8, Some(Knight))
    ]);

    let fen = "4q3/3P2p1/5N1N/4p3/1Pp1p3/2K5/P7/8 b - b3 0 1";    

    assert_squares(fen, c4, [b3]);
    assert_squares(fen, e5, []);
    assert_squares(fen, e4, [e3]);
    assert_squares(fen, g7, [f6, h6, g6, g5]);
}


} 