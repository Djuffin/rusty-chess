use bitset::BitSet;
use types::{Square};


struct SquareData {
    //This mask contains ones in the same file as the square.
    file_mask : BitSet,

    //This mask contains ones in the same diagonal as the square. 
    //For example e5 is on the diagonal a1 - h8. (bottom-left -> top-right)
    diagonal_mask : BitSet,

    //This mask contains ones in the same anti-diagonal as the square. 
    //For example e5 is on the anti-diagonal b8 - h2. (top-left -> bottom-right)
    antidiagonal_mask : BitSet
}

static EMPTY_SQ_DATA  : SquareData = SquareData { 
    file_mask: BitSet { bits:0 },
    diagonal_mask: BitSet { bits:0 },
    antidiagonal_mask: BitSet { bits:0 } 
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
                    antidiagonal_mask: BitSet::new(antidiagonal_mask) & (!sq_set)
                };
            }

        }
        file_mask <<= 1;         
    }
} 

fn shift(x:u8, base:uint, n:int) -> u64 {
    let byte:u8 = if n >= 0 {
        x << (n as uint)
    } else {
        x >> ((-n) as uint)
    };
    (byte as u64) << base * 8
}