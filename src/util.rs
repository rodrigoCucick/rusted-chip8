// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod utilities {
    pub struct Math2d;

    impl Math2d {
        pub fn wrap_coord(axis: u8, win_size: u32) -> u8 {
            if axis as u32 > win_size - 1 { axis % win_size as u8 } else { axis }
        }
    }

    pub struct BitManipulator;

    impl BitManipulator {
        pub fn make_16bit_addr_from_nibbles(second_nibble: u8, third_nibble: u8, fourth_nibble: u8) -> u16 {
            (second_nibble as u16) << 8 | (third_nibble as u16) << 4 | (fourth_nibble as u16)
        }

        pub fn make_16bit_instr_from_bytes(first_byte: u8, second_byte: u8) -> u16 {
            (first_byte as u16) << 8 | (second_byte as u16)
        }
    }
}
