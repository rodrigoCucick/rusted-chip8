// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod aud;
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
use util::utilities::EmuFile;

fn main() {
    let emu_config = EmulatorConfiguration::new();

    let mut roms_path = String::from(env::current_dir().unwrap().to_str().unwrap());
    roms_path.push_str(emu_config.get_default_ch8_folder());

    let roms = match EmuFile::get_files_in_directory(&roms_path) {
        Ok(file_names) => file_names,
        Err(_) => panic!("Invalid folder provided to 'default_ch8_folder' in 'config.txt'!")
    };

    roms_path.push_str(roms.get(EmuFile::file_selection_menu(&roms)).unwrap());

    let mut window_controller =
        CustomWindowController::new(CustomWindow::create_and_display_window(
            "Rusted - Chip-8 Emulator/Interpreter",
            64,
            32,
            emu_config.get_scale(),
            emu_config.get_bg_color(),
            emu_config.get_pixel_color()
        ));

    let mut mem_ctrl = MemoryController::new(Memory::new());
    mem_ctrl.init_ram(&roms_path);

    let mut keyboard_ctrl = KeyboardController::new(Keyboard::new());

    let mut cpu_ctrl = CpuController::new(&mem_ctrl, emu_config.get_cycles_per_frame());

    window_controller.render_and_listen_events(&mut mem_ctrl, &mut keyboard_ctrl, &mut cpu_ctrl, emu_config.get_dt_equals_buzzer());
}
