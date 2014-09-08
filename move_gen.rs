use types::*;
use bitset::BitSet;

//Dirty bit tricks are used in move_gen 
//see more here:
//https://chessprogramming.wikispaces.com/Efficient+Generation+of+Sliding+Piece+Attacks
//https://chessprogramming.wikispaces.com/Hyperbola+Quintessence


pub fn gen_moves(pos:Position) -> Vec<Move> {
    let board = &pos.board;
    let color = pos.next_to_move;
    let occupied_set = board.whites | board.blacks;
    let friendly_set = board.get_color_bitset(color);
    let mut result:Vec<Move> = Vec::with_capacity(40);

    //queen
    for from_sq in board.get_pieces(Queen, color) {
        let moves_set = gen_queen_moves(occupied_set, friendly_set, from_sq);
        for to_sq in moves_set.iter() {
            result.push(squares_to_move(Queen, color, from_sq, to_sq));
        }
    }

    //rook
    for from_sq in board.get_pieces(Rook, color) {
        let moves_set = gen_rook_moves(occupied_set, friendly_set, from_sq);
        for to_sq in moves_set.iter() {
            result.push(squares_to_move(Rook, color, from_sq, to_sq));
        }
    }    

    //bishops
    for from_sq in board.get_pieces(Bishop, color) {
        let moves_set = gen_bishop_moves(occupied_set, friendly_set, from_sq);
        for to_sq in moves_set.iter() {
            result.push(squares_to_move(Bishop, color, from_sq, to_sq));
        }
    }

    //knight
    for from_sq in board.get_pieces(Knight, color) {
        let moves_set = gen_knight_moves(friendly_set, from_sq);
        for to_sq in moves_set.iter() {
            result.push(squares_to_move(Knight, color, from_sq, to_sq));
        }
    }

    //pawns
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
            add_pawn_moves(&mut result, color, from_sq, moves_set);
        }
    } else {
        //see comment for whites
        let free_set = !(occupied_set | BitSet::new(occupied_set.get_rank(5) as u64) << 8 * 4);        
        let pawn_enemy_set = board.get_color_bitset(White) | en_passant_set;
        for from_sq in board.get_pieces(Pawn, color) {
            let moves_set = gen_black_pawn_moves(free_set, pawn_enemy_set, from_sq);
            add_pawn_moves(&mut result, color, from_sq, moves_set);
        }
    }


    //king
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

    result
}


#[inline]
fn add_pawn_moves(list:&mut Vec<Move>, color: Color, from:Square, moves:BitSet) {
    for to in moves.iter() {
        if to.rank() == 7 || to.rank() == 1 {
            for &p in [Queen, Rook, Bishop, Knight].iter() {
                list.push(OrdinalMove (OrdinalMoveInfo{
                        from:from, 
                        to:to, 
                        piece: Piece(Pawn, color),
                        promotion: Some(p)  
                    }))
            }
        } else {
            list.push(squares_to_move(Pawn, color, from, to));
        }
    }
}

#[inline]
fn squares_to_move(kind:Kind, color: Color, from:Square, to:Square) -> Move {
    OrdinalMove (OrdinalMoveInfo{
        from:from, 
        to:to, 
        piece: Piece(kind, color),
        promotion: None  
    })
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