/*   Matrix that describes the word on the clock
 0 01234567890
 0 ESKISTLFUNF
 1 ZEHNZWANZIG
 2 DREIVIERTEL
 3 TGNACHVORJM
 4 HALBQZWOLFP
 5 ZWEINSIEBEN
 6 KDREIRHFUNF
 7 ELFNEUNVIER
 8 WACHTZEHNRS
 9 BSECHSFMUHR
*/

#![allow(clippy::clippy::unusual_byte_groupings)]

const VOR: (usize, u16) = (3, 0b0000001110_0000_0);
const NACH: (usize, u16) = (3, 0b00000001111_0000_0);

const ESIST: (usize, u16) = (0, 0b11011100000_0000_0);
const UHR: (usize, u16) = (9, 0b00000000111_0000_0);

const FUENF: (usize, u16) = (0, 0b00000001111_0000_0);
const ZEHN: (usize, u16) = (1, 0b11110000000_0000_0);
const VIERTEL: (usize, u16) = (2, 0b00001111111_0000_0);
const ZWANZIG: (usize, u16) = (1, 0b00001111111_0000_0);
const HALB: (usize, u16) = (4, 0b11110000000_0000_0);
const DREIVIERTEL: (usize, u16) = (2, 0b11111111111_0000_0);

const EIN_HOURE: (usize, u16) = (5, 0b0011100000_0000_0);
const EINS_HOURE: (usize, u16) = (5, 0b0011110000_0000_0);
const ZWEI_HOURE: (usize, u16) = (5, 0b1111000000_0000_0);
const DREI_HOURE: (usize, u16) = (6, 0b01111000000_0000_0);
const VIER_HOURE: (usize, u16) = (6, 0b00000001111_0000_0);
const FUENF_HOURE: (usize, u16) = (6, 0b00000001111_0000_0);
const SECHS_HOURE: (usize, u16) = (9, 0b01111100000_0000_0);
const SIEBEN_HOURE: (usize, u16) = (5, 0b00000111111_0000_0);
const ACHT_HOURE: (usize, u16) = (8, 0b0111100000_0000_1);
const NEUN_HOURE: (usize, u16) = (7, 0b00011110000_0000_0);
const ZEHN_HOURE: (usize, u16) = (8, 0b00000111100_0000_0);
const ELF_HOURE: (usize, u16) = (4, 0b1110000000_0000_0);
const ZWOELF_HOURE: (usize, u16) = (8, 0b00000111110_0000_0);
