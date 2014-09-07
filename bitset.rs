use std::fmt;
use std::iter::range_step;
use types::Square;

#[deriving(PartialEq, Clone)]
pub struct BitSet {
    pub bits:u64
}

impl BitSet {

    pub fn empty() -> BitSet {
        BitSet { bits: 0 }
    }

    pub fn new(bits:u64) -> BitSet {
        BitSet { bits: bits }
    }

    #[inline]
    pub fn transpose(self) -> BitSet {
        BitSet { bits: ::utils::transpose(self.bits) }
    }

    #[inline]
    pub fn from_one_square(sq: Square) -> BitSet {
        BitSet { bits : 1u64 << sq.file_and_rank() as uint }
    }

    #[inline]
    pub fn get(self, sq: Square) -> bool {
        self.bits & BitSet::from_one_square(sq).bits != 0
    }

    #[inline]
    pub fn set(&mut self, sq: Square, value:bool) {
        let s = BitSet::from_one_square(sq);
        if value {
            self.bits |= s.bits
        } else {
            self.bits &= !s.bits 
        }
    }

    #[inline]
    pub fn get_file(self, file:u8) -> u8 {
        (self.transpose().bits >> (file * 8u8) as uint) as u8
    }

    #[inline]
    pub fn get_rank(self, rank:u8) -> u8 {
        (self.bits >> (rank * 8u8) as uint) as u8
    }

    #[inline]
    pub fn get_active_squares(self) -> Vec<Square> {
        let mut bits = self.bits;
        let size = bits.count_ones();
        let mut result: Vec<Square> = Vec::with_capacity(size as uint);

        for i in range(0, size) {
            let least_sig_bit = bits.trailing_zeros();
            result.push(Square(least_sig_bit as u8));
            bits &= bits - 1; //least significant bit
        }
        result
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.bits == 0
    }

}

impl BitAnd<BitSet, BitSet> for BitSet {
    #[inline]
    fn bitand(&self, rhs: &BitSet) -> BitSet {
        BitSet { bits : self.bits & rhs.bits }
    }
}

impl BitOr<BitSet, BitSet> for BitSet {
    #[inline]
    fn bitor(&self, rhs: &BitSet) -> BitSet {
        BitSet { bits : self.bits | rhs.bits }
    }
}

impl BitXor<BitSet, BitSet> for BitSet {
    #[inline]
    fn bitxor(&self, rhs: &BitSet) -> BitSet {
        BitSet { bits : self.bits ^ rhs.bits }
    }
}

impl Not<BitSet> for BitSet {
    #[inline]
    fn not(&self) -> BitSet {
        BitSet { bits : !self.bits }
    }
}


impl fmt::Show for BitSet {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        for rank in range_step(7i, -1, -1) {
            for file in range(0i, 8) {
                let sq = Square::new(file as u8, rank as u8);
                let c = if self.get(sq) { '*' } else {'.'};
                try!(write!(f, "{0}", c ));
            }
            try!(write!(f, "{0}", "\n"));
        }
        Ok (())
     }
}
