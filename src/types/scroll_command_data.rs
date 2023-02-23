use super::subcode::SubCodeData;
use crate::bitmask::functions::BitMaskFunctions;

#[derive(Debug, Clone, Copy)]
pub struct ScrollCommandData {
    pub color: u8,
    pub horizontal_scroll: HorizontalScroll,
    pub vertical_scroll: VerticalScroll,
}
#[derive(Debug, Clone, Copy)]

pub struct HorizontalScroll {
    pub direction: HorizontalScrollDirection,
    pub offset: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalScroll {
    pub direction: VerticalScrollDirection,
    pub offset: u8,
}

#[derive(Debug, Clone, Copy)]

pub enum HorizontalScrollDirection {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]

pub enum VerticalScrollDirection {
    None,
    Down,
    Up,
}

impl TryFrom<u8> for HorizontalScroll {
    type Error = String;
    fn try_from(byte: u8) -> Result<HorizontalScroll, String> {
        Ok(HorizontalScroll {
            offset: byte.last_3_bits().min(5),
            direction: match byte.bitpair_2() >> 4 {
                0 => HorizontalScrollDirection::None,
                1 => HorizontalScrollDirection::Right,
                2 => HorizontalScrollDirection::Left,
                b => {
                    return Err(format!(
                        "invalid value for HorizontalScrollDirection: `{}`",
                        b
                    ))
                },
            },
        })
    }
}

impl TryFrom<u8> for VerticalScroll {
    type Error = String;
    fn try_from(byte: u8) -> Result<VerticalScroll, String> {
        Ok(VerticalScroll {
            offset: byte.lower_nybble().min(11),
            direction: match byte.bitpair_2() >> 4 {
                0 => VerticalScrollDirection::None,
                1 => VerticalScrollDirection::Down,
                2 => VerticalScrollDirection::Up,
                b => {
                    return Err(format!(
                        "invalid value for VerticalScrollDirection: `{}`",
                        b
                    ))
                },
            },
        })
    }
}

impl TryFrom<SubCodeData> for ScrollCommandData {
    type Error = String;
    fn try_from(data: SubCodeData) -> Result<ScrollCommandData, String> {
        Ok(ScrollCommandData {
            color: data[0].lower_nybble(),
            horizontal_scroll: data[1].last_6_bits().try_into()?,
            vertical_scroll: data[2].last_6_bits().try_into()?,
        })
    }
}
