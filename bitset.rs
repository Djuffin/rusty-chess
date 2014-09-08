use std::fmt;
use std::iter::range_step;
use types::Square;

#[deriving(PartialEq)]
pub struct BitSet {
    pub bits:u64
}

pub struct SquareIter {
    bits:u64
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
    pub fn iter(self) -> SquareIter {
        SquareIter::new(self.bits)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.bits == 0
    }

}

impl Iterator<Square> for SquareIter {
    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.bits == 0 {
            None
        } else {
            let least_sig_bit = self.bits.trailing_zeros();
            self.bits &= self.bits - 1;
            Some (Square(least_sig_bit as u8))
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        let count = self.bits.count_ones() as uint; 
        (count, Some(count))    
    }  
}

impl SquareIter {
    #[inline]
    fn new(bits: u64) -> SquareIter {
        SquareIter { bits:bits }
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

impl Shl<uint, BitSet> for BitSet {
    #[inline]
    fn shl(&self, rhs: &uint) -> BitSet {
        BitSet { bits: self.bits << *rhs }
    }
}


impl Shr<uint, BitSet> for BitSet {
    #[inline]
    fn shr(&self, rhs: &uint) -> BitSet {
        BitSet { bits: self.bits >> *rhs }
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

impl fmt::Show for SquareIter {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let list:Vec<Square> = FromIterator::from_iter(*self);
        write!(f, "{0}", list)
     }
}
