// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

#![allow(warnings)]
pub mod emulator {
    use std::fs::File;
    use std::io::Read;

    use crate::gfx::graphics::WindowController;
    
    pub struct Memory {
        // Addresses from 0x000 (0) to 0x1ff (511) were originally occupied by the interpreter,
        // which means most common Chip-8 programs start at address 0x200 (512).
        ram: [u8;4096],

        // The stack allows for up to 16 nested subroutines.
        stack: [u16;16],

        // General purpose registers.
        v0: u8,
        v1: u8,
        v2: u8,
        v3: u8,
        v4: u8,
        v5: u8,
        v6: u8,
        v7: u8,
        v8: u8,
        v9: u8,
        va: u8,
        vb: u8,
        vc: u8,
        vd: u8,
        ve: u8,
        vf: u8, // VF is only used as a flag by some Chip-8 instructions, not by programs.

        // Delay timer: If non-zero (activated), it will be decremented by 1 at 60Hz (tied to the screen's refresh rate)
        // until it reaches zero (deactivated).
        dt: u8,

        // Sound timer: It works with the same logic as the delay timer,
        // with the only difference that, when non-zero (active), the buzzer sounds.
        st: u8,

        // Program counter: Stores the memory address of the currently executing instruction.
        pc: u16,

        // Stack pointer: Used to point to the topmost level of the stack.
        sp: u8,

        // I: Used to store memory addresses.
        i: u16,
    }

    impl Memory {
        pub fn new() -> Self {
            Self {
                ram:   [0;4096],
                stack: [0;16],
                v0: 0,
                v1: 0,
                v2: 0,
                v3: 0,
                v4: 0,
                v5: 0,
                v6: 0,
                v7: 0,
                v8: 0,
                v9: 0,
                va: 0,
                vb: 0,
                vc: 0,
                vd: 0,
                ve: 0,
                vf: 0,
                dt: 0,
                st: 0,
                pc: 0x200, // Default initial address for the program counter.
                sp: 0,
                i:  0
            }
        }
        
        pub fn init_ram(&mut self, game_program_path: &str) {
            self.load_game_program(game_program_path);
            self.load_hex_digits_sprites();
        }

        pub fn get_v_register_val_by_nibble_val(&mut self, nibble: u8) -> u8 {
            match nibble {
                0 =>   self.v0,
                1 =>   self.v1,
                2 =>   self.v2,
                3 =>   self.v3,
                4 =>   self.v4,
                5 =>   self.v5,
                6 =>   self.v6,
                7 =>   self.v7,
                8 =>   self.v8,
                9 =>   self.v9,
                0xa => self.va,
                0xb => self.vb,
                0xc => self.vc,
                0xd => self.vd,
                0xe => self.ve,
                _ => 0
            }
        }

        pub fn set_v_register_val_by_nibble_val(&mut self, nibble: u8, val: u8) {
            match nibble {
                0 =>   self.v0 = val,
                1 =>   self.v1 = val,
                2 =>   self.v2 = val,
                3 =>   self.v3 = val,
                4 =>   self.v4 = val,
                5 =>   self.v5 = val,
                6 =>   self.v6 = val,
                7 =>   self.v7 = val,
                8 =>   self.v8 = val,
                9 =>   self.v9 = val,
                0xa => self.va = val,
                0xb => self.vb = val,
                0xc => self.vc = val,
                0xd => self.vd = val,
                0xe => self.ve = val,
                _ => ()
            }
        }

        // For debugging purposes, prints to the standard output only.
        pub fn hex_dump_ram(&self) {
            for (i, byte) in self.ram.iter().enumerate() {
                print!("{i:#05x}: {byte:#04x}\t");
                if (i + 1) % 8 == 0 {
                    println!();
                }
            }
        }

