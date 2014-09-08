use bitset::BitSet;
use types::{Square};


struct SquareData {
    //This mask contains active bits in the same file as this square.
    //The square itself is 0.
    file_mask : BitSet,

    //This mask contains active bits in the same diagonal as this square. 
    //For example e5 is on the diagonal a1 - h8. (bottom-left -> top-right)
    //The square itself is 0.
    diagonal_mask : BitSet,

    //This mask contains active bits in the same anti-diagonal as this square. 
    //For example e5 is on the anti-diagonal b8 - h2. (top-left -> bottom-right)
    //The square itself is 0.
    antidiagonal_mask : BitSet,

    //This mask contains active bits in locations where a knight can jump from this square.
    knight_moves_mask : BitSet,

    //This mask contains active bits in locations where a king can jump from this square.
    king_moves_mask : BitSet    
}

static EMPTY_SQ_DATA  : SquareData = SquareData { 
    file_mask: BitSet { bits:0 },
    diagonal_mask: BitSet { bits:0 },
    antidiagonal_mask: BitSet { bits:0 }, 
    knight_moves_mask: BitSet { bits:0 }, 
    king_moves_mask: BitSet { bits:0 } 
};
static mut SQ_DATA:[SquareData, ..64] = [ EMPTY_SQ_DATA, ..64];

#[inline]
pub fn get_diagonal_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_DATA[sq.file_and_rank() as uint].diagonal_mask
    }
}

#[inline]
pub fn get_antidiagonal_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_DATA[sq.file_and_rank() as uint].antidiagonal_mask
    }
}

#[inline]
pub fn get_file_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_DATA[sq.file_and_rank() as uint].file_mask
    }
}

#[inline]
pub fn get_knight_moves_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_DATA[sq.file_and_rank() as uint].knight_moves_mask
    }
}

#[inline]
pub fn get_king_moves_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_DATA[sq.file_and_rank() as uint].king_moves_mask
    }
}


pub fn init_square_data() {
    let mut file_mask = 0x0101010101010101u64; //all active bits on a file where sq belongs
    for file in range(0i, 8) {
        let file_byte = file_mask as u8; //byte with one active bit number file
        for rank in range(0i, 8) {
            let sq = Square::new(file as u8, rank as u8);
            let sq_set = BitSet::from_one_square(sq);
            let diagonal_mask = shift(file_byte, 0u, 0 - rank) |
                                shift(file_byte, 1u, 1 - rank) |
                                shift(file_byte, 2u, 2 - rank) |
                                shift(file_byte, 3u, 3 - rank) |
                                shift(file_byte, 4u, 4 - rank) |
                                shift(file_byte, 5u, 5 - rank) |
                                shift(file_byte, 6u, 6 - rank) |
                                shift(file_byte, 7u, 7 - rank);

            let antidiagonal_mask = shift(file_byte, 0u, rank - 0) |
                                    shift(file_byte, 1u, rank - 1) |
                                    shift(file_byte, 2u, rank - 2) |
                                    shift(file_byte, 3u, rank - 3) |
                                    shift(file_byte, 4u, rank - 4) |
                                    shift(file_byte, 5u, rank - 5) |
                                    shift(file_byte, 6u, rank - 6) |
                                    shift(file_byte, 7u, rank - 7);
            
            unsafe {
                SQ_DATA[sq.file_and_rank()  as uint] = SquareData {
                    file_mask: BitSet::new(file_mask) & (!sq_set),
                    diagonal_mask: BitSet::new(diagonal_mask) & (!sq_set),
                    antidiagonal_mask: BitSet::new(antidiagonal_mask) & (!sq_set),
                    king_moves_mask: gen_king_moves(sq),
                    knight_moves_mask: gen_knight_moves(sq)
                };
            }

        }
        file_mask <<= 1;         
    }
} 

#[inline]
fn gen_knight_moves(sq:Square) -> BitSet {
    let sq_set  = BitSet::from_one_square(sq);
    let file_a  = 0b00000001;
    let file_h  = 0b10000000;
    let file_ab = 0b00000011;
    let file_gh = 0b11000000;
    let result = clear_files (file_a,  sq_set << 8 << 8 << 1) //north-north-east
               | clear_files (file_ab, sq_set << 8 << 1 << 1) //east-north-east
               | clear_files (file_ab, sq_set >> 8 << 1 << 1) //east-south-east
               | clear_files (file_a,  sq_set >> 8 >> 8 << 1) //south-south-east
               | clear_files (file_h,  sq_set << 8 << 8 >> 1) //north-north-west
               | clear_files (file_gh, sq_set << 8 >> 1 >> 1) //west-north-west
               | clear_files (file_gh, sq_set >> 8 >> 1 >> 1) //west-south-west
               | clear_files (file_h,  sq_set >> 8 >> 8 >> 1);//south-south-west
    result
}

#[inline]
fn gen_king_moves(sq:Square) -> BitSet {
    let sq_set = BitSet::from_one_square(sq);
    let file_a  = 0b00000001;
    let file_h  = 0b10000000;
    let result = (sq_set << 8) //north
               | (sq_set >> 8) //south
               | clear_files (file_a, sq_set << 1) //east
               | clear_files (file_h, sq_set >> 1) //west
               | clear_files (file_a, sq_set << 8 << 1) //north-east
               | clear_files (file_a, sq_set >> 8 << 1) //south-east
               | clear_files (file_h, sq_set << 8 >> 1) //north-west
               | clear_files (file_h, sq_set >> 8 >> 1);//south-west
    result
}

#[inline]
fn clear_files(mask:u64, b:BitSet) -> BitSet {
    let mut mask64 = 0u64;
    for i in range(0i, 8) {
        mask64 <<= 8;
        mask64 |= mask;
    }
    b & BitSet::new(!mask64)
}

#[inline]
fn shift(x:u8, base:uint, n:int) -> u64 {
    let byte:u8 = if n >= 0 {
        x << (n as uint)
    } else {
        x >> ((-n) as uint)
    };
    (byte as u64) << base * 8
}