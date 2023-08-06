// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod keyboard {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::EventPump;

    pub struct Keyboard {
        key_arr: [u8;16],
    }

    impl Keyboard {
        pub fn new() -> Self {
            Self { key_arr: [0;16], }
        }
    }

    pub struct KeyboardController {
        keyboard: Keyboard,
    }

    impl KeyboardController {
        pub fn new(keyboard: Keyboard) -> Self {
            Self { keyboard, }
        }

        pub fn check_input_events(&mut self, event_pump: &mut EventPump) -> Option<CustomKeyEvent> {
            self.reset_state();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Some(CustomKeyEvent::Quit),
                    _ => continue
                }
            }

            let pressed_keys: Vec<Keycode> = event_pump.keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            for pressed_key in pressed_keys {
                match pressed_key {
                    Keycode::Num1 => self.set_key_down_by_index(1),
                    Keycode::Num2 => self.set_key_down_by_index(2),
                    Keycode::Num3 => self.set_key_down_by_index(3),
                    Keycode::Num4 => self.set_key_down_by_index(0xc),
                    Keycode::Q =>    self.set_key_down_by_index(4),
                    Keycode::W =>    self.set_key_down_by_index(5),
                    Keycode::E =>    self.set_key_down_by_index(6),
                    Keycode::R =>    self.set_key_down_by_index(0xd),
                    Keycode::A =>    self.set_key_down_by_index(7),
                    Keycode::S =>    self.set_key_down_by_index(8),
                    Keycode::D =>    self.set_key_down_by_index(9),
                    Keycode::F =>    self.set_key_down_by_index(0xe),
                    Keycode::Z =>    self.set_key_down_by_index(0xa),
                    Keycode::X =>    self.set_key_down_by_index(0),
                    Keycode::C =>    self.set_key_down_by_index(0xb),
                    Keycode::V =>    self.set_key_down_by_index(0xf),
                    _ => continue
                }
            }
            
            None
        }

        pub fn is_key_x_pressed(&mut self, key_index: u8) -> bool {
            self.keyboard.key_arr[key_index as usize] == 1
        }

        pub fn get_any_key_down(&self) -> Option<u8> {
            for (i, key) in self.keyboard.key_arr.iter().enumerate() {
                if *key == 1 {
                    return Some(i as u8)
                }
            }
            None
        }

        pub fn set_key_down_by_index(&mut self, key_index: usize) {
            self.keyboard.key_arr[key_index] = 1;
        }

        fn reset_state(&mut self) {
            self.keyboard.key_arr = [0;16];
        }
    }

    pub enum CustomKeyEvent {
        Quit,
    }
}
