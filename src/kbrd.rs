// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod keyboard {
    pub struct Keyboard {
        key_arr: [u8;16],
    }

    impl Keyboard {
        pub fn new() -> Self {
            Self { key_arr: [0;16] }
        }
    }

    pub struct KeyboardController {
        keyboard: Keyboard,
    }

    impl KeyboardController {
        pub fn new(keyboard: Keyboard) -> Self {
            Self { keyboard }
        }

        pub fn set_key_down_x(&mut self, key_index: usize) {
            self.keyboard.key_arr[key_index] = 1;
        }

        pub fn set_key_up_x(&mut self, key_index: usize) {
            self.keyboard.key_arr[key_index] = 0;
        }

        pub fn is_key_x_pressed(&mut self, key_index: u8) -> bool {
            self.keyboard.key_arr[key_index as usize] == 1
        }
    }
}
