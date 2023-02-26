use crate::commands::Command;
use crate::types::color_spec::ColorSpec;
use crate::types::color_spec::ColorTable;

type Row = [u8; 6];
type Tile = [Row; 12];
type DisplayRow = [Tile; 50];

#[derive(Default)]
#[derive(Debug)]
pub struct TODO {}

// #[derive(Default)]
#[derive(Debug)]
struct CDGDisplay {
    color_table_high: ColorTable,
    color_table_low: ColorTable,
    tiles: [DisplayRow; 18],
}

impl Default for CDGDisplay {
    fn default() -> Self {
        CDGDisplay {
            color_table_high: [ColorSpec::black(); 8],
            color_table_low: [ColorSpec::black(); 8],
            tiles: [[[[0; 6]; 12]; 50]; 18],
        }
    }
}

impl CDGDisplay {
    pub fn execute(&mut self, command: Command) {
        match command {
            Command::LoadColorTableHigh { colors } => self.load_color_table_high(colors),
            Command::LoadColorTableLow { colors } => self.load_color_table_low(colors),
            _ => (),
        };
    }
    pub fn load_color_table_high(&mut self, colors: [ColorSpec; 8]) {
        self.color_table_high = colors;
    }
    pub fn load_color_table_low(&mut self, colors: [ColorSpec; 8]) {
        self.color_table_low = colors;
    }
}
