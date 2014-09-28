use bitset::BitSet;
use types::*;

//data that helps generate move available for different pieces from each square
struct SquareMoveData {
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
    king_moves_mask : BitSet,

    //This mask contains active bits in locations where a pawn 
    //can move (without taking) from this square.
    white_pawn_moves_mask : BitSet,
    black_pawn_moves_mask : BitSet,

    //This mask contains active bits in locations where a pawn 
    //can take from this square.
    white_pawn_attacks_mask : BitSet,
    black_pawn_attacks_mask : BitSet,
}

static EMPTY_SQ_MOVE_DATA  : SquareMoveData = SquareMoveData { 
    file_mask: BitSet { bits:0 },
    diagonal_mask: BitSet { bits:0 },
    antidiagonal_mask: BitSet { bits:0 }, 
    knight_moves_mask: BitSet { bits:0 }, 
    king_moves_mask: BitSet { bits:0 },
    white_pawn_attacks_mask: BitSet { bits:0 },
    white_pawn_moves_mask: BitSet { bits:0 },
    black_pawn_attacks_mask: BitSet { bits:0 },
    black_pawn_moves_mask: BitSet { bits:0 }
};
static mut SQ_MOVE_DATA:[SquareMoveData, ..64] = [ EMPTY_SQ_MOVE_DATA, ..64];


static BYTE_REVERSE:[u8, ..256] = 
[0x0, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0, 
 0x8, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8, 
 0x4, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4, 
 0xC, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC, 
 0x2, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2, 
 0xA, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA, 
 0x6, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6, 
 0xE, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE, 
 0x1, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1, 
 0x9, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9, 
 0x5, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5, 
 0xD, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD, 
 0x3, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3, 
 0xB, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB, 
 0x7, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7, 
 0xF, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF]; 

 static mut RANDOM_NUMBERS:[u64, ..850] = [0, ..850];

//this function reverses bits in a given byte
#[inline]
pub fn reverse(x:u8) -> u8 {
    BYTE_REVERSE[x as uint] 
}

#[inline]
pub fn get_diagonal_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].diagonal_mask
    }
}

#[inline]
pub fn get_antidiagonal_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].antidiagonal_mask
    }
}

#[inline]
pub fn get_file_mask(sq:Square) -> BitSet {
    unsafe {
        let result = SQ_MOVE_DATA[sq.file_and_rank() as uint].file_mask;
        debug_assert!(!result.is_empty(), "file tables are not initialized");
        result
    }
}

#[inline]
pub fn get_knight_moves_mask(sq:Square) -> BitSet {
    unsafe {
        let result = SQ_MOVE_DATA[sq.file_and_rank() as uint].knight_moves_mask;
        debug_assert!(!result.is_empty(), "knight tables are not initialized");
        result

    }
}

#[inline]
pub fn get_king_moves_mask(sq:Square) -> BitSet {
    unsafe {
        let result = SQ_MOVE_DATA[sq.file_and_rank() as uint].king_moves_mask;
        debug_assert!(!result.is_empty(), "king tables are not initialized");
        result
    }
}

#[inline]
pub fn get_white_pawn_moves_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].white_pawn_moves_mask
    }
}

#[inline]
pub fn get_white_pawn_attacks_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].white_pawn_attacks_mask
    }
}

#[inline]
pub fn get_black_pawn_moves_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].black_pawn_moves_mask
    }
}

#[inline]
pub fn get_black_pawn_attacks_mask(sq:Square) -> BitSet {
    unsafe {
        SQ_MOVE_DATA[sq.file_and_rank() as uint].black_pawn_attacks_mask
    }
}

#[inline]
pub fn get_random_number(n:uint) -> u64 {
    unsafe {
        RANDOM_NUMBERS[n]
    }
}


pub fn init_tables() {
    init_move_data();
    init_random_numbers();
}

fn init_random_numbers() {
    use std::rand::{Isaac64Rng, SeedableRng, Rng};
    let seed:&[u64] = &[
        0xe0d639d1dacd02beu64,
        0xef96c10706fd5913u64,
        0x2ecb7a7419590303u64,
        0x109a590b6eeb408cu64,
        0x532c075b28329e05u64,
        0x25bb59f5b8a75bd1u64,
        0x676ace4d6694cd1du64
    ];
    let mut generator:Isaac64Rng = SeedableRng::from_seed(seed); 
    unsafe {
        for i in range(0, RANDOM_NUMBERS.len()) {
            RANDOM_NUMBERS[i] = generator.next_u64();
        }
    }
}

