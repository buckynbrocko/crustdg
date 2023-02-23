use super::subcode::SubCodeData;
use crate::bitmask::functions::BitMaskFunctions;

#[allow(unused)]
fn some_string() -> String {
    String::from("some text")
}

#[derive(Debug, Clone, Copy)]
pub struct TileCommandData {
    pub color_0: u8,
    pub color_1: u8,
    pub tile_row: u8,
    pub tile_column: u8,
    pub tile_pixels: [u8; 12],
}

impl From<SubCodeData> for TileCommandData {
    fn from(data: SubCodeData) -> TileCommandData {
        TileCommandData {
            color_0: data[0].lower_nybble(),
            color_1: data[1].lower_nybble(),
            tile_row: data[2].last_5_bits(),
            tile_column: data[3].last_6_bits(),
            tile_pixels: TryInto::<[u8; 12]>::try_into(&data[4..])
                .unwrap()
                .map(|b| b.lower_nybble()),
        }
    }
}
