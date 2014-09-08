use types::*;
use bitset::BitSet;

//Dirty bit tricks are used in movegen 
//see more here:
//https://chessprogramming.wikispaces.com/Efficient+Generation+of+Sliding+Piece+Attacks
//https://chessprogramming.wikispaces.com/Hyperbola+Quintessence


//generate rook moves on a given board from a given square 
//we don't check that rook is acutally there
pub fn gen_rook_moves(board: Board, sq: Square, color: Color) -> BitSet {
    let occupied_set = board.whites | board.blacks;
    let friendly_color_set = *board.get_color_bitset(color);

    let rank_move_set = gen_rank_sliding_moves(occupied_set, sq);
    let file_move_set = gen_file_sliding_moves(occupied_set, sq);

    (rank_move_set | file_move_set) & !friendly_color_set
}

//generate bishop moves on a given board from a given square 
//we don't check that bishop is acutally there
pub fn gen_bishop_moves(board: Board, sq: Square, color: Color) -> BitSet {
    let occupied_set = board.whites | board.blacks;
    let friendly_color_set = *board.get_color_bitset(color);

    let diag_move_set     = gen_diagonal_sliding_moves(occupied_set, sq);
    let antidiag_move_set = gen_antidiagonal_sliding_moves(occupied_set, sq);

    (diag_move_set | antidiag_move_set) & !friendly_color_set
}

//generate queen moves on a given board from a given square 
//we don't check that queen is acutally there
pub fn gen_queen_moves(board: Board, sq: Square, color: Color) -> BitSet {
    let occupied_set = board.whites | board.blacks;
    let friendly_color_set = *board.get_color_bitset(color);

    let diag_move_set     = gen_diagonal_sliding_moves(occupied_set, sq);
    let antidiag_move_set = gen_antidiagonal_sliding_moves(occupied_set, sq);
    let rank_move_set     = gen_rank_sliding_moves(occupied_set, sq);
    let file_move_set     = gen_file_sliding_moves(occupied_set, sq);

    (diag_move_set | antidiag_move_set | rank_move_set | file_move_set) 
        & !friendly_color_set
}

pub fn gen_king_moves(board: Board, sq: Square, color: Color) -> BitSet {
    use tables::get_king_moves_mask;
    let friendly_color_set = *board.get_color_bitset(color);
    let raw_moves = get_king_moves_mask(sq);
    raw_moves & !friendly_color_set
}

pub fn gen_knight_moves(board: Board, sq: Square, color: Color) -> BitSet {
    use tables::get_knight_moves_mask;
    let friendly_color_set = *board.get_color_bitset(color);
    let raw_moves = get_knight_moves_mask(sq);
    raw_moves & !friendly_color_set
}

#[inline]
fn gen_rank_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
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
    let occupied_set = occupied_set.bits;
    let file_mask = get_file_mask(sq).bits;
    let piece_mask = BitSet::from_one_square(sq).bits;
    let occupied_file = occupied_set & file_mask;
    let ray_up = occupied_file - piece_mask;
    let ray_down = (occupied_file.swap_bytes() - piece_mask.swap_bytes()).swap_bytes();
    let changes = ray_up ^ ray_down;
    BitSet::new(changes & file_mask)
}

#[inline]
fn gen_diagonal_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
    use tables::get_diagonal_mask;
    let occupied_set = occupied_set.bits;
    let diag_mask = get_diagonal_mask(sq).bits;
    let piece_mask = BitSet::from_one_square(sq).bits;
    let occupied_diag = occupied_set & diag_mask;
    let ray_up = occupied_diag - piece_mask;
    let ray_down = (occupied_diag.swap_bytes() - piece_mask.swap_bytes()).swap_bytes();
    let changes = ray_up ^ ray_down;
    BitSet::new(changes & diag_mask)
}

#[inline]
fn gen_antidiagonal_sliding_moves(occupied_set: BitSet, sq:Square) -> BitSet {
    use tables::get_antidiagonal_mask;
    let occupied_set = occupied_set.bits;
    let antidiag_mask = get_antidiagonal_mask(sq).bits;
    let piece_mask = BitSet::from_one_square(sq).bits;
    let occupied_diag = occupied_set & antidiag_mask;
    let ray_up = occupied_diag - piece_mask;
    let ray_down = (occupied_diag.swap_bytes() - piece_mask.swap_bytes()).swap_bytes();
    let changes = ray_up ^ ray_down;
    BitSet::new(changes & antidiag_mask)
}