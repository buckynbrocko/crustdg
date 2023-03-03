mod bitmask;
#[allow(unused)]
mod commands;
pub mod syntactical;
mod types;

pub mod prelude {
    pub use crate::bitmask::BitMask;
    #[allow(unused)]
    use crate::bitmask::*;
    pub use crate::commands::Command;
    #[allow(unused)]
    use crate::commands::*;
    use crate::types::*;

    pub use subcode::data_pairs;
    pub use subcode::SubCodeData;
    pub use subcode::SubCodePacket;
    pub use subcode::SubCodeStruct;
    pub use subcode::SUBCODE_CDG_COMMAND_MARKER;

    pub use color_spec::ColorSpec;

    pub use scroll_command_data::HorizontalScroll;
    pub use scroll_command_data::HorizontalScrollDirection;
    pub use scroll_command_data::ScrollCommandData;
    pub use scroll_command_data::VerticalScroll;
    pub use scroll_command_data::VerticalScrollDirection;

    pub use tile_command_data::TileCommandData;
}

#[allow(non_snake_case)]
pub mod CrustDG {
    #[allow(unused)]
    use super::prelude::*;
}
