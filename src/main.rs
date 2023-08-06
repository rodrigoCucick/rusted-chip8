// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod emu;
pub mod sdl;
pub mod util;

use emu::{
    audio::buzzer::BuzzerController,
    config::settings::EmuSettings ,
    core_emu::emulator::EmuController,
    input::keyboard::{ Keyboard, KeyboardController },
    logic::cpu::CpuController,
    memory::memory::{ Memory, MemoryController }
};
use sdl::wrapper::*;
use std::env;
use util::utilities::FileSelectionUtil;

fn main() {
    println!("Rusted - Chip-8 Emulator/Interpreter");
    println!("2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>");
    println!("Repository: https://github.com/rodrigoCucick/rusted-chip8");
    println!("Settings can be adjusted via 'config.txt'.");

    let emu_settings = EmuSettings::new();

    let mut roms_path = String::from(env::current_dir().unwrap().to_str().unwrap());
    roms_path.push_str(emu_settings.get_default_ch8_folder());

    let roms = match FileSelectionUtil::get_files_in_directory(&roms_path) {
        Ok(file_names) => file_names,
        Err(_) => panic!("Invalid folder provided to 'default_ch8_folder' in 'config.txt'!")
    };

    roms_path.push_str(roms.get(FileSelectionUtil::file_selection_menu(&roms)).unwrap());

    let mut sdl_ctrl =
        SDLController::new(CustomWindow::new(
            "Rusted - Chip-8 Emulator/Interpreter",
            64,
            32,
            emu_settings.get_scale(),
            emu_settings.get_bg_color(),
            emu_settings.get_pixel_color()
        ));

    let mut mem_ctrl = MemoryController::new(Memory::new());
    mem_ctrl.init_ram(&roms_path);

    let mut cpu_ctrl = CpuController::new(
        &mem_ctrl,
        emu_settings.get_cycles_per_frame(),
        emu_settings.get_bit_shift_instructions_use_vy(),
        emu_settings.get_store_read_instructions_change_i());

    let mut keyboard_ctrl = KeyboardController::new(Keyboard::new());

    let mut buzzer_ctrl = BuzzerController::new_square_wave_buzzer(
        sdl_ctrl.get_audio_subsystem(),
        emu_settings.get_st_equals_buzzer());

    EmuController::run_emulator(
        &mut sdl_ctrl,
        &mut mem_ctrl,
        &mut cpu_ctrl,
        &mut keyboard_ctrl,
        &mut buzzer_ctrl);
}
