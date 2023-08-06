// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod settings {
    use sdl2::pixels::Color;
    use std::fs::File;
    use std::io::Read;

    pub struct EmuSettings {
        scale: u32,
        cycles_per_frame: u32,
        bg_color: Color,
        pixel_color: Color,
        default_ch8_folder: String,
        st_equals_buzzer: bool,
        bit_shift_instructions_use_vy: bool,
        store_read_instructions_change_i: bool,
    }

    impl EmuSettings {
        pub fn new() -> Self {
            let mut emu_settings = EmuSettings::new_default();

            match File::open("config.txt") {
                Ok(mut file) => {
                    let mut file_str = String::new();
                    if let Err(_) = file.read_to_string(&mut file_str) {
                        return EmuSettings::log_invalid_use_default();
                    }

                    let lines: Vec<&str> = file_str.split('\n').collect();
                    for line in lines {
                        if line.starts_with("#") { continue; }

                        if !line.contains("=") { return EmuSettings::log_invalid_use_default(); }

                        let full_setting: Vec<&str> = line.split('=').collect();
                        if full_setting.len() > 2 { return EmuSettings::log_invalid_use_default(); }

                        let setting_name: &str = full_setting.first().unwrap();
                        let setting_val: &str = full_setting.last().unwrap();

                        // Single values.
                        if !setting_val.contains(",") {
                            match setting_name {
                                "scale" =>
                                    emu_settings.scale = EmuSettings::parse_u32(setting_val, 1, 20, 10),
                                "cycles_per_frame" =>
                                    emu_settings.cycles_per_frame = EmuSettings::parse_u32(setting_val, 1, 99999, 20),
                                "default_ch8_folder" =>
                                    emu_settings.default_ch8_folder = String::from(setting_val.trim()),
                                "st_equals_buzzer" =>
                                    emu_settings.st_equals_buzzer = EmuSettings::parse_bool(setting_val, false),
                                "bit_shift_instructions_use_vy" =>
                                    emu_settings.bit_shift_instructions_use_vy = EmuSettings::parse_bool(setting_val, true),
                                "store_read_instructions_change_i" =>
                                    emu_settings.store_read_instructions_change_i = EmuSettings::parse_bool(setting_val, true),
                                _ => return EmuSettings::log_invalid_use_default()
                            }
                        // CSV (color).
                        } else {
                            let rgb_vals: Vec<&str> = setting_val.split(',').collect();
                            if rgb_vals.len() != 3 {
                                return EmuSettings::log_invalid_use_default();
                            }

                            match setting_name {
                                "bg_color" =>
                                    emu_settings.bg_color = Color::RGB(
                                        EmuSettings::parse_color(rgb_vals.get(0).unwrap(), false),
                                        EmuSettings::parse_color(rgb_vals.get(1).unwrap(), false),
                                        EmuSettings::parse_color(rgb_vals.get(2).unwrap(), false)),
                                "pixel_color" =>
                                    emu_settings.pixel_color = Color::RGB(
                                        EmuSettings::parse_color(rgb_vals.get(0).unwrap(), true),
                                        EmuSettings::parse_color(rgb_vals.get(1).unwrap(), true),
                                        EmuSettings::parse_color(rgb_vals.get(2).unwrap(), true)),
                                _ => return EmuSettings::log_invalid_use_default()
                            }
                        }
                    }
                },
                Err(_) => {
                    println!("Couldn't locate the file 'config.txt'! Default values will be used.");
                    return EmuSettings::new_default();
                }
            };

            emu_settings.default_ch8_folder.insert_str(0, "\\");
            emu_settings.default_ch8_folder.insert_str(emu_settings.get_default_ch8_folder().len(), "\\");
            emu_settings
        }

        pub fn get_bg_color(&self) -> Color {
            self.bg_color
        }

        pub fn get_cycles_per_frame(&self) -> u32 {
            self.cycles_per_frame
        }

        pub fn get_default_ch8_folder(&self) -> &str {
            &self.default_ch8_folder
        }

        pub fn get_pixel_color(&self) -> Color {
            self.pixel_color
        }
        
        pub fn get_scale(&self) -> u32 {
            self.scale
        }

        pub fn get_st_equals_buzzer(&self) -> bool {
            self.st_equals_buzzer
        }

        pub fn get_bit_shift_instructions_use_vy(&self) -> bool {
            self.bit_shift_instructions_use_vy
        }

        pub fn get_store_read_instructions_change_i(&self) -> bool {
            self.store_read_instructions_change_i
        }

        fn new_default() -> Self {
            Self {
                scale: 10,
                cycles_per_frame: 20,
                bg_color: Color::RGB(0x00, 0x00, 0x00),
                pixel_color: Color::RGB(0xff, 0xff, 0xff),
                default_ch8_folder: String::from("\\ch8\\"),
                st_equals_buzzer: false,
                bit_shift_instructions_use_vy: true,
                store_read_instructions_change_i: true,
            }
        }

        fn parse_u32(
            setting_val: &str,
            min_val_incl: u32,
            max_val_incl: u32,
            default_val: u32) ->u32 {
                
            match setting_val.trim().parse() {
                Ok(parsed_val) =>
                    if parsed_val < min_val_incl || parsed_val > max_val_incl {
                        default_val
                    } else {
                        parsed_val
                    }
                Err(_) => default_val
            }
        }

        fn parse_bool(setting_val: &str, default_val: bool) -> bool {
            match setting_val.trim().parse() {
                Ok(parsed_val) => parsed_val,
                Err(_) => default_val
            }
        }

        fn parse_color(setting_val: &str, is_pixel: bool) -> u8 {
            match setting_val.trim().parse() {
                Ok(parsed_val) => parsed_val,
                Err(_) => if is_pixel { 255 } else { 0 }
            }
        }

        fn log_invalid_use_default() -> EmuSettings {
            println!("Invalid 'config.txt' file! Default values will be used.");
            EmuSettings::new_default()
        }
    }
}
