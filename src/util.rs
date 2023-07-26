// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod utilities {
    pub struct Math2d;

    impl Math2d {
        pub fn wrap_coord(axis: u8, win_size: u32) -> u8 {
            if axis as u32 > win_size - 1 { axis % win_size as u8 } else { axis }
        }
    }

    pub struct Bit;

    impl Bit {
        pub fn make_16bit_addr_from_nibbles(second_nibble: u8, third_nibble: u8, fourth_nibble: u8) -> u16 {
            (second_nibble as u16) << 8 | (third_nibble as u16) << 4 | (fourth_nibble as u16)
        }

        pub fn make_16bit_instr_from_bytes(first_byte: u8, second_byte: u8) -> u16 {
            (first_byte as u16) << 8 | (second_byte as u16)
        }

        pub fn decimal_to_8bit_bcd_tuple(decimal: u8) -> (u8, u8, u8) {
            let mut hundreds: u8 = 0;
            let mut tens: u8 = 0;
            let mut ones: u8 = 0;

            if decimal >= 100 {
                hundreds = decimal / 100;
                let hundreds_mod = decimal % 100;
                tens = hundreds_mod / 10;
                let tens_mod = hundreds_mod % 10;
                ones = tens_mod / 1;
            } else if decimal >= 10 && decimal <= 99 {
                tens = decimal / 10;
                let tens_mod = decimal % 10;
                ones = tens_mod / 1;
            } else if decimal <= 9 {
                ones = decimal;
            }

            (hundreds, tens, ones)
        }
    }

    pub struct Logic;

    impl Logic {
        pub fn bool_to_u8(exp: bool) -> u8 {
            match exp {
                true  => 1,
                false => 0
            }
        }
    }
}
