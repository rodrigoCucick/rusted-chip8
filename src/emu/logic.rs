// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod cpu {
    use crate::emu::input::keyboard::KeyboardController;
    use crate::emu::memory::memory::MemoryController;
    use crate::sdl::wrapper::SDLController;
    use crate::util::utilities::{ BitUtil, LogicUtil };

    use rand::Rng;

    pub struct CpuController {
        // Big-endian.
        word: u16,         // [0000111100001111]
        first_byte: u8,    // [00001111]00001111
        second_byte: u8,   // 00001111[00001111]
        first_nibble: u8,  // [0000]111100001111
        x: u8,             // 0000[1111]00001111
        y: u8,             // 00001111[0000]1111
        fourth_nibble: u8, // 000011110000[1111]
        inc_pc: bool,
        cycles_per_frame: u32,
        bit_shift_instructions_use_vy: bool,
        store_read_instructions_change_i: bool,
    }

    impl CpuController {
        pub fn new(
            mem_ctrl: &MemoryController,
            cycles_per_frame: u32,
            bit_shift_instructions_use_vy: bool,
            store_read_instructions_change_i: bool) -> Self {

            let lower_addr = mem_ctrl.get_pc() as usize;
            let first_byte = mem_ctrl.get_ram()[lower_addr];
            let second_byte = mem_ctrl.get_ram()[lower_addr + 1];

            Self {
                first_byte,
                second_byte,
                word:          BitUtil::make_16bit_instr_from_bytes(first_byte, second_byte),
                first_nibble:  mem_ctrl.get_ram()[lower_addr] >> 4,
                x:             mem_ctrl.get_ram()[lower_addr] & 0b0000_1111,
                y:             mem_ctrl.get_ram()[lower_addr + 1] >> 4,
                fourth_nibble: mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111,
                inc_pc: true,
                cycles_per_frame,
                bit_shift_instructions_use_vy,
                store_read_instructions_change_i,
            }
        }

        pub fn fetch_exec(
            &mut self,
            sdl_ctrl: &mut SDLController,
            mem_ctrl: &mut MemoryController,
            keyboard_ctrl: &mut KeyboardController) {

            self.inc_pc = true;
            self.load_next_instr(mem_ctrl);
            
            match self.word {
                0x00e0 => self.clear_screen(sdl_ctrl),
                0x00ee => self.return_from_subroutine(mem_ctrl),
                _ => self.exec_instr_by_nibble(sdl_ctrl, mem_ctrl, keyboard_ctrl)
            }

            if self.inc_pc {
                // The program counter is always incremented by 2 because all instructions are 2 bytes
                // and the ram stores 1 byte values only.
                mem_ctrl.inc_pc_by(2);
            }
        }

        fn load_next_instr(&mut self, mem_ctrl: &MemoryController) {
            let lower_addr = mem_ctrl.get_pc() as usize;

            self.first_byte =    mem_ctrl.get_ram()[lower_addr];
            self.second_byte =   mem_ctrl.get_ram()[lower_addr + 1];
            self.word =          BitUtil::make_16bit_instr_from_bytes(self.first_byte, self.second_byte);
            self.first_nibble =  mem_ctrl.get_ram()[lower_addr] >> 4;
            self.x =             mem_ctrl.get_ram()[lower_addr] & 0b0000_1111;
            self.y =             mem_ctrl.get_ram()[lower_addr + 1] >> 4;
            self.fourth_nibble = mem_ctrl.get_ram()[lower_addr + 1] & 0b0000_1111;
        }

        fn exec_instr_by_nibble(
            &mut self,
            sdl_ctrl: &mut SDLController,
            mem_ctrl: &mut MemoryController,
            keyboard_ctrl: &mut KeyboardController) {
                
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
                        _ => CpuController::log_not_implemented(self.word)
                    }
                },
                9 => self.skip_not_equal_vx_vy(mem_ctrl),
                0xa => self.set_i_address(mem_ctrl),
                0xb => { self.jump_to_address_plus_v0(mem_ctrl); self.inc_pc = false; },
                0xc => self.set_vx_and_random_byte(mem_ctrl),
                0xd => self.draw_sprite(mem_ctrl, sdl_ctrl),
                0xe => {
                    match self.second_byte {
                        0x9e => self.skip_if_key_vx_is_pressed(keyboard_ctrl, mem_ctrl),
                        0xa1 => self.skip_if_key_vx_is_not_pressed(keyboard_ctrl, mem_ctrl),
                        _ => CpuController::log_not_implemented(self.word)
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
                        _ => CpuController::log_not_implemented(self.word)
                    }
                },
                _ => CpuController::log_not_implemented(self.word)
            }
        }

        pub fn get_cycles_per_frame(&self) -> u32 {
            self.cycles_per_frame
        }

        // 00E0 - CLS
        fn clear_screen(&self, sdl_ctrl: &mut SDLController) {
            sdl_ctrl.clear_screen();
        }

        // 00EE - RET
        fn return_from_subroutine(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.stack_pop();
        }

        // 1nnn - JP addr
        fn jump_to_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_pc(BitUtil::make_16bit_addr_from_nibbles(
                self.x,
                self.y,
                self.fourth_nibble));
        }

        // 2nnn - CALL addr
        fn call_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.stack_push(BitUtil::make_16bit_addr_from_nibbles(
                self.x,
                self.y,
                self.fourth_nibble));
        }
        
        // 3xkk - SE Vx, byte
        fn skip_equal_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v(self.x) == self.second_byte {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 4xkk - SNE Vx, byte
        fn skip_not_equal_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v(self.x) != self.second_byte {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 5xy0 - SE Vx, Vy
        fn skip_equal_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v(self.x) == mem_ctrl.get_v(self.y) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // 6xkk - LD Vx, byte
        fn set_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v(self.x, self.second_byte);
        }

        // 7xkk - ADD Vx, byte.
        fn add_vx_byte(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x);
            mem_ctrl.set_v(self.x, vx.overflowing_add(self.second_byte).0);
        }

        // 8xy0 - LD Vx, Vy
        fn set_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vy = mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(self.x, vy);
        }

        // 8xy1 - OR Vx, Vy
        fn set_vx_or_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_or_vy = mem_ctrl.get_v(self.x) | mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(self.x, vx_or_vy);
        }

        // 8xy2 - AND Vx, Vy
        fn set_vx_and_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_and_vy = mem_ctrl.get_v(self.x) & mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(self.x, vx_and_vy);
        }

        // 8xy3 - XOR Vx, Vy
        fn set_vx_xor_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_xor_vy = mem_ctrl.get_v(self.x) ^ mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(self.x, vx_xor_vy);
        }

        // 8xy4 - ADD Vx, Vy
        fn add_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx_plus_vy = mem_ctrl.get_v(self.x) as u16 + mem_ctrl.get_v(self.y) as u16;
            mem_ctrl.set_v(0xf, LogicUtil::bool_to_u8(vx_plus_vy > 255)); // vf = carry flag
            mem_ctrl.set_v(self.x, (vx_plus_vy & 0b1111_1111) as u8);
        }

        // 8xy5 - SUB Vx, Vy
        fn sub_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x);
            let vy = mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(0xf, LogicUtil::bool_to_u8(vx > vy)); // vf = NOT borrow flag
            mem_ctrl.set_v(self.x, vx.wrapping_sub(vy));
        }

        // 8xy6 - SHR Vx {, Vy}
        fn shift_right_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let v =  mem_ctrl.get_v(if self.bit_shift_instructions_use_vy { self.y } else { self.x } );
            mem_ctrl.set_v(0xf, v & 1); // vf = least significant bit of v
            mem_ctrl.set_v(self.x, v >> 1);            
        }

        // 8xy7 - SUBN Vx, Vy
        fn subn_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x);
            let vy = mem_ctrl.get_v(self.y);
            mem_ctrl.set_v(0xf, LogicUtil::bool_to_u8(vy > vx)); // vf = NOT borrow flag
            mem_ctrl.set_v(self.x, vy.wrapping_sub(vx));
        }

        // 8xyE - SHL Vx {, Vy}
        fn shift_left_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            let v = mem_ctrl.get_v(if self.bit_shift_instructions_use_vy { self.y } else { self.x } );
            mem_ctrl.set_v(0xf, v >> 7); // vf = most significant bit of v
            mem_ctrl.set_v(self.x, v << 1);
        }

        // 9xy0 - SNE Vx, Vy
        fn skip_not_equal_vx_vy(&self, mem_ctrl: &mut MemoryController) {
            if mem_ctrl.get_v(self.x) != mem_ctrl.get_v(self.y) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // Annn - LD I, addr
        fn set_i_address(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_i(BitUtil::make_16bit_addr_from_nibbles(
                self.x,
                self.y,
                self.fourth_nibble));
        }

        // Bnnn - JP V0, addr
        fn jump_to_address_plus_v0(&self, mem_ctrl: &mut MemoryController) {
            let v0 = mem_ctrl.get_v(0) as u16;
            mem_ctrl.set_pc(BitUtil::make_16bit_addr_from_nibbles(
                    self.x,
                    self.y,
                    self.fourth_nibble) + v0);
        }

        // Cxkk - RND Vx, byte
        fn set_vx_and_random_byte(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v(
                self.x,
                rand::thread_rng().gen_range(0..=255) & self.second_byte);
        }

        // Dxyn - DRW Vx, Vy, nibble.
        fn draw_sprite(&self, mem_ctrl: &mut MemoryController, sdl_ctrl: &mut SDLController) {
            mem_ctrl.set_v(0xf, 0);

            // fourth_nibble specifies the size of the sprite.
            for byte_i in 0..self.fourth_nibble {
                // Load is based on the address stored on the register i.
                let sprite_byte = mem_ctrl.get_ram()[(mem_ctrl.get_i() + byte_i as u16) as usize];

                let mut curr_bit = 0;
                for rev_bit_i in (0..8).rev() {
                    // Only draws bits that are equal to 1, from most significant to least significant.
                    if (sprite_byte >> rev_bit_i & 1) == 1 {
                        // If a collision happened, vf is set to 1 for the entire current drawing routine.
                        sdl_ctrl.put_pixel(
                            mem_ctrl.get_v(self.x).wrapping_add(curr_bit), 
                            mem_ctrl.get_v(self.y).wrapping_add(byte_i),
                            mem_ctrl);
                    }
                    curr_bit += 1;
                }
            }
        }

        // Ex9E - SKP Vx
        fn skip_if_key_vx_is_pressed(&self, keyboard_ctrl: &mut KeyboardController, mem_ctrl: &mut MemoryController) {
            if keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v(self.x)) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // ExA1 - SKNP Vx
        fn skip_if_key_vx_is_not_pressed(&self, keyboard_ctrl: &mut KeyboardController, mem_ctrl: &mut MemoryController) {
            if !keyboard_ctrl.is_key_x_pressed(mem_ctrl.get_v(self.x)) {
                mem_ctrl.inc_pc_by(2);
            }
        }

        // Fx07 - LD Vx, DT
        fn set_vx_dt(&self, mem_ctrl: &mut MemoryController) {
            mem_ctrl.set_v(self.x, mem_ctrl.get_dt());
        }

        // Fx0A - LD Vx, K
        fn halt_until_key_press(
            &self, keyboard_ctrl: &mut KeyboardController,
            mem_ctrl: &mut MemoryController) -> CpuState {

            match keyboard_ctrl.get_any_key_down() {
                Some(key_down) => {
                    mem_ctrl.set_v(self.x, key_down);
                    CpuState::NotHalted
                },
                None => CpuState::Halted
            }
        }

        // Fx15 - LD DT, Vx
        fn set_dt_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x);
            mem_ctrl.set_dt(vx);
        }

        // Fx18 - LD ST, Vx
        fn set_st_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x);
            mem_ctrl.set_st(vx);
        }

        // Fx1E - ADD I, Vx
        fn add_i_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x) as u16;
            mem_ctrl.inc_i_by(vx);
        }

        // Fx29 - LD F, Vx
        fn set_i_sprite_digit_vx(&self, mem_ctrl: &mut MemoryController) {
            let vx = mem_ctrl.get_v(self.x) as u16;
            // See load_hex_digits_sprites() (MemoryController).
            mem_ctrl.set_i(vx * 5);
        }

        // Fx33 - LD B, Vx
        fn copy_bcd_vx_into_addr_i(&self, mem_ctrl: &mut MemoryController) {
            let bcd_tuple = BitUtil::decimal_to_8bit_bcd_tuple(mem_ctrl.get_v(self.x));
            let i = mem_ctrl.get_i() as usize;
            mem_ctrl.set_ram(i, bcd_tuple.0);
            mem_ctrl.set_ram(i + 1, bcd_tuple.1);
            mem_ctrl.set_ram(i + 2, bcd_tuple.2);
        }

        // Fx55 - LD [I], Vx
        fn copy_v0_through_vx_into_addr_i(&self, mem_ctrl: &mut MemoryController) {
            let mut vi: u8;
            for i in 0..=self.x {
                let index = (mem_ctrl.get_i() + i as u16) as usize;
                vi = mem_ctrl.get_v(i);
                mem_ctrl.set_ram(index, vi);
            }
            self.check_inc_i(mem_ctrl);
        }

        // Fx65 - LD Vx, [I]
        fn read_v0_through_vx_from_addr_i(&self, mem_ctrl: &mut MemoryController) {
            for i in 0..=self.x {
                mem_ctrl.set_v(
                    i,
                    mem_ctrl.get_ram()[(mem_ctrl.get_i() + i as u16) as usize]);
            }
            self.check_inc_i(mem_ctrl);
        }

        fn check_inc_i(&self, mem_ctrl: &mut MemoryController) {
            if self.store_read_instructions_change_i {
                mem_ctrl.inc_i_by(self.x as u16 + 1);
            }
        }

        fn log_not_implemented(instr: u16) {
            println!("Tried to execute non-implemented instruction: {:#06X}", instr);
        }
    }

    enum CpuState {
        Halted,
        NotHalted,
    }
}
