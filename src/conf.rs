// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod config {
    use sdl2::pixels::Color;
    use std::fs::File;
    use std::io::Read;

    pub struct EmulatorConfiguration {
        scale: u32,
        cycles_per_frame: u32,
        bg_color: Color,
        pixel_color: Color,
        default_ch8_folder: String,
        dt_equals_buzzer: bool,
    }

    impl EmulatorConfiguration {
        pub fn new() -> Self {
            let mut emu_config = EmulatorConfiguration::new_default();

            match File::open("config.txt") {
                // Succesfully opened the file.
                Ok(mut file) => {
                    let mut file_str = String::new();

                    // Error while reading the contents of the file.
                    if let Err(_) = file.read_to_string(&mut file_str) {
                        println!("Invalid \"config.txt\" file! It will be overwritten with default values.");
                        // TODO: Overwrite config.txt contents with default values.
                    }

                    let lines: Vec<&str> = file_str.split('\n').collect();
                    for line in lines {
                        if line.starts_with("#") {
                            continue;
                        }

                        let full_config: Vec<&str> = line.split('=').collect();
                        let config_name: &str = full_config.first().unwrap();
                        let config_value: &str = full_config.last().unwrap();

                        // TODO: Handle parsing erros.
                        // Parse single values.
                        if !config_value.contains(",") {
                            match config_name {
                                "scale" => emu_config.scale = config_value.trim().parse().unwrap(),
                                "cycles_per_frame" => emu_config.cycles_per_frame = config_value.trim().parse().unwrap(),
                                "default_ch8_folder" => emu_config.default_ch8_folder = String::from(config_value.trim()),
                                "dt_equals_buzzer" => emu_config.dt_equals_buzzer = config_value.trim().parse().unwrap(),
                                _ => ()
                            }
                        // Parse CSV values.
                        } else {
                            let rgb_vals: Vec<&str> = config_value.split(',').collect();
                            let r: u8 = rgb_vals.get(0).unwrap().trim().parse().unwrap();
                            let g: u8 = rgb_vals.get(1).unwrap().trim().parse().unwrap();
                            let b: u8 = rgb_vals.get(2).unwrap().trim().parse().unwrap();

                            match config_name {
                                "bg_color" => emu_config.bg_color = Color::RGB(r, g, b),
                                "pixel_color" => emu_config.pixel_color = Color::RGB(r, g, b),
                                _ => ()
                            }
                        }
                    }
                },
                // Error while opening the file.
                Err(_) => {
                    println!("Couldn't locate the file \"config.txt\"! A new one will be created with default values.");
                    // TODO: Create new config.txt file with default values.
                }
            };

            emu_config.default_ch8_folder.insert_str(0, "\\");
            emu_config.default_ch8_folder.insert_str(emu_config.get_default_ch8_folder().len(), "\\");
            emu_config
        }
        
        pub fn get_scale(&self) -> u32 {
            self.scale
        }

        pub fn get_cycles_per_frame(&self) -> u32 {
            self.cycles_per_frame
        }

        pub fn get_bg_color(&self) -> Color {
            self.bg_color
        }

        pub fn get_pixel_color(&self) -> Color {
            self.pixel_color
        }

        pub fn get_default_ch8_folder(&self) -> &str {
            &self.default_ch8_folder
        }

        pub fn get_dt_equals_buzzer(&self) -> bool {
            self.dt_equals_buzzer
        }

        fn new_default() -> Self {
            Self {
                scale: 10,
                cycles_per_frame: 20,
                bg_color: Color::RGB(0x00, 0x00, 0x00),
                pixel_color: Color::RGB(0xff, 0xff, 0xff),
                default_ch8_folder: String::from("ch8"),
                dt_equals_buzzer: false
            }
        }
    }
}
