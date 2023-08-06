// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod memory {
    use std::fs::File;
    use std::io::Read;
    
    pub struct Memory {
        // Addresses from 0x000 (0) to 0x1ff (511) were originally occupied by the interpreter,
        // which means most common Chip-8 programs start at address 0x200 (512).
        ram: [u8;4096],

        // The stack allows for up to 16 nested subroutines.
        stack: [u16;16],

        // General purpose registers.
        v0: u8, v1: u8, v2: u8, v3: u8,
        v4: u8, v5: u8, v6: u8, v7: u8,
        v8: u8, v9: u8, va: u8, vb: u8,
        vc: u8, vd: u8, ve: u8, vf: u8,

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
                v0: 0, v1: 0, v2: 0, v3: 0,
                v4: 0, v5: 0, v6: 0, v7: 0,
                v8: 0, v9: 0, va: 0, vb: 0,
                vc: 0, vd: 0, ve: 0, vf: 0,
                dt: 0,
                st: 0,
                pc: 0x200, // Default initial address for the program counter.
                sp: 0,
                i:  0,
            }
        }
    }

    pub struct MemoryController {
        mem: Memory,
    }

    impl MemoryController {
        pub fn new(mem: Memory) -> Self {
            Self { mem, }
        }

        pub fn init_ram(&mut self, rom_path: &str) {
            self.load_rom(rom_path);
            self.load_hex_digits();
        }

        pub fn get_ram(&self) -> [u8;4096] {
            self.mem.ram
        }

        pub fn set_ram(&mut self, index: usize, val: u8) {
            self.mem.ram[index] = val;
        }

        pub fn get_v(&mut self, nibble: u8) -> u8 {
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
                0xf => self.mem.vf,
                _ => 0
            }
        }

        pub fn set_v(&mut self, nibble: u8, val: u8) {
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
                0xf => self.mem.vf = val,
                _ => ()
            }
        }

        pub fn get_dt(&self) -> u8 {
            self.mem.dt
        }

        pub fn set_dt(&mut self, val: u8) {
            self.mem.dt = val;
        }

        pub fn dec_dt(&mut self) {
            if self.mem.dt > 0 {
                self.mem.dt -= 1;
            }
        }

        pub fn get_st(&self) -> u8 {
            self.mem.st
        }

        pub fn set_st(&mut self, val: u8) {
            self.mem.st = val;
        }

        pub fn dec_st(&mut self) {
            if self.mem.st > 0 {
                self.mem.st -= 1;
            }
        }

        pub fn dec_all_timers(&mut self) {
            self.dec_dt();
            self.dec_st();
        }

        pub fn get_pc(&self) -> u16 {
            self.mem.pc
        }

        pub fn set_pc(&mut self, val: u16) {
            self.mem.pc = val;
        }

        pub fn inc_pc_by(&mut self, val: u16) {
            self.mem.pc += val;
        }

        pub fn get_i(&self) -> u16 {
            self.mem.i
        }

        pub fn set_i(&mut self, val:u16) {
            self.mem.i = val;
        }

        pub fn inc_i_by(&mut self, val:u16) {
            self.mem.i += val;
        }

        pub fn stack_push(&mut self, new_pc_addr: u16) {
            self.mem.sp += 1;
            self.mem.stack[(self.mem.sp - 1) as usize] = self.mem.pc;
            self.mem.pc = new_pc_addr;
        }

        pub fn stack_pop(&mut self) {
            self.mem.pc = self.mem.stack[(self.mem.sp - 1) as usize];
            self.mem.stack[(self.mem.sp - 1) as usize] = 0;
            self.mem.sp -= 1;
        }

        // Loads the default sprites for the hexadecimal digits (0 to f) into memory starting at address 0,
        // with each bit of the byte representing the state of a pixel (ON/OFF).
        //
        // Example - The full sprite representation of the number 0 is composed of the following:
        // 1st byte -> 11110000
        // 2nd byte -> 10010000
        // 3rd byte -> 10010000
        // 4th byte -> 10010000
        // 5th byte -> 11110000
        fn load_hex_digits(&mut self) {
            let hex_digits: [u8;80] = [
                0xF0, 0x90, 0x90, 0x90, 0xF0,
                0x20, 0x60, 0x20, 0x20, 0x70,
                0xF0, 0x10, 0xF0, 0x80, 0xF0,
                0xF0, 0x10, 0xF0, 0x10, 0xF0,
                0x90, 0x90, 0xF0, 0x10, 0x10,
                0xF0, 0x80, 0xF0, 0x10, 0xF0,
                0xF0, 0x80, 0xF0, 0x90, 0xF0,
                0xF0, 0x10, 0x20, 0x40, 0x40,
                0xF0, 0x90, 0xF0, 0x90, 0xF0,
                0xF0, 0x90, 0xF0, 0x10, 0xF0,
                0xF0, 0x90, 0xF0, 0x90, 0x90,
                0xE0, 0x90, 0xE0, 0x90, 0xE0,
                0xF0, 0x80, 0x80, 0x80, 0xF0,
                0xE0, 0x90, 0x90, 0x90, 0xE0,
                0xF0, 0x80, 0xF0, 0x80, 0xF0,
                0xF0, 0x80, 0xF0, 0x80, 0x80];
            
            for i in 0..hex_digits.len() {
                self.mem.ram[i] = hex_digits[i];
            }
        }

        fn load_rom(&mut self, path: &str) {
            let mut byte_vec = Vec::new();
            File::open(path).unwrap()
                .read_to_end(&mut byte_vec).unwrap();

            
            if byte_vec.len() > 3232 {
                panic!("Selected ROM size is greater than the available RAM!");
            }

            let mut address = self.mem.pc as usize;
            for byte in byte_vec {
                self.mem.ram[address] = byte;
                address += 1;
            }
        }
    }
}
