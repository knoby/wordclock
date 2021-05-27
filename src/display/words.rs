#![allow(clippy::clippy::unusual_byte_groupings)]
#![allow(dead_code)]

pub const VOR: (usize, u16) = (3, 0b11100000000_0000_0);
pub const NACH: (usize, u16) = (3, 0b00000001111_0000_0);

pub const ESIST: (usize, u16) = (0, 0b11011100000_0000_0);
pub const UHR: (usize, u16) = (9, 0b00000000111_0000_0);

pub const FUENF: (usize, u16) = (0, 0b00000001111_0000_0);
pub const ZEHN: (usize, u16) = (1, 0b11110000000_0000_0);
pub const VIERTEL: (usize, u16) = (2, 0b00001111111_0000_0);
pub const ZWANZIG: (usize, u16) = (1, 0b00001111111_0000_0);
pub const HALB: (usize, u16) = (4, 0b11110000000_0000_0);
pub const DREIVIERTEL: (usize, u16) = (2, 0b11111111111_0000_0);

pub const EIN_HOUR: (usize, u16) = (5, 0b11100000000_0000_0);
pub const EINS_HOUR: (usize, u16) = (5, 0b11110000000_0000_0);
pub const ZWEI_HOUR: (usize, u16) = (5, 0b00000001111_0000_0);
pub const DREI_HOUR: (usize, u16) = (6, 0b11110000000_0000_0);
pub const VIER_HOUR: (usize, u16) = (6, 0b00000001111_0000_0);
pub const FUENF_HOUR: (usize, u16) = (4, 0b00000001111_0000_0);
pub const SECHS_HOUR: (usize, u16) = (7, 0b11111000000_0000_0);
pub const SIEBEN_HOUR: (usize, u16) = (8, 0b11111100000_0000_0);
pub const ACHT_HOUR: (usize, u16) = (7, 0b00000001111_0000_0);
pub const NEUN_HOUR: (usize, u16) = (9, 0b00011110000_0000_0);
pub const ZEHN_HOUR: (usize, u16) = (9, 0b11110000000_0000_0);
pub const ELF_HOUR: (usize, u16) = (4, 0b00000111000_0000_0);
pub const ZWOELF_HOUR: (usize, u16) = (8, 0b00000111110_0000_0);