        // Loads the default hex sprites (digits 0 to f) into memory starting at address 0,
        // with each bit of the byte representing the state of a pixel (ON/OFF).
        //
        // Example - The full sprite representation of the number 0 is composed of the following:
        // 1st byte -> 11110000
        // 2nd byte -> 10010000
        // 3rd byte -> 10010000
        // 4th byte -> 10010000
        // 5th byte -> 11110000
        fn load_hex_digits_sprites(&mut self) {
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 0, sprite_data:  [0xF0, 0x90, 0x90, 0x90, 0xF0] }); // 0
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 5, sprite_data:  [0x20, 0x60, 0x20, 0x20, 0x70] }); // 1
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 10, sprite_data: [0xF0, 0x10, 0xF0, 0x80, 0xF0] }); // 2
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 15, sprite_data: [0xF0, 0x10, 0xF0, 0x10, 0xF0] }); // 3
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 20, sprite_data: [0x90, 0x90, 0xF0, 0x10, 0x10] }); // 4
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 25, sprite_data: [0xF0, 0x80, 0xF0, 0x10, 0xF0] }); // 5
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 30, sprite_data: [0xF0, 0x80, 0xF0, 0x90, 0xF0] }); // 6
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 35, sprite_data: [0xF0, 0x10, 0x20, 0x40, 0x40] }); // 7
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 40, sprite_data: [0xF0, 0x90, 0xF0, 0x90, 0xF0] }); // 8
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 45, sprite_data: [0xF0, 0x90, 0xF0, 0x10, 0xF0] }); // 9
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 50, sprite_data: [0xF0, 0x90, 0xF0, 0x90, 0x90] }); // a
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 55, sprite_data: [0xE0, 0x90, 0xE0, 0x90, 0xE0] }); // b
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 60, sprite_data: [0xF0, 0x80, 0x80, 0x80, 0xF0] }); // c
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 65, sprite_data: [0xE0, 0x90, 0x90, 0x90, 0xE0] }); // d
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 70, sprite_data: [0xF0, 0x80, 0xF0, 0x80, 0xF0] }); // e
            self.put_hex_sprite(SpriteFiveBytes { starting_index: 75, sprite_data: [0xF0, 0x80, 0xF0, 0x80, 0x80] }); // f
        }

        fn put_hex_sprite(&mut self, sprite_five_bytes: SpriteFiveBytes) {
            for (i, &byte) in sprite_five_bytes.sprite_data.iter().enumerate() {
                self.ram[sprite_five_bytes.starting_index + (i as usize)] = byte;
            }
        }

        fn load_game_program(&mut self, path: &str) {
            let mut byte_vec = Vec::new();
            File::open(path)
                .unwrap()
                .read_to_end(&mut byte_vec)
                .unwrap();

            // TODO: Validate if the game program fits into memory.
            let mut address: usize = 0x200;
            for byte in byte_vec {
                self.ram[address] = byte;
                address += 1;
            }
        }
    }

    struct SpriteFiveBytes {
        starting_index: usize,
        sprite_data: [u8;5],
    }

    pub struct Cpu;

    impl Cpu {
        pub fn execute_next_instruction(mem: &mut Memory, window_controller: &mut WindowController) {
            // Fragments the whole instruction into 4 nibbles, for easier manipulation.
            let first_nibble  = mem.ram[mem.pc as usize] >> 4;
            let second_nibble = mem.ram[mem.pc as usize] & 0b0000_1111;
            let third_nibble  = mem.ram[(mem.pc + 1) as usize] >> 4;
            let fourth_nibble = mem.ram[(mem.pc + 1) as usize] & 0b0000_1111;

            let first_byte = mem.ram[mem.pc as usize];
            let second_byte = mem.ram[(mem.pc + 1) as usize];

            // Dxyn - DRW Vx, Vy, nibble.
            if first_nibble == 0xd {  
                mem.vf = 0; 

                // fourth_nibble (Dxy(n)) specifies the size of the sprite.
                for byte_i in 0..fourth_nibble {
                    let sprite_byte = mem.ram[(mem.i + byte_i as u16) as usize]; // Load is based on the address stored on the register i.

                    let mut curr_bit = 0;
                    for rev_bit_i in (0..8).rev() {
                        // Only draws bits that are equal to 1, from most significant to less significant.
                        if (sprite_byte >> rev_bit_i & 1) == 1 {
                            window_controller.put_pixel(
                                mem.get_v_register_val_by_nibble_val(second_nibble).wrapping_add(curr_bit), 
                                mem.get_v_register_val_by_nibble_val(third_nibble).wrapping_add(byte_i),
                                &mut mem.vf); // If there was a collision between pixels, vf is set to 1.
                        }
                        curr_bit += 1;
                    }
                }
            // 7xkk - ADD Vx, byte.
            } else if first_nibble == 0x7 {
                let curr_vx_val = mem.get_v_register_val_by_nibble_val(second_nibble);
                mem.set_v_register_val_by_nibble_val(second_nibble, curr_vx_val.wrapping_add(second_byte));
            // 4xkk - SNE Vx, byte
            } else if first_nibble == 0x4 {
                if mem.get_v_register_val_by_nibble_val(second_nibble) != second_byte {
                    mem.pc += 2;
                }
            // 6xkk - LD Vx, byte
            } else if first_nibble == 0x6 {
                mem.set_v_register_val_by_nibble_val(second_nibble, second_byte);
            // 1nnn - JP addr
            } else if first_nibble == 0x1 {
                mem.pc = (second_nibble as u16) << 8 | (third_nibble as u16) << 4 | (fourth_nibble as u16);
                return;
            } 

            // The program counter is incremented by 2 because all instructions are 2 bytes
            // and the ram stores 1 byte values only.
            mem.pc += 2; 
        }
    }
}
