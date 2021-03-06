use types::*;
use bitset::BitSet;

//Dirty bit tricks are used in move_gen
//see more here:
//https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks
//https://www.chessprogramming.org/Hyperbola_Quintessence

pub struct LegalMovesIterator {
    moves_iter: MovesIterator
}

impl Iterator for LegalMovesIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        loop {
            let next_move = self.moves_iter.next();
            match next_move {
                Some(m) => {
                    if is_legal_move(&self.moves_iter.position, &m) {
                        return next_move;
                    }
                }
                None => { return None; }
            };
        }
    }
}

impl LegalMovesIterator {
    pub fn new(pos: &Position) -> LegalMovesIterator {
        LegalMovesIterator {
            moves_iter : MovesIterator::new(pos)
        }
    }
}

pub struct MovesIterator {
    position: Position,
    moves_cache: Vec<Move>,
    occupied_set: BitSet,
    friendly_set: BitSet,
    next_kind: Kind,
    can_gen_more: bool
}

impl Iterator for MovesIterator {
    type Item = Move;

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
    fn size_hint(&self) -> (usize, Option<usize>) {
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
                        result.push(squares_to_move(Queen, from_sq, to_sq));
                    }
                }
                self.next_kind = Rook;
                true
           }
           Rook => {
                for from_sq in board.get_pieces(Rook, color) {
                    let moves_set = gen_rook_moves(occupied_set, friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Rook, from_sq, to_sq));
                    }
                }
                self.next_kind = Bishop;
                true
           }
           Bishop => {
                for from_sq in board.get_pieces(Bishop, color) {
                    let moves_set = gen_bishop_moves(occupied_set, friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Bishop, from_sq, to_sq));
                    }
                }
                self.next_kind = Knight;
                true
           }
           Knight => {
                for from_sq in board.get_pieces(Knight, color) {
                    let moves_set = gen_knight_moves(friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(Knight, from_sq, to_sq));
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
                    let pawn_enemy_set = board.get_color_bitset(Black) | en_passant_set;
                    for from_sq in board.get_pieces(Pawn, color) {
                        let moves_set = gen_white_pawn_moves(occupied_set, pawn_enemy_set, from_sq);
                        add_pawn_moves(result, from_sq, moves_set);
                    }
                } else {
                    let pawn_enemy_set = board.get_color_bitset(White) | en_passant_set;
                    for from_sq in board.get_pieces(Pawn, color) {
                        let moves_set = gen_black_pawn_moves(occupied_set, pawn_enemy_set, from_sq);
                        add_pawn_moves(result, from_sq, moves_set);
                    }
                }
                self.next_kind = King;
                true
           },
           King => {
                for from_sq in board.get_pieces(King, color) {
                    let moves_set = gen_king_moves(friendly_set, from_sq);
                    for to_sq in moves_set.iter() {
                        result.push(squares_to_move(King, from_sq, to_sq));
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

                //here we assume that if castling right is specified then
                //king and rook are on the castling ready positions
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
fn is_legal_move(pos: &Position, mv: &Move) -> bool {
    let mut new_pos = *pos;
    new_pos.apply_move(mv);
    let test_area = match (*mv, pos.next_to_move) {
        (CastleQueenSide, White) => BitSet::new(0b00011100u64),
        (CastleKingSide,  White) => BitSet::new(0b01110000u64),
        (CastleQueenSide, Black) => BitSet::new(0b00011100u64) << 7 * 8,
        (CastleKingSide,  Black) => BitSet::new(0b01110000u64) << 7 * 8,
        (_, White)               => new_pos.board.kings & new_pos.board.whites,
        (_, Black)               => new_pos.board.kings & new_pos.board.blacks,
    };
    !is_under_attack(&new_pos.board, new_pos.next_to_move, test_area)
}

pub fn is_under_attack(board: &Board, attacking_color: Color, test_area:BitSet) -> bool {
    let occupied_set = board.whites | board.blacks;
    let friendly_set = BitSet::empty(); //here it doesn't matter who's friend

    for from_sq in board.get_pieces(Queen, attacking_color) {
        let attack_set = gen_queen_moves(occupied_set, friendly_set, from_sq);
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    for from_sq in board.get_pieces(Rook, attacking_color) {
        let attack_set = gen_rook_moves(occupied_set, friendly_set, from_sq);
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    for from_sq in board.get_pieces(Bishop, attacking_color) {
        let attack_set = gen_bishop_moves(occupied_set, friendly_set, from_sq);
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    for from_sq in board.get_pieces(Knight, attacking_color) {
        let attack_set = ::tables::get_knight_moves_mask(from_sq);
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    if attacking_color == White {
        let mut attack_set = BitSet::empty();
        for from_sq in board.get_pieces(Pawn, attacking_color) {
            attack_set = attack_set | ::tables::get_white_pawn_attacks_mask(from_sq);
        }
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    } else {
        let mut attack_set = BitSet::empty();
        for from_sq in board.get_pieces(Pawn, attacking_color) {
            attack_set = attack_set | ::tables::get_black_pawn_attacks_mask(from_sq);
        }
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    for from_sq in board.get_pieces(King, attacking_color) {
        let attack_set = gen_king_moves(friendly_set, from_sq);
        if !(attack_set & test_area).is_empty() {
            return true;
        }
    }
    false
}

#[inline]
fn add_pawn_moves(list:&mut Vec<Move>, from:Square, moves:BitSet) {
    for to in moves.iter() {
        if to.rank() == 7 || to.rank() == 0 {
            for &p in [Queen, Rook, Bishop, Knight].iter() {
                list.push( Move::new(Pawn, from, to, Some(p)) )
            }
        } else {
            list.push( Move::new(Pawn, from, to, None) );
        }
    }
}

#[inline]
fn squares_to_move(kind:Kind, from:Square, to:Square) -> Move {
    Move::new(kind, from, to, None)
}

fn gen_white_pawn_moves(occupied_set:BitSet, enemy_set:BitSet, sq:Square) -> BitSet {
    use tables::{get_white_pawn_moves_mask, get_white_pawn_attacks_mask};
    let free_set = if sq.rank() == 1 {
        //free_set is a set of squares a pawn can go to.
        //We copy all occupied squares on 3rd rank to 4th rank in order
        //to account for the fact that if pawn can't step one square
        //it also can't step two squares at a time.
        !(occupied_set | BitSet::new(occupied_set.get_rank(2) as u64) << 8 * 3)
    } else {
        !occupied_set
    };
    (get_white_pawn_moves_mask(sq) & free_set)
        | (get_white_pawn_attacks_mask(sq) & enemy_set)
}

pub fn gen_black_pawn_moves(occupied_set:BitSet, enemy_set:BitSet, sq:Square) -> BitSet {
    use tables::{get_black_pawn_moves_mask, get_black_pawn_attacks_mask};
    let free_set = if sq.rank() == 6 {
        //see comment for whites
        !(occupied_set | BitSet::new(occupied_set.get_rank(5) as u64) << 8 * 4)
    } else {
        !occupied_set
    };
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
    use tables::reverse;
    let rank_blockers_mask:u8 = occupied_set.get_rank(sq.rank());
    let piece_mask:u8 = 1u8 << sq.file() as usize;
    let rank_attack = (rank_blockers_mask.wrapping_sub(piece_mask << 1)) ^
                      reverse(reverse(rank_blockers_mask).wrapping_sub(reverse(piece_mask) << 1));
    BitSet::new( (rank_attack as u64) << (sq.rank() * 8) as usize )
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
use squares::*;

fn from_square(sq:Square, it:MovesIterator) -> Vec<Move>{
    let mut result:Vec<Move> = it.filter_map(|m| {
        match m {
            OrdinaryMove(mi) if mi.from == sq => Some(m),
            _ => None
        }
    }).collect();
    result.sort();
    result
}

fn prepare_moves(kind: Kind, from:Square, squares:&[Square]) -> Vec<Move> {
    let it = squares.iter().map(|to_sq| Move::new(kind, from, *to_sq, None));
    let mut result:Vec<Move> = it.collect();
    result.sort();
    result
}

fn assert_moves(fen:&str, from:Square, expected_moves:&[Move]) {
    let pos = parse_fen(fen).unwrap();
    let it = MovesIterator::new(&pos);
    let generated_moves = from_square(from, it);
    let mut expected_moves = expected_moves.to_vec();
    expected_moves.sort();
    assert_eq!(generated_moves, expected_moves);
}

fn assert_castles(fen:&str, expected_moves:&[Move]) {
    let pos = parse_fen(fen).unwrap();
    let it = MovesIterator::new(&pos);
    let mut generated_moves:Vec<Move> = it.filter_map(|m| {
        match m {
            CastleQueenSide | CastleKingSide => Some(m),
            _ => None
        }
    }).collect();
    generated_moves.sort();
    let mut expected_moves = expected_moves.to_vec();
    expected_moves.sort();
    assert_eq!(generated_moves, expected_moves);
}

fn assert_squares(fen:&str, from:Square, squares:&[Square]) {
    let pos = parse_fen(fen).unwrap();
    let it = MovesIterator::new(&pos);
    let generated_moves = from_square(from, it);
    let piece = pos.board.get_piece(from).unwrap();
    let expected_moves = prepare_moves(piece.kind(), from, squares);
    assert_eq!(generated_moves, expected_moves);
}

#[test]
fn rook_moves_test() {
    ::tables::init_tables();
    let fen = "R6R/8/8/3rr3/3RR3/8/8/r6r b - - 0 1";
    assert_squares(fen, a1, &[a2, a3, a4, a5, a6, a7, a8, b1, c1, d1, e1, g1, f1]);
    assert_squares(fen, d5, &[a5, b5, c5, d4, d6, d7, d8]);
    assert_squares(fen, e5, &[f5, g5, h5, e6, e7, e8, e4]);
    assert_squares(fen, h1, &[b1, c1, d1, e1, f1, g1, h2, h3, h4, h5, h6, h7, h8]);

    //same but white to move
    let fen = "R6R/8/8/3rr3/3RR3/8/8/r6r w - - 0 1";
    assert_squares(fen, a8, &[a1, a2, a3, a4, a5, a6, a7, b8, c8, d8, e8, f8, g8]);
    assert_squares(fen, d4, &[a4, b4, c4, d5, d3, d2, d1]);
    assert_squares(fen, e4, &[f4, g4, h4, e5, e3, e2, e1]);
    assert_squares(fen, h8, &[b8, c8, d8, e8, f8, g8, h1, h2, h3, h4, h5, h6, h7]);
}

#[test]
fn bishop_moves_test() {
    ::tables::init_tables();
    let fen = "b7/8/8/8/2bB4/8/pP2Pp2/7B w - - 0 1";
    assert_squares(fen, d4, &[c3, e5, f6, g7, h8, a7, b6, c5, e3, f2]);
    assert_squares(fen, h1, &[a8, b7, c6, d5, e4, f3, g2]);

    let fen = "b7/8/8/8/2bB4/8/pP2Pp2/7B b - - 0 1";
    assert_squares(fen, a8, &[b7, c6, d5, e4, f3, g2, h1]);
    assert_squares(fen, c4, &[a6, b5, d3, e2, b3, d5, e6, f7, g8]);
}

#[test]
fn queen_moves_test() {
    ::tables::init_tables();
    let fen = "Q7/8/5q2/8/8/2Q5/8/8 w - - 0 1";
    assert_squares(fen, a8, &[a1, a2, a3, a4, a5, a6, a7,
                             b8, c8, d8, e8, f8, g8, h8,
                             b7, c6, d5, e4, f3, g2, h1]);
    assert_squares(fen, c3, &[a3, b3, d3, e3, f3, g3, h3,
                             c1, c2, c4, c5, c6, c7, c8,
                             a1, b2, d4, e5, f6,
                             a5, b4, d2, e1]);

    let fen = "Q7/8/5q2/8/8/2Q5/8/8 b - - 0 1";
    assert_squares(fen, f6, &[a6, b6, c6, d6, e6, g6, h6,
                             f1, f2, f3, f4, f5, f7, f8,
                             c3, d4, e5, g7, h8,
                             d8, e7, g5, h4]);
}

#[test]
fn knight_moves_test() {
    ::tables::init_tables();
    let fen = "N7/8/7p/4n3/6n1/8/5p2/8 w - - 0 1";
    assert_squares(fen, a8, &[b6, c7]);

    let fen = "N7/8/7p/4n3/6n1/8/5p2/8 b - - 0 1";
    assert_squares(fen, e5, &[c6, d7, c4, d3, f3, f7, g6]);
    assert_squares(fen, g4, &[h2, e3, f6]);
}

#[test]
fn king_moves_test() {
    ::tables::init_tables();
    let fen = "rn2k2r/8/8/2K2k2/8/8/8/R3K1NR w KQkq - 0 1";
    assert_squares(fen, c5, &[c4, c6, b5, d5, d4, d6, b4, b6]);
    assert_squares(fen, e1, &[d1, d2, e2, f2, f1]);
    assert_castles(fen, &[CastleQueenSide]);

    let fen = "rn2k2r/8/8/2K2k2/8/8/8/R3K1NR b KQkq - 0 1";
    assert_squares(fen, f5, &[f4, f6, e4, e5, e6, g4, g5, g6]);
    assert_squares(fen, e8, &[d8, d7, e7, f7, f8]);
    assert_castles(fen, &[CastleKingSide]);
}

#[test]
fn pawn_moves_test() {
    ::tables::init_tables();
    let fen = "4q3/3P2p1/5N1N/4p3/1Pp1p3/2K3P1/P7/8 w - - 0 1";

    assert_squares(fen, a2, &[a3, a4]);
    assert_squares(fen, b4, &[b5]);
    assert_squares(fen, g3, &[g4]);
    assert_moves(fen, d7, &[
        Move::new(Pawn, d7, d8, Some(Queen)),
        Move::new(Pawn, d7, d8, Some(Rook)),
        Move::new(Pawn, d7, d8, Some(Bishop)),
        Move::new(Pawn, d7, d8, Some(Knight)),
        Move::new(Pawn, d7, e8, Some(Queen)),
        Move::new(Pawn, d7, e8, Some(Rook)),
        Move::new(Pawn, d7, e8, Some(Bishop)),
        Move::new(Pawn, d7, e8, Some(Knight))
    ]);

    let fen = "4q3/3P2p1/5N1N/4p3/1Pp1p3/2K3P1/P7/8 b - b3 0 1";

    assert_squares(fen, c4, &[b3]);
    assert_squares(fen, e5, &[]);
    assert_squares(fen, e4, &[e3]);
    assert_squares(fen, g7, &[f6, h6, g6, g5]);
}

}