fn init_move_data() {
    let mut file_mask = 0x0101010101010101u64; //all active bits on a file where sq belongs
    for file in range(0i, 8) {
        let file_byte = 1u8 << file as uint; //byte with one active bit number file
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
                SQ_MOVE_DATA[sq.file_and_rank()  as uint] = SquareMoveData {
                    file_mask: BitSet::new(file_mask) & (!sq_set),
                    diagonal_mask: BitSet::new(diagonal_mask) & (!sq_set),
                    antidiagonal_mask: BitSet::new(antidiagonal_mask) & (!sq_set),
                    king_moves_mask: gen_king_moves(sq),
                    knight_moves_mask: gen_knight_moves(sq),
                    white_pawn_moves_mask: gen_pawn_moves(sq, White),
                    white_pawn_attacks_mask: gen_pawn_attacks(sq, White),
                    black_pawn_moves_mask: gen_pawn_moves(sq, Black),
                    black_pawn_attacks_mask: gen_pawn_attacks(sq, Black),
                };
            }

        }
        file_mask <<= 1;         
    }
} 

fn gen_pawn_moves(sq:Square, color:Color) -> BitSet {
    let sq_set = BitSet::from_one_square(sq);
    if color == White {
        if sq.rank() == 1 {
            sq_set << 8 | sq_set << 16
        } else {
            sq_set << 8
        }
    } else {
        if sq.rank() == 6 {
            sq_set >> 8 | sq_set >> 16
        } else {
            sq_set >> 8
        }        
    }
}

fn gen_pawn_attacks(sq:Square, color:Color) -> BitSet {
    let sq_set = BitSet::from_one_square(sq);
    if color == White {
        clear_files(FILE_A, sq_set << 8 << 1) | 
        clear_files(FILE_H, sq_set << 8 >> 1)
    } else {
        clear_files(FILE_A, sq_set >> 8 << 1) | 
        clear_files(FILE_H, sq_set >> 8 >> 1)
    }
}


fn gen_knight_moves(sq:Square) -> BitSet {
    let sq_set  = BitSet::from_one_square(sq);
    let result = clear_files(FILE_A,  sq_set << 8 << 8 << 1) //north-north-east
               | clear_files(FILE_AB, sq_set << 8 << 1 << 1) //east-north-east
               | clear_files(FILE_AB, sq_set >> 8 << 1 << 1) //east-south-east
               | clear_files(FILE_A,  sq_set >> 8 >> 8 << 1) //south-south-east
               | clear_files(FILE_H,  sq_set << 8 << 8 >> 1) //north-north-west
               | clear_files(FILE_GH, sq_set << 8 >> 1 >> 1) //west-north-west
               | clear_files(FILE_GH, sq_set >> 8 >> 1 >> 1) //west-south-west
               | clear_files(FILE_H,  sq_set >> 8 >> 8 >> 1);//south-south-west
    result
}

fn gen_king_moves(sq:Square) -> BitSet {
    let sq_set = BitSet::from_one_square(sq);
    let result = (sq_set << 8) //north
               | (sq_set >> 8) //south
               | clear_files(FILE_A, sq_set << 1) //east
               | clear_files(FILE_H, sq_set >> 1) //west
               | clear_files(FILE_A, sq_set << 8 << 1) //north-east
               | clear_files(FILE_A, sq_set >> 8 << 1) //south-east
               | clear_files(FILE_H, sq_set << 8 >> 1) //north-west
               | clear_files(FILE_H, sq_set >> 8 >> 1);//south-west
    result
}


static FILE_A:u64  = 0b00000001;
static FILE_H:u64  = 0b10000000;
static FILE_AB:u64 = 0b00000011;
static FILE_GH:u64 = 0b11000000;

fn clear_files(mask:u64, b:BitSet) -> BitSet {
    let mut mask64 = 0u64;
    for _ in range(0i, 8) {
        mask64 <<= 8;
        mask64 |= mask;
    }
    b & BitSet::new(!mask64)
}

fn shift(x:u8, base:uint, n:int) -> u64 {
    let byte = if n >= 0 {
        x << (n as uint)
    } else {
        x >> ((-n) as uint)
    };
    (byte as u64) << base * 8
}