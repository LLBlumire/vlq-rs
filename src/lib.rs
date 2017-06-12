extern crate bit_vec;

use bit_vec::BitVec;

pub struct Vlq {
    pub inner: BitVec<u8>
}

impl Vlq {
    pub fn from_u64(mut n: u64) -> Vlq {
        let mut reversed: BitVec<u8> = BitVec::default();

        while n > 0 {
            for _ in 0..7 {
                reversed.push(n % 2 != 0);
                n = n / 2;
            }
            reversed.push(false);
        }

        let mut inner: BitVec<u8> = BitVec::default();
        for b in reversed.iter().rev() {
            inner.push(b)
        }

        if inner.len != 0 {
            for i in 0..(inner.len() / 8 - 1) {
                inner.set(8 * i, true);
            }
        } else {
            for n in 0..8 {
                inner.push(false);
            }
        }

        Vlq {
            inner: inner
        }
    }
}

macro_rules! testgen {
    ($n:ident, $i:expr, $b:expr) => {
        #[test]
        fn $n() {
            assert_eq!(Vlq::from_u64($i).inner.to_bytes(), $b);
        }
    }
}

testgen!(zero, 0x00, [0x00]);
testgen!(sevenf, 0x7F, [0x7F]);
testgen!(eighty, 0x80, [0x81, 0x00]);
testgen!(twothousand, 0x2000, [0xC0, 0x00]);
testgen!(threefff, 0x3FFF, [0xFF, 0x7F]);
testgen!(fourthousand, 0x4000, [0x81, 0x80, 0x00]);
