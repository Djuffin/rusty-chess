use std::fmt;
use types::Square;
use std::ops::{BitAnd, BitOr, BitXor, Not, Add, Sub, Shl, Shr};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BitSet {
    pub bits:u64
}

#[derive(Clone, Copy)]
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
    pub fn swap(self) -> BitSet {
        BitSet { bits: self.bits.swap_bytes() }
    }


    #[inline]
    pub fn from_one_square(sq: Square) -> BitSet {
        BitSet { bits : 1u64 << sq.file_and_rank() as usize }
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
    pub fn get_rank(self, rank:u8) -> u8 {
        (self.bits >> (rank * 8u8) as usize) as u8
    }

    #[inline]
    pub fn iter(self) -> SquareIter {
        SquareIter::new(self.bits)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.bits == 0
    }

    #[inline]
    pub fn count(self) -> usize {
        self.bits.count_ones() as usize
    }

}

impl Iterator for SquareIter {
    type Item = Square;

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
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.bits.count_ones() as usize; 
        (count, Some(count))    
    }  
}

impl SquareIter {
    #[inline]
    fn new(bits: u64) -> SquareIter {
        SquareIter { bits:bits }
    }
  
}

impl BitAnd for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitand(self, rhs: BitSet) -> BitSet {
        BitSet { bits : self.bits & rhs.bits }
    }
}

impl Add for BitSet {
    type Output = BitSet;

    #[inline]
    fn add(self, rhs: BitSet) -> BitSet {
        BitSet { bits :  self.bits.wrapping_add(rhs.bits) }
    }
}

impl Sub for BitSet {
    type Output = BitSet;

    #[inline]
    fn sub(self, rhs: BitSet) -> BitSet {
        BitSet { bits : self.bits.wrapping_sub(rhs.bits) }
    }
}

impl BitOr for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitor(self, rhs: BitSet) -> BitSet {
        BitSet { bits : self.bits | rhs.bits }
    }
}

impl BitXor for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitxor(self, rhs: BitSet) -> BitSet {
        BitSet { bits : self.bits ^ rhs.bits }
    }
}

impl Not for BitSet {
    type Output = BitSet;

    #[inline]
    fn not(self) -> BitSet {
        BitSet { bits : !self.bits }
    }
}

impl Shl<usize> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, rhs: usize) -> BitSet {
        BitSet { bits: self.bits << rhs }
    }
}


impl Shr<usize> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, rhs: usize) -> BitSet {
        BitSet { bits: self.bits >> rhs }
    }
}

impl fmt::Display for BitSet {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8is).rev() {
            for file in 0..8 {
                let sq = Square::new(file as u8, rank as u8);
                let c = if self.get(sq) { '*' } else {'.'};
                try!(write!(f, "{0}", c ));
            }
            try!(write!(f, "{0}", "\n"));
        }
        Ok (())
     }
}

impl fmt::Display for SquareIter {
     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        let cp = *self;
        for sq in cp {
            try!(write!(f, "{0} ", sq));    
        }
        try!(write!(f, "]"));
        Ok (())
     }
}
