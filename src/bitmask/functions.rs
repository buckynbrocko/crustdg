use crate::bitmask::constants::*;

#[allow(non_camel_case_types)]
pub trait BitMaskFunctions {
    fn upper_nybble(self: Self) -> Self;
    fn lower_nybble(self: Self) -> Self;
    fn inner_nybble(self: Self) -> Self;

    fn first_bitpair(self: Self) -> Self;
    fn last_bitpair(self: Self) -> Self;

    fn bitpair_1(self: Self) -> Self;
    fn bitpair_2(self: Self) -> Self;
    fn bitpair_3(self: Self) -> Self;
    fn bitpair_4(self: Self) -> Self;

    fn first_bit(self: Self) -> Self;
    fn first_1_bits(self: Self) -> Self;
    fn first_2_bits(self: Self) -> Self;
    fn first_3_bits(self: Self) -> Self;
    fn first_4_bits(self: Self) -> Self;
    fn first_5_bits(self: Self) -> Self;
    fn first_6_bits(self: Self) -> Self;
    fn first_7_bits(self: Self) -> Self;

    fn last_bit(self: Self) -> Self;
    fn last_1_bits(self: Self) -> Self;
    fn last_2_bits(self: Self) -> Self;
    fn last_3_bits(self: Self) -> Self;
    fn last_4_bits(self: Self) -> Self;
    fn last_5_bits(self: Self) -> Self;
    fn last_6_bits(self: Self) -> Self;
    fn last_7_bits(self: Self) -> Self;
}

impl BitMaskFunctions for u8 {
    fn upper_nybble(self) -> u8 {
        self & UPPER
    }
    fn lower_nybble(self) -> u8 {
        self & LOWER
    }
    fn inner_nybble(self) -> u8 {
        self & INNER_4
    }

    fn first_bitpair(self) -> u8 {
        self & PAIR_1
    }
    fn last_bitpair(self) -> u8 {
        self & PAIR_4
    }

    fn bitpair_1(self) -> u8 {
        self & PAIR_1
    }
    fn bitpair_2(self) -> u8 {
        self & PAIR_2
    }
    fn bitpair_3(self) -> u8 {
        self & PAIR_3
    }
    fn bitpair_4(self) -> u8 {
        self & PAIR_4
    }

    fn first_bit(self) -> u8 {
        self & FIRST_1
    }
    fn first_1_bits(self) -> u8 {
        self & FIRST_1
    }
    fn first_2_bits(self) -> u8 {
        self & FIRST_2
    }
    fn first_3_bits(self) -> u8 {
        self & FIRST_3
    }
    fn first_4_bits(self) -> u8 {
        self & FIRST_4
    }
    fn first_5_bits(self) -> u8 {
        self & FIRST_5
    }
    fn first_6_bits(self) -> u8 {
        self & FIRST_6
    }
    fn first_7_bits(self) -> u8 {
        self & FIRST_7
    }

    fn last_bit(self) -> u8 {
        self & LAST_1
    }
    fn last_1_bits(self) -> u8 {
        self & LAST_1
    }
    fn last_2_bits(self) -> u8 {
        self & LAST_2
    }
    fn last_3_bits(self) -> u8 {
        self & LAST_3
    }
    fn last_4_bits(self) -> u8 {
        self & LAST_4
    }
    fn last_5_bits(self) -> u8 {
        self & LAST_5
    }
    fn last_6_bits(self) -> u8 {
        self & LAST_6
    }
    fn last_7_bits(self) -> u8 {
        self & LAST_7
    }
}
