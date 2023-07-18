// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

#![allow(warnings)]
pub mod emulator {
    use std::fs::File;
    use std::io::Read;

    use crate::gfx::graphics::CustomWindowController;
    
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
    }

    pub struct MemoryController {
        mem: Memory,
    }

    impl MemoryController {
        pub fn new(mem: Memory) -> Self {
            Self { mem }
        }

        pub fn init_ram(&mut self, game_program_path: &str) {
            self.load_game_program(game_program_path);
            self.load_hex_digits_sprites();
        }

        pub fn get_v_register_val_by_nibble_val(&mut self, nibble: u8) -> u8 {
            match nibble {
                0 =>   self.mem.v0,
                1 =>   self.mem.v1,
                2 =>   self.mem.v2,
                3 =>   self.mem.v3,
                4 =>   self.mem.v4,
                5 =>   self.mem.v5,
                6 =>   self.mem.v6,
                7 =>   self.mem.v7,
                8 =>   self.mem.v8,
                9 =>   self.mem.v9,
                0xa => self.mem.va,
                0xb => self.mem.vb,
                0xc => self.mem.vc,
                0xd => self.mem.vd,
                0xe => self.mem.ve,
                _ => 0
            }
        }

        pub fn set_v_register_val_by_nibble_val(&mut self, nibble: u8, val: u8) {
            match nibble {
                0 =>   self.mem.v0 = val,
                1 =>   self.mem.v1 = val,
                2 =>   self.mem.v2 = val,
                3 =>   self.mem.v3 = val,
                4 =>   self.mem.v4 = val,
                5 =>   self.mem.v5 = val,
                6 =>   self.mem.v6 = val,
                7 =>   self.mem.v7 = val,
                8 =>   self.mem.v8 = val,
                9 =>   self.mem.v9 = val,
                0xa => self.mem.va = val,
                0xb => self.mem.vb = val,
                0xc => self.mem.vc = val,
                0xd => self.mem.vd = val,
                0xe => self.mem.ve = val,
                _ => ()
            }
        }

        // For debugging purposes, prints to the standard output only.
        pub fn hex_dump_ram(&self) {
            for (i, byte) in self.mem.ram.iter().enumerate() {
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
                self.mem.ram[sprite_five_bytes.starting_index + (i as usize)] = byte;
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
                self.mem.ram[address] = byte;
                address += 1;
            }
        }
    }

    struct SpriteFiveBytes {
        starting_index: usize,
        sprite_data: [u8;5],
    }

    pub struct InstructionsController;

    impl InstructionsController {
        pub fn exec_next_instr(mem_ctrl: &mut MemoryController, win_ctrl: &mut CustomWindowController) {
            // Fragments the whole instruction into 4 nibbles, for easier manipulation.
            let first_nibble  = mem_ctrl.mem.ram[mem_ctrl.mem.pc as usize] >> 4;
            let second_nibble = mem_ctrl.mem.ram[mem_ctrl.mem.pc as usize] & 0b0000_1111;
            let third_nibble  = mem_ctrl.mem.ram[(mem_ctrl.mem.pc + 1) as usize] >> 4;
            let fourth_nibble = mem_ctrl.mem.ram[(mem_ctrl.mem.pc + 1) as usize] & 0b0000_1111;

            let first_byte = mem_ctrl.mem.ram[mem_ctrl.mem.pc as usize];
            let second_byte = mem_ctrl.mem.ram[(mem_ctrl.mem.pc + 1) as usize];
            let complete_instr = InstructionsController::make_16_bit_instr_from_bytes(first_byte, second_byte);

            // 00E0 - CLS
            if complete_instr == 0x00e0 {
                win_ctrl.clear_screen();
            // 1nnn - JP addr
            } else if first_nibble == 0x1 {
                mem_ctrl.mem.pc = InstructionsController::make_16_bit_addr_from_nibbles(second_nibble, third_nibble, fourth_nibble);
                return;
            // 3xkk - SE Vx, byte
            } else if first_nibble == 3 {
                if mem_ctrl.get_v_register_val_by_nibble_val(second_nibble) == second_byte {
                    mem_ctrl.mem.pc += 2;
                }
            // 4xkk - SNE Vx, byte
            } else if first_nibble == 4 {
                if mem_ctrl.get_v_register_val_by_nibble_val(second_nibble) != second_byte {
                    mem_ctrl.mem.pc += 2;
                }
            // 5xy0 - SE Vx, Vy
            } else if first_nibble == 5 && fourth_nibble == 0 {
                if mem_ctrl.get_v_register_val_by_nibble_val(second_nibble) ==
                    mem_ctrl.get_v_register_val_by_nibble_val(third_nibble) {
                    mem_ctrl.mem.pc += 2;
                }
            // 6xkk - LD Vx, byte
            } else if first_nibble == 6 {
                mem_ctrl.set_v_register_val_by_nibble_val(second_nibble, second_byte);
            // 7xkk - ADD Vx, byte.
            } else if first_nibble == 7 {
                let curr_vx_val = mem_ctrl.get_v_register_val_by_nibble_val(second_nibble);
                mem_ctrl.set_v_register_val_by_nibble_val(second_nibble, curr_vx_val.wrapping_add(second_byte));
            // 9xy0 - SNE Vx, Vy
            } else if first_nibble == 9 && fourth_nibble == 0 {
                if mem_ctrl.get_v_register_val_by_nibble_val(second_nibble) !=
                    mem_ctrl.get_v_register_val_by_nibble_val(third_nibble) {
                    mem_ctrl.mem.pc += 2;
                }
            // Annn - LD I, addr
            } else if first_nibble == 0xa {
                mem_ctrl.mem.i = InstructionsController::make_16_bit_addr_from_nibbles(second_nibble, third_nibble, fourth_nibble);
            // Bnnn - JP V0, addr
            } else if first_nibble == 0xb {
                mem_ctrl.mem.pc =
                    InstructionsController::make_16_bit_addr_from_nibbles(second_nibble, third_nibble, fourth_nibble) + mem_ctrl.mem.v0 as u16;
            // Dxyn - DRW Vx, Vy, nibble.
            } else if first_nibble == 0xd {  
                mem_ctrl.mem.vf = 0; 

                // fourth_nibble (Dxy(n)) specifies the size of the sprite.
                for byte_i in 0..fourth_nibble {
                    // Load is based on the address stored on the register i.
                    let sprite_byte = mem_ctrl.mem.ram[(mem_ctrl.mem.i + byte_i as u16) as usize];

                    let mut curr_bit = 0;
                    for rev_bit_i in (0..8).rev() {
                        // Only draws bits that are equal to 1, from most significant to least significant.
                        if (sprite_byte >> rev_bit_i & 1) == 1 {
                            win_ctrl.put_pixel(
                                mem_ctrl.get_v_register_val_by_nibble_val(second_nibble).wrapping_add(curr_bit), 
                                mem_ctrl.get_v_register_val_by_nibble_val(third_nibble).wrapping_add(byte_i),
                                &mut mem_ctrl.mem.vf); // If there was a collision between pixels, vf is set to 1.
                        }
                        curr_bit += 1;
                    }
                }
            }

            // The program counter is incremented by 2 because all instructions are 2 bytes
            // and the ram stores 1 byte values only.
            mem_ctrl.mem.pc += 2; 
        }

        fn make_16_bit_addr_from_nibbles(second_nibble: u8, third_nibble: u8, fourth_nibble: u8) -> u16 {
            (second_nibble as u16) << 8 | (third_nibble as u16) << 4 | (fourth_nibble as u16)
        }

        fn make_16_bit_instr_from_bytes(first_byte: u8, second_byte: u8) -> u16 {
            (first_byte as u16) << 8 | (second_byte as u16)
        }
    }
}
