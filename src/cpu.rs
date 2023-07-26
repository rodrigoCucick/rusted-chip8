// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod cpu {
    use crate::gfx::graphics::CustomWindowController;
    use crate::kbrd::keyboard::KeyboardController;
    use crate::mem::memory::MemoryController;
    use crate::util::utilities::{ Bit, Logic };

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
        inc_pc: bool,
    }

    impl CpuController {
        pub fn new(mem_ctrl: &MemoryController, cycles_per_frame: u32) -> Self {
            let lower_addr = mem_ctrl.get_pc() as usize;
            let first_byte = mem_ctrl.get_ram()[lower_addr];
            let second_byte = mem_ctrl.get_ram()[lower_addr + 1];

            Self {
                first_byte,
                second_byte,
                word:          Bit::make_16bit_instr_from_bytes(first_byte, second_byte),
                first_nibble:  mem_ctrl.get_ram()[lower_addr] >> 4,
                second_nibble: mem_ctrl.get_ram()[lower_addr] & 0b0000_1111,
                third_nibble:  mem_ctrl.get_ram()[lower_addr + 1] >> 4,
                fourth_nibble: mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111,
                cycles_per_frame,
                inc_pc: true
            }
        }

        pub fn get_cycles_per_frame(&self) -> u32 {
            self.cycles_per_frame
        }

        pub fn fetch_exec(
            &mut self,
            keyboard_ctrl: &mut KeyboardController,
            mem_ctrl: &mut MemoryController,
            win_ctrl: &mut CustomWindowController) {

            self.inc_pc = true;
            self.load_next_instr(mem_ctrl);
            
            match self.word {
                0x00e0 => self.clear_screen(win_ctrl),
                0x00ee => self.return_from_subroutine(mem_ctrl),
                _ => self.exec_instr_by_nibble(keyboard_ctrl, mem_ctrl, win_ctrl)
            }

            // The program counter is always incremented by 2 because all instructions are 2 bytes
            // and the ram stores 1 byte values only.
            if self.inc_pc {
                mem_ctrl.inc_pc_by(2);
            }
        }

        fn load_next_instr(&mut self, mem_ctrl: &MemoryController) {
            let lower_addr = mem_ctrl.get_pc() as usize;

            self.first_byte =    mem_ctrl.get_ram()[lower_addr];
            self.second_byte =   mem_ctrl.get_ram()[lower_addr + 1];
            self.word =          Bit::make_16bit_instr_from_bytes(self.first_byte, self.second_byte);
            self.first_nibble =  mem_ctrl.get_ram()[lower_addr] >> 4;
            self.second_nibble = mem_ctrl.get_ram()[lower_addr] & 0b0000_1111;
            self.third_nibble =  mem_ctrl.get_ram()[lower_addr + 1] >> 4;
            self.fourth_nibble = mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111;
        }

        fn exec_instr_by_nibble(
            &mut self,
            keyboard_ctrl: &mut KeyboardController,
            mem_ctrl: &mut MemoryController,
            win_ctrl: &mut CustomWindowController) {
                
            match self.first_nibble {
                1 => { self.jump_to_address(mem_ctrl); self.inc_pc = false; },
                2 => { self.call_address(mem_ctrl); self.inc_pc = false; },
                3 => self.skip_equal_vx_byte(mem_ctrl),
                4 => self.skip_not_equal_vx_byte(mem_ctrl),
                5 => self.skip_equal_vx_vy(mem_ctrl),
                6 => self.set_vx_byte(mem_ctrl),
                7 => self.add_vx_byte(mem_ctrl),
                8 => {
                    match self.fourth_nibble {
                        0 => self.set_vx_vy(mem_ctrl),
                        1 => self.set_vx_or_vy(mem_ctrl),
                        2 => self.set_vx_and_vy(mem_ctrl),
                        3 => self.set_vx_xor_vy(mem_ctrl),
                        4 => self.add_vx_vy(mem_ctrl),
                        5 => self.sub_vx_vy(mem_ctrl),
                        6 => self.shift_right_vx_vy(mem_ctrl),
                        7 => self.subn_vx_vy(mem_ctrl),
                        0xe => self.shift_left_vx_vy(mem_ctrl),
                        _ => return
                    }
                },
                9 => self.skip_not_equal_vx_vy(mem_ctrl),
                0xa => self.set_i_address(mem_ctrl),
                0xb => self.jump_to_address_plus_v0(mem_ctrl),
                0xc => self.set_vx_and_random_byte(mem_ctrl),
                0xd => self.draw_sprite(mem_ctrl, win_ctrl),
                0xe => {
                    match self.second_byte {
                        0x9e => self.skip_if_key_vx_is_pressed(keyboard_ctrl, mem_ctrl),
                        0xa1 => self.skip_if_key_vx_is_not_pressed(keyboard_ctrl, mem_ctrl),
                        _ => return
                    }
                },
                0xf => {
                    match self.second_byte {
                        0x07 => self.set_vx_dt(mem_ctrl),
                        0x0a => {
                            if let CpuState::Halted = self.halt_until_key_press(keyboard_ctrl, mem_ctrl) {
                                self.inc_pc = false;
                            }
                        },
                        0x15 => self.set_dt_vx(mem_ctrl),
                        0x18 => self.set_st_vx(mem_ctrl),
                        0x1e => self.add_i_vx(mem_ctrl),
                        0x29 => self.set_i_sprite_digit_vx(mem_ctrl),
                        0x33 => self.copy_bcd_vx_into_addr_i(mem_ctrl),
                        0x55 => self.copy_v0_through_vx_into_addr_i(mem_ctrl),
                        0x65 => self.read_v0_through_vx_from_addr_i(mem_ctrl),
                        _ => return
                    }
                },
                _ => return
            }
        }

        // 00E0 - CLS
        fn clear_screen(&self, win_ctrl: &mut CustomWindowController) {
            win_ctrl.clear_screen();
        }

        // 00EE - RET
        fn return_from_subroutine(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.stack_pop();
        }

        // 1nnn - JP addr
        fn jump_to_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_pc(Bit::make_16bit_addr_from_nibbles(
                self.second_nibble,
                self.third_nibble,
                self.fourth_nibble));
        }

        // 2nnn - CALL addr
        fn call_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.stack_push(Bit::make_16bit_addr_from_nibbles(
                self.second_nibble,
                self.third_nibble,
                self.fourth_nibble));
        }
        
        // 3xkk - SE Vx, byte
        fn skip_equal_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v_by_nibble(self.second_nibble) == self.second_byte {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 4xkk - SNE Vx, byte
        fn skip_not_equal_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v_by_nibble(self.second_nibble) != self.second_byte {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 5xy0 - SE Vx, Vy
        fn skip_equal_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v_by_nibble(self.second_nibble) ==
                mem_ctrl.get_v_by_nibble(self.third_nibble) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 6xkk - LD Vx, byte
        fn set_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v_by_nibble(self.second_nibble, self.second_byte);
        }

        // 7xkk - ADD Vx, byte.
        fn add_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
            mem_ctrl.set_v_by_nibble(self.second_nibble, vx.wrapping_add(self.second_byte));
        }

        // 8xy0 - LD Vx, Vy
        fn set_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vy = mem_ctrl.get_v_by_nibble(self.third_nibble);
            mem_ctrl.set_v_by_nibble(self.second_nibble, vy);
        }

        // 8xy1 - OR Vx, Vy
        fn set_vx_or_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_or_vy =
                mem_ctrl.get_v_by_nibble(self.second_nibble) |
                mem_ctrl.get_v_by_nibble(self.third_nibble);
            mem_ctrl.set_v_by_nibble(self.second_nibble, vx_or_vy);
        }

        // 8xy2 - AND Vx, Vy
        fn set_vx_and_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_and_vy =
                mem_ctrl.get_v_by_nibble(self.second_nibble) &
                mem_ctrl.get_v_by_nibble(self.third_nibble);
            mem_ctrl.set_v_by_nibble(self.second_nibble, vx_and_vy);
        }

        // 8xy3 - XOR Vx, Vy
        fn set_vx_xor_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_xor_vy =
                mem_ctrl.get_v_by_nibble(self.second_nibble) ^
                mem_ctrl.get_v_by_nibble(self.third_nibble);
            mem_ctrl.set_v_by_nibble(self.second_nibble, vx_xor_vy);
        }

        // 8xy4 - ADD Vx, Vy
        fn add_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_plus_vy =
                mem_ctrl.get_v_by_nibble(self.second_nibble) as u16 +
                mem_ctrl.get_v_by_nibble(self.third_nibble) as u16;
            // vf = carry flag
            mem_ctrl.set_v_by_nibble(0xf, Logic::bool_to_u8(vx_plus_vy > 255));
            mem_ctrl.set_v_by_nibble(self.second_nibble, (vx_plus_vy & 0b1111_1111) as u8);
        }

        // 8xy5 - SUB Vx, Vy
        fn sub_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
            let vy = mem_ctrl.get_v_by_nibble(self.third_nibble);
            // vf = NOT borrow/underflow flag
            mem_ctrl.set_v_by_nibble(0xf, Logic::bool_to_u8(vx > vy));
            mem_ctrl.set_v_by_nibble(self.second_nibble, vx.wrapping_sub(vy));
        }

        // 8xy6 - SHR Vx {, Vy}
        fn shift_right_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let v;
            if self.third_nibble != 0 {
                v = mem_ctrl.get_v_by_nibble(self.third_nibble);
            } else {
                v = mem_ctrl.get_v_by_nibble(self.second_nibble);
                // vf = least significant bit of vx
                mem_ctrl.set_v_by_nibble(0xf, v & 1);
            }
            mem_ctrl.set_v_by_nibble(self.second_nibble, v >> 1);
        }

        // 8xy7 - SUBN Vx, Vy
        fn subn_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
            let vy = mem_ctrl.get_v_by_nibble(self.third_nibble);
            // vf = NOT borrow/underflow flag
            mem_ctrl.set_v_by_nibble(0xf, Logic::bool_to_u8(vy > vx));
            mem_ctrl.set_v_by_nibble(self.second_nibble, vy.wrapping_sub(vx));
        }

        // 8xyE - SHL Vx {, Vy}
        fn shift_left_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let v;
            if self.third_nibble != 0 {
                v = mem_ctrl.get_v_by_nibble(self.third_nibble);
            } else {
                v = mem_ctrl.get_v_by_nibble(self.second_nibble);
                // vf = most significant bit of vx
                mem_ctrl.set_v_by_nibble(0xf, v & 0b1000);
            }
            mem_ctrl.set_v_by_nibble(self.second_nibble, v << 1);
        }

        // 9xy0 - SNE Vx, Vy
        fn skip_not_equal_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v_by_nibble(self.second_nibble) !=
                mem_ctrl.get_v_by_nibble(self.third_nibble) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // Annn - LD I, addr
        fn set_i_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_i(Bit::make_16bit_addr_from_nibbles(
                self.second_nibble,
                self.third_nibble,
                self.fourth_nibble));
        }

        // Bnnn - JP V0, addr
        fn jump_to_address_plus_v0(&self, mem_ctrl: &mut MemoryController) {
            let v0 = mem_ctrl.get_v_by_nibble(0) as u16;
            mem_ctrl.set_pc(Bit::make_16bit_addr_from_nibbles(
                    self.second_nibble,
                    self.third_nibble,
                    self.fourth_nibble) + v0);
        }

        // Cxkk - RND Vx, byte
        fn set_vx_and_random_byte(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v_by_nibble(self.second_nibble,
                rand::thread_rng().gen_range(0..=255) & self.second_byte);
        }

        // Dxyn - DRW Vx, Vy, nibble.
        fn draw_sprite(&self, mem_ctrl: &mut MemoryController, win_ctrl: &mut CustomWindowController) {
            mem_ctrl.set_v_by_nibble(0xf, 0);

            // fourth_nibble specifies the size of the sprite.
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
                            &mut mem_ctrl.get_v_by_nibble(0xf)); // If a collision happened, vf is set to 1.
                    }
                    curr_bit += 1;
                }
            }
        }

        // Ex9E - SKP Vx
        fn skip_if_key_vx_is_pressed(&self, keyboard_ctrl: &mut KeyboardController, mem_ctrl: &mut MemoryController) {
            if keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v_by_nibble(self.second_nibble)) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // ExA1 - SKNP Vx
        fn skip_if_key_vx_is_not_pressed(&self, keyboard_ctrl: &mut KeyboardController, mem_ctrl: &mut MemoryController) {
            if !keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v_by_nibble(self.second_nibble)) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // Fx07 - LD Vx, DT
        fn set_vx_dt(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v_by_nibble(self.second_nibble, mem_ctrl.get_dt());
        }

        // Fx0A - LD Vx, K
        fn halt_until_key_press(&self, keyboard_ctrl: &mut KeyboardController, mem_ctrl: &mut MemoryController) -> CpuState {
            match keyboard_ctrl.get_any_key_down() {
                Some(key_down) => {
                    mem_ctrl.set_v_by_nibble(self.second_nibble, key_down);
                    CpuState::NotHalted
                },
                None => CpuState::Halted
            }
        }

        // Fx15 - LD DT, Vx
        fn set_dt_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_nibble);
            mem_ctrl.set_dt(vx);
        }

        // Fx18 - LD ST, Vx
        fn set_st_vx(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_st(self.second_nibble);
        }

        // Fx1E - ADD I, Vx
        fn add_i_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_nibble) as u16;
            mem_ctrl.set_i(mem_ctrl.get_i().wrapping_add(vx));
        }

        // Fx29 - LD F, Vx
        fn set_i_sprite_digit_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v_by_nibble(self.second_byte) as usize;
            // See load_hex_digits_sprites() (MemoryController).
            mem_ctrl.set_i(mem_ctrl.get_ram()[vx * 5] as u16);
        }

        // Fx33 - LD B, Vx
        fn copy_bcd_vx_into_addr_i(&self, mem_ctrl: &mut MemoryController) {
            let bcd_tuple = Bit::decimal_to_8bit_bcd_tuple(
                mem_ctrl.get_v_by_nibble(self.second_nibble));
            let i = mem_ctrl.get_i() as usize;
            mem_ctrl.get_ram()[i] = bcd_tuple.0;
            mem_ctrl.get_ram()[i + 1] = bcd_tuple.1;
            mem_ctrl.get_ram()[i + 2] = bcd_tuple.2;
        }

        // Fx55 - LD [I], Vx
        fn copy_v0_through_vx_into_addr_i(&self, mem_ctrl: &mut MemoryController) {
            for i in 0..=self.second_nibble {
                mem_ctrl.get_ram()[(mem_ctrl.get_i() + i as u16) as usize] =
                    mem_ctrl.get_v_by_nibble(i);
            }
        }

        // Fx65 - LD Vx, [I]
        fn read_v0_through_vx_from_addr_i(&self, mem_ctrl: &mut MemoryController) {
            for i in 0..=self.second_nibble {
                mem_ctrl.set_v_by_nibble(i,
                    mem_ctrl.get_ram()[(mem_ctrl.get_i() + i as u16) as usize]);
            }
        }
    }

    enum CpuState {
        Halted,
        NotHalted,
    }
}
