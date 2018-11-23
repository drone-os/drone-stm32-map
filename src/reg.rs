//! Memory-mapped register mappings.

pub use drone_cortex_m::map::reg::*;
pub use drone_stm32_map_piece_1::reg::*;
pub use drone_stm32_map_piece_10::reg::*;
pub use drone_stm32_map_piece_11::reg::*;
pub use drone_stm32_map_piece_12::reg::*;
pub use drone_stm32_map_piece_2::reg::*;
pub use drone_stm32_map_piece_3::reg::*;
pub use drone_stm32_map_piece_4::reg::*;
pub use drone_stm32_map_piece_5::reg::*;
pub use drone_stm32_map_piece_6::reg::*;
pub use drone_stm32_map_piece_7::reg::*;
pub use drone_stm32_map_piece_8::reg::*;
pub use drone_stm32_map_piece_9::reg::*;

include!(concat!(env!("OUT_DIR"), "/svd_reg_tokens.rs"));
