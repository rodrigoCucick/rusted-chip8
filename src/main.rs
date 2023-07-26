// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod conf;
pub mod cpu;
pub mod gfx;
pub mod kbrd;
pub mod mem;
pub mod util;

use conf::config::EmulatorConfiguration;
use cpu::cpu::CpuController;
use gfx::graphics::*;
use kbrd::keyboard::*;
use mem::memory::*;
use std::env;

fn main() {
    let emu_config = EmulatorConfiguration::new();
    
    let mut window_controller =
        CustomWindowController::new(CustomWindow::create_and_display_window(
            "Rusted - Chip-8 Emulator/Interpreter",
            64,
            32,
            emu_config.get_scale(),
            emu_config.get_bg_color(),
            emu_config.get_pixel_color()
        ));
    
    let mut rom_path = String::from(env::current_dir().unwrap().to_str().unwrap());
    rom_path.push_str("\\");
    rom_path.push_str(emu_config.get_default_ch8_folder());
    // TODO: Let the user decide which ROM file to load from folder via CLI.
    rom_path.push_str("\\default.ch8"); 
    
    let mut mem_ctrl = MemoryController::new(Memory::new());
    mem_ctrl.init_ram(&rom_path);

    let mut keyboard_ctrl = KeyboardController::new(Keyboard::new());

    let mut cpu_ctrl = CpuController::new(&mem_ctrl, emu_config.get_cycles_per_frame());

    window_controller.render_and_listen_events(&mut mem_ctrl, &mut keyboard_ctrl, &mut cpu_ctrl);
}
