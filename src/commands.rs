use crate::bitmask::BitMaskFunctions;

use crate::types::color_spec::ColorSpec;
use crate::types::scroll_command_data::ScrollCommandData;
use crate::types::subcode::SubCodeData;
use crate::types::subcode::SubCodePacket;
use crate::types::tile_command_data::TileCommandData;
use crate::types::SUBCODE_CDG_COMMAND_MARKER;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Command {
    MemoryPreset { color: u8, repeat: bool },
    BorderPreset { color: u8 },
    TileBlock { data: TileCommandData },
    TileBlockXOR { data: TileCommandData },
    ScrollPreset { data: ScrollCommandData },
    ScrollCopy { data: ScrollCommandData },
    DefineTransparentColor { color: u8 },
    LoadColorTableLow { colors: [ColorSpec; 8] },
    LoadColorTableHigh { colors: [ColorSpec; 8] },
}
impl Command {
    fn memory_preset(data: SubCodeData) -> Command {
        Self::MemoryPreset {
            color: data[0].lower_nybble(),
            repeat: data[1].lower_nybble() != 0,
        }
    }
    fn border_preset(data: SubCodeData) -> Command {
        Command::BorderPreset {
            color: data[0].lower_nybble(),
        }
    }

    fn tile_block(data: SubCodeData) -> Command {
        Command::TileBlock { data: data.into() }
    }
    fn tile_block_xor(data: SubCodeData) -> Command {
        Command::TileBlockXOR { data: data.into() }
    }

    fn scroll_preset(data: SubCodeData) -> Result<Command, String> {
        Ok(Command::ScrollPreset {
            data: ScrollCommandData::try_from(data)?,
        })
    }
    fn scroll_copy(data: SubCodeData) -> Result<Command, String> {
        Ok(Command::ScrollCopy {
            data: ScrollCommandData::try_from(data)?,
        })
    }

    fn define_transparent_color(data: SubCodeData) -> Command {
        Command::DefineTransparentColor {
            color: data[0].lower_nybble(),
        }
    }
    fn load_color_table_low(data: SubCodeData) -> Command {
        Command::LoadColorTableLow {
            colors: ColorSpec::table(data),
        }
    }
    fn load_color_table_high(data: SubCodeData) -> Command {
        Command::LoadColorTableHigh {
            colors: ColorSpec::table(data),
        }
    }
}
impl TryFrom<SubCodePacket> for Command {
    type Error = String;
    fn try_from(packet: SubCodePacket) -> Result<Command, String> {
        let command = packet[0].last_6_bits();
        if command != SUBCODE_CDG_COMMAND_MARKER {
            return Err("".to_string());
        }

        let instruction = packet[1].last_6_bits();
        let function = match instruction {
            1 => Command::memory_preset,
            2 => Command::border_preset,
            6 => Command::tile_block,
            28 => Command::define_transparent_color,
            30 => Command::load_color_table_low,
            31 => Command::load_color_table_high,
            38 => Command::tile_block_xor,
            20 | 24 => {
                let data: [u8; 16] = packet[4..=20].try_into().unwrap();
                match instruction {
                    20 => return Ok(Command::scroll_preset(data)?),
                    24 => return Ok(Command::scroll_copy(data)?),
                    _ => return Err("".to_string()),
                };
            },
            _ => return Err("".to_string()),
        };

        let data: [u8; 16] = packet[4..20].try_into().unwrap();
        Ok(function(data))
    }
}
