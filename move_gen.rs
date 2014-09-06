use types::*;
use bitset::BitSet;

//generate rook moves on a given board from a given position 
//we don't check that rook is acutally there
//Dirty bit tricks are used. 
//see (Subtraction and Reverse Subtraction of rooks from blockers)
//https://chessprogramming.wikispaces.com/Efficient+Generation+of+Sliding+Piece+Attacks
pub fn gen_rook_moves(board: Board, sq: Square, color: Color) -> BitSet {
    use utils::reverse;
    let occupied_set = board.whites | board.blacks;
    let (rank, file) = (sq.rank(), sq.file());

    let rank_blockers_mask:u8 = occupied_set.get_rank(rank);
    let piece_mask:u8 = 1u8 << file as uint;
    let rank_attack = (rank_blockers_mask - (piece_mask << 1)) ^ 
                      reverse(reverse(rank_blockers_mask) - (reverse(piece_mask) << 1));

    let file_blockers_mask:u8 = occupied_set.get_file(file);
    let piece_mask:u8 = 1u8 << rank as uint;
    let file_attack = (file_blockers_mask - (piece_mask << 1)) ^ 
                      reverse(reverse(file_blockers_mask) - (reverse(piece_mask) << 1));

    let rank_attack_set = BitSet::new( (rank_attack as u64) << (rank * 8u8) as uint );
    let file_attack_set = BitSet::new( (file_attack as u64) << (file * 8u8) as uint ).transpose();

    let friendly_color_set = *board.get_color_bitset(color);

    (rank_attack_set | file_attack_set) & !friendly_color_set
}