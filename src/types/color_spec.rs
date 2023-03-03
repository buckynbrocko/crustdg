// use crate::bitmask::*;
use crate::bitmask::functions::BitMaskFunctions;
use crate::types::subcode::SubCodeData;
use hex;

pub type ColorTable = [ColorSpec; 8];

#[derive(Debug, Clone, Copy)]
pub struct ColorSpec {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn all_black() -> ColorTable {
    [ColorSpec::black(); 8]
}

impl ColorSpec {
    pub fn colors(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }

    pub fn as_hex(self) -> String {
        let hexes = hex::encode_upper(self.colors().map(|b| 17 * b));
        format!("#{}", hexes)
    }

    pub fn set_red(mut self, value: u8) -> ColorSpec {
        self.red = value.lower_nybble();
        self
    }

    pub fn set_green(mut self, value: u8) -> ColorSpec {
        self.green = value.lower_nybble();
        self
    }

    pub fn set_blue(mut self, value: u8) -> ColorSpec {
        self.blue = value.lower_nybble();
        self
    }

    pub fn black() -> ColorSpec {
        ColorSpec::from((0, 0, 0))
    }

    pub fn table(data: SubCodeData) -> ColorTable {
        let mut vector: Vec<ColorSpec> = Vec::new();
        for index in 0..data.len() {
            if (index % 2) != 0 {
                let pair = [data[index - 1], data[index]];
                vector.push(ColorSpec::from(pair));
            }
        }
        vector.try_into().unwrap()
    }
}

impl From<(u8, u8, u8)> for ColorSpec {
    fn from(colors: (u8, u8, u8)) -> ColorSpec {
        let (red, green, blue) = colors;
        ColorSpec {
            red: red.lower_nybble(),
            green: green.lower_nybble(),
            blue: blue.lower_nybble(),
        }
    }
}

impl From<[u8; 2]> for ColorSpec {
    fn from(bytes: [u8; 2]) -> ColorSpec {
        let high = bytes[0].last_6_bits();
        let low = bytes[1].last_6_bits();
        ColorSpec {
            red: high.inner_nybble() >> 2,
            green: (high.bitpair_4() << 2) | (low.bitpair_2() >> 4),
            blue: low.lower_nybble(),
        }
    }
}

impl From<u16> for ColorSpec {
    fn from(short: u16) -> ColorSpec {
        short.to_be_bytes().into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0,0],[0; 3])]
    #[case([0xFF,0xFF],[255; 3])]
    #[case([0b_0011_1100,0],[0b_1111,0,0])]
    #[case([0b_0000_0011,0b_0011_0000],[0,0b_1111,0])]
    #[case([0,0b_0000_1111],[0,0,0b_1111])]
    #[case([0b_00_0001_00,0b_00_01_0001],[1; 3])]
    #[case([0b_0010_1010,0b_0010_1010],[0b_1010; 3])]
    #[case([0b_1110_1010,0b_1110_1010],[0b_1010; 3])]
    #[case([0b_11_0011_00,0b_11_00_0000],[0b_0011, 0, 0])]
    fn from_u8(#[case] input: [u8; 2], #[case] expectation: [u8; 3]) {
        let color = ColorSpec::from(input);
        let result = [color.red, color.green, color.blue];
        assert_eq!(result, expectation.map(|b| b.lower_nybble()));
    }
    #[rstest]
    #[case(0, [0; 3])]
    #[case(0xFFFF,[255; 3])]
    #[case(0b_0011_1100__0000_0000,[0b_1111,0,0])]
    #[case(0b_0000_0011__0011_0000,[0,0b_1111,0])]
    #[case(0x000F,[0,0,0b_1111])]
    #[case(0b_00_0001_00__00_01_0001,[1; 3])]
    #[case(0b_0010_1010__0010_1010,[0b_1010; 3])]
    #[case(0b_1110_1010__1110_1010,[0b_1010; 3])]
    #[case(0b_11_0011_00__11_00_0000,[0b_0011, 0, 0])]
    fn from_u16(#[case] input: u16, #[case] expectation: [u8; 3]) {
        let color = ColorSpec::from(input);
        let result = [color.red, color.green, color.blue];
        assert_eq!(result, expectation.map(|b| b.lower_nybble()));
    }
}
