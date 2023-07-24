// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod cpu {
    use crate::gfx::graphics::CustomWindowController;
    use crate::kbrd::keyboard::KeyboardController;
    use crate::mem::memory::MemoryController;
    use crate::util::utilities::BitManipulator;

    use rand::Rng;

    pub struct CpuController {
        // Big-endian.
        word: u16,         // [0000111100001111]
        first_byte: u8,    // [00001111]00001111
        second_byte: u8,   // 00001111[00001111]
        first_nibble: u8,  // [0000]111100001111
        second_nibble: u8, // 0000[1111]00001111
        third_nibble: u8,  // 00001111[0000]1111
        fourth_nibble: u8, // 000011110000[1111]
        cycles_per_frame: u32,
    }

    impl CpuController {
        pub fn new(mem_ctrl: &MemoryController, cycles_per_frame: u32) -> Self {
            let lower_addr = mem_ctrl.get_pc() as usize;
            let first_byte = mem_ctrl.get_ram()[lower_addr];
            let second_byte = mem_ctrl.get_ram()[lower_addr + 1];

            Self {
                first_byte,
                second_byte,
                word:          BitManipulator::make_16bit_instr_from_bytes(first_byte, second_byte),
                first_nibble:  mem_ctrl.get_ram()[lower_addr] >> 4,
                second_nibble: mem_ctrl.get_ram()[lower_addr] & 0b0000_1111,
                third_nibble:  mem_ctrl.get_ram()[lower_addr + 1] >> 4,
                fourth_nibble: mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111,
                cycles_per_frame,
            }
        }

        pub fn get_cycles_per_frame(&self) -> u32 {
            self.cycles_per_frame
        }

        pub fn exec_next_instr(
            &mut self, mem_ctrl: &mut MemoryController,
            win_ctrl: &mut CustomWindowController,
            keyboard_ctrl: &mut KeyboardController) {
            self.load_next_instr(mem_ctrl);
            
            // 00E0 - CLS
            if self.word == 0x00e0 {
                win_ctrl.clear_screen();

            // 00EE - RET
            } else if self.word == 0x00ee {
                mem_ctrl.stack_pop();
                //return;

            // 1nnn - JP addr
            } else if self.first_nibble == 1 {
                mem_ctrl.set_pc(BitManipulator::make_16bit_addr_from_nibbles(
                    self.second_nibble,
                    self.third_nibble,
                    self.fourth_nibble));
                return;

            // 2nnn - CALL addr
            } else if self.first_nibble == 2 {
                mem_ctrl.stack_push(BitManipulator::make_16bit_addr_from_nibbles(
                    self.second_nibble,
                    self.third_nibble,
                    self.fourth_nibble));
                return;

            // 3xkk - SE Vx, byte
            } else if self.first_nibble == 3 {
                if mem_ctrl.get_v_by_nibble(self.second_nibble) == self.second_byte {
                    mem_ctrl.inc_pc_by(2);
                }

            // 4xkk - SNE Vx, byte
            } else if self.first_nibble == 4 {
                if mem_ctrl.get_v_by_nibble(self.second_nibble) != self.second_byte {
                    mem_ctrl.inc_pc_by(2);
                }

            // 5xy0 - SE Vx, Vy
            } else if self.first_nibble == 5 && self.fourth_nibble == 0 {
                if mem_ctrl.get_v_by_nibble(self.second_nibble) == mem_ctrl.get_v_by_nibble(self.third_nibble) {
                    mem_ctrl.inc_pc_by(2);
                }

            // 6xkk - LD Vx, byte
            } else if self.first_nibble == 6 {
                mem_ctrl.set_v_by_nibble(self.second_nibble, self.second_byte);

            // 7xkk - ADD Vx, byte.
            } else if self.first_nibble == 7 {
                let curr_vx_val = mem_ctrl.get_v_by_nibble(self.second_nibble);
                mem_ctrl.set_v_by_nibble(self.second_nibble, curr_vx_val.wrapping_add(self.second_byte));

            // 8xy0 - LD Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 0 {
                let temp_vy_val = mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vy_val);

            // 8xy1 - OR Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 1 {
                let temp_vx_or_vy =
                    mem_ctrl.get_v_by_nibble(self.second_nibble) | mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx_or_vy);

            // 8xy2 - AND Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 2 {
                let temp_vx_and_vy =
                    mem_ctrl.get_v_by_nibble(self.second_nibble) & mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx_and_vy);

            // 8xy3 - XOR Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 3 {
                let temp_vx_xor_vy =
                    mem_ctrl.get_v_by_nibble(self.second_nibble) ^ mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx_xor_vy);

            // 8xy4 - ADD Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 4 {
                let temp_vx_plus_vy =
                    mem_ctrl.get_v_by_nibble(self.second_nibble) as u16 + mem_ctrl.get_v_by_nibble(self.third_nibble) as u16;
                mem_ctrl.set_v_by_nibble(0xf, if temp_vx_plus_vy > 255 { 1 } else { 0 }); // Carry.
                mem_ctrl.set_v_by_nibble(self.second_nibble, (temp_vx_plus_vy & 0b0000_0000_1111_1111) as u8);

            // 8xy5 - SUB Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 5 {
                let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
                let temp_vy = mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(0xf, if temp_vx > temp_vy { 1 } else { 0 }); // NOT borrow.
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx.wrapping_sub(temp_vy));

            // 8xy6 - SHR Vx {, Vy}
            } else if self.first_nibble == 8 && self.fourth_nibble == 6 {
                let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
                mem_ctrl.set_v_by_nibble(0xf, if temp_vx & 0b0001 == 1 { 1 } else { 0 });
                let temp_vx_shr_one = temp_vx >> 1;
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx_shr_one);

            // 8xy7 - SUBN Vx, Vy
            } else if self.first_nibble == 8 && self.fourth_nibble == 7 {
                let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
                let temp_vy = mem_ctrl.get_v_by_nibble(self.third_nibble);
                mem_ctrl.set_v_by_nibble(0xf, if temp_vy > temp_vx { 1 } else { 0 }); // NOT borrow.
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vy.wrapping_sub(temp_vx));

            // 8xyE - SHL Vx {, Vy}
            } else if self.first_nibble == 8 && self.fourth_nibble == 0xe {
                let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
                mem_ctrl.set_v_by_nibble(0xf, if temp_vx & 0b1000 == 1 { 1 } else { 0 });
                let temp_vx_shl_one = temp_vx << 1;
                mem_ctrl.set_v_by_nibble(self.second_nibble, temp_vx_shl_one);

            // 9xy0 - SNE Vx, Vy
            } else if self.first_nibble == 9 && self.fourth_nibble == 0 {
                if mem_ctrl.get_v_by_nibble(self.second_nibble) != mem_ctrl.get_v_by_nibble(self.third_nibble) {
                    mem_ctrl.inc_pc_by(2);
                }

            // Annn - LD I, addr
            } else if self.first_nibble == 0xa {
                mem_ctrl.set_i(BitManipulator::make_16bit_addr_from_nibbles(
                    self.second_nibble,
                    self.third_nibble,
                    self.fourth_nibble));

            // Bnnn - JP V0, addr
            } else if self.first_nibble == 0xb {
                let temp_v0 = mem_ctrl.get_v_by_nibble(0) as u16;
                mem_ctrl.set_pc(BitManipulator::make_16bit_addr_from_nibbles(
                        self.second_nibble,
                        self.third_nibble,
                        self.fourth_nibble) + temp_v0);

            // Cxkk - RND Vx, byte
            } else if self.first_nibble == 0xc {
                mem_ctrl.set_v_by_nibble(self.second_nibble, rand::thread_rng().gen_range(0..=255) & self.second_byte);

            // Dxyn - DRW Vx, Vy, nibble.
            } else if self.first_nibble == 0xd {  
                mem_ctrl.set_v_by_nibble(0xf, 0);

                // fourth_nibble (Dxy(n)) specifies the size of the sprite.
                for byte_i in 0..self.fourth_nibble {
                    // Load is based on the address stored on the register i.
                    let sprite_byte = mem_ctrl.get_ram()[(mem_ctrl.get_i() + byte_i as u16) as usize];

                    let mut curr_bit = 0;
                    for rev_bit_i in (0..8).rev() {
                        // Only draws bits that are equal to 1, from most significant to least significant.
                        if (sprite_byte >> rev_bit_i & 1) == 1 {
                            win_ctrl.put_pixel(
                                mem_ctrl.get_v_by_nibble(self.second_nibble).wrapping_add(curr_bit), 
                                mem_ctrl.get_v_by_nibble(self.third_nibble).wrapping_add(byte_i),
                                &mut mem_ctrl.get_v_by_nibble(0xf)); // If there was a collision between pixels, vf is set to 1.
                        }
                        curr_bit += 1;
                    }
                }

            // Ex
            } else if self.first_nibble == 0xe {
                // 9E - SKP Vx
                if self.second_byte == 0x9e &&
                    keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v_by_nibble(self.second_nibble)) {
                    mem_ctrl.inc_pc_by(2);

                // A1 - SKNP Vx
                } else if self.second_byte == 0xa1 &&
                    !keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v_by_nibble(self.second_nibble)) {
                    mem_ctrl.inc_pc_by(2);
                }
            
            // Fx
            } else if self.first_nibble == 0xf {
                // 07 - LD Vx, DT
                if self.second_byte == 0x07 {
                    mem_ctrl.set_v_by_nibble(self.second_nibble, mem_ctrl.get_dt());
                }

                // 0A - LD Vx, K
                else if self.second_byte == 0x0a {
                    match keyboard_ctrl.get_any_key_down() {
                        Some(key_down) => mem_ctrl.set_v_by_nibble(self.second_nibble, key_down),
                        None => return
                    }
                }

                // 15 - LD DT, Vx
                else if self.second_byte == 0x15 {
                    let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
                    mem_ctrl.set_dt(temp_vx);
                }

                // 18 - LD ST, Vx
                else if self.second_byte == 0x18 {
                    mem_ctrl.set_st(self.second_nibble);
                }

                // 1E - ADD I, Vx
                else if self.second_byte == 0x1e {
                    let temp_vx = mem_ctrl.get_v_by_nibble(self.second_nibble) as u16;
                    mem_ctrl.set_i(mem_ctrl.get_i().wrapping_add(temp_vx));
                }

                // 29 - LD F, Vx
                else if self.second_byte == 0x29 {
                    let temp_vx = mem_ctrl.get_v_by_nibble(self.second_byte) as usize;
                    mem_ctrl.set_i(mem_ctrl.get_ram()[temp_vx * 5] as u16);
                }

                // 33 - LD B, Vx
                else if self.second_byte == 0x33 {
                    let bcd_tuple = BitManipulator::decimal_to_8bit_bcd_tuple(
                        mem_ctrl.get_v_by_nibble(self.second_nibble));
                    let temp_i = mem_ctrl.get_i();
                    mem_ctrl.get_ram()[temp_i as usize] = bcd_tuple.0;
                    mem_ctrl.get_ram()[temp_i as usize + 1] = bcd_tuple.1;
                    mem_ctrl.get_ram()[temp_i as usize + 2] = bcd_tuple.2;
                }

                // 55 - LD [I], Vx
                else if self.second_byte == 0x55 {
                    for i in 0..=self.second_nibble {
                        mem_ctrl.get_ram()[(mem_ctrl.get_i() + i as u16) as usize] = mem_ctrl.get_v_by_nibble(i);
                    }
                }

                // 65 - LD Vx, [I]
                else if self.second_byte == 0x65 {
                    for i in 0..=self.second_nibble {
                        mem_ctrl.set_v_by_nibble(i, mem_ctrl.get_ram()[(mem_ctrl.get_i() + i as u16) as usize]);
                    }
                }
            }

            // The program counter is always incremented by 2 because all instructions are 2 bytes
            // and the ram stores 1 byte values only.
            mem_ctrl.inc_pc_by(2); 
        }

        fn load_next_instr(&mut self, mem_ctrl: &MemoryController) {
            let lower_addr = mem_ctrl.get_pc() as usize;

            self.first_byte =    mem_ctrl.get_ram()[lower_addr];
            self.second_byte =   mem_ctrl.get_ram()[lower_addr + 1];
            self.word =          BitManipulator::make_16bit_instr_from_bytes(self.first_byte, self.second_byte);
            self.first_nibble =  mem_ctrl.get_ram()[lower_addr] >> 4;
            self.second_nibble = mem_ctrl.get_ram()[lower_addr] & 0b0000_1111;
            self.third_nibble =  mem_ctrl.get_ram()[lower_addr + 1] >> 4;
            self.fourth_nibble = mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111;
        }
    }
}
