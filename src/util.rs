// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod utilities {
    use std::io::Result;

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

    pub struct EmuFile;

    impl EmuFile {
        pub fn get_files_in_directory(path: &str) -> Result<Vec<String>> {
            let entries = std::fs::read_dir(path)?;
        
            let file_names: Vec<String> = entries
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    if path.is_file() {
                        path.file_name()?.to_str().map(|s| s.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        
            Ok(file_names)
        }

        pub fn file_selection_menu(roms: &Vec<String>) -> usize {
            // TODO: Refactor this associated function.
            if roms.len() == 0 {
                panic!("Couldn't find any .ch8 file in the folder provided to 'default_ch8_folder' in 'config.txt'!");
            }

            let mut choice: usize;
            loop {
                let mut contains_ch8 = false;
                println!("Select a file to run:");
                for (i, rom) in roms.iter().enumerate() {
                    if rom.contains(".ch8") {
                        println!("[{i}]: {rom}");
                        contains_ch8 = true;
                    }
                }

                if !contains_ch8 {
                    panic!("Couldn't find any .ch8 file in the folder provided to 'default_ch8_folder' in 'config.txt'!");
                }

                let mut input_str = String::new();
                std::io::stdin().read_line(&mut input_str).unwrap();
            
                choice = match input_str.trim().parse() {
                    Ok(choice) => choice,
                    Err(_) => {
                        println!("Invalid input!");
                        continue;
                    }
                };

                if choice >= roms.len() {
                    println!("Invalid number!");
                    continue;
                }
                break;
            }
            choice
        }
    }
}
