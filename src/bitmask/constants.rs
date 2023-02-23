pub type BitMask = u8;

pub const LOWER: BitMask = 0b_0000_1111; // 0x0F
pub const UPPER: BitMask = 0b_1111_0000; // 0xF0

pub const INNER_4: BitMask = 0b_0011_1100; // 0x3C

pub const PAIR_1: BitMask = 0b_1100_0000; // 0xC0
pub const PAIR_2: BitMask = 0b_0011_0000; // 0x30
pub const PAIR_3: BitMask = 0b_0000_1100; // 0x0C
pub const PAIR_4: BitMask = 0b_0000_0011; // 0x03

#[allow(unused)]
pub const FIRST: BitMask = FIRST_1; // 0x80
pub const FIRST_1: BitMask = 0b_1000_0000; // 0x80
pub const FIRST_2: BitMask = 0b_1100_0000; // 0xC0
pub const FIRST_3: BitMask = 0b_1110_0000; // 0xE0
pub const FIRST_4: BitMask = 0b_1111_0000; // 0xF0
pub const FIRST_5: BitMask = 0b_1111_1000; // 0xF8
pub const FIRST_6: BitMask = 0b_1111_1100; // 0xFC
pub const FIRST_7: BitMask = 0b_1111_1110; // 0xFE

#[allow(unused)]
pub const LAST: BitMask = LAST_1;
pub const LAST_1: BitMask = 0b_0000_0001; // 0x01
pub const LAST_2: BitMask = 0b_0000_0011; // 0x03
pub const LAST_3: BitMask = 0b_0000_0111; // 0x07
pub const LAST_4: BitMask = 0b_0000_1111; // 0x0F
pub const LAST_5: BitMask = 0b_0001_1111; // 0x1F
pub const LAST_6: BitMask = 0b_0011_1111; // 0x3F
pub const LAST_7: BitMask = 0b_0111_1111; // 0x7F

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[LOWER, LAST_4, 0x0F])]
    #[case(&[UPPER, FIRST_4, 0xF0])]
    #[case(&[INNER_4, 0x3C])]
    #[case(&[PAIR_1, FIRST_2, 0xC0])]
    #[case(&[PAIR_2, 0x30])]
    #[case(&[PAIR_3, 0x0C])]
    #[case(&[PAIR_4, LAST_2, 0x03])]
    #[case(&[FIRST, FIRST_1, 0x80])]
    #[case(&[FIRST_3, 0xE0])]
    #[case(&[FIRST_5, 0xF8])]
    #[case(&[FIRST_6, 0xFC])]
    #[case(&[FIRST_7, 0xFE])]
    #[case(&[LAST, LAST_1, 0x01])]
    #[case(&[LAST_3, 0x07])]
    #[case(&[LAST_5, 0x1F])]
    #[case(&[LAST_6, 0x3F])]
    #[case(&[LAST_7, 0x7F])]
    fn accuracy(#[case] masks: &[BitMask]) {
        for window in masks.windows(2) {
            let a = &window[0];
            let b = &window[1];
            assert_eq!(a, b)
        }
    }
}
