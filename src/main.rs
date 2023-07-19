// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod emu;
pub mod gfx;
pub mod util;

use emu::emulator::{ Memory, MemoryController, CpuController };
use gfx::graphics::{ CustomWindow, CustomWindowController };
use sdl2::pixels::Color;
use std::env;

fn main() {
    let mut window_controller =
        CustomWindowController::new(CustomWindow::create_and_display_window(
            "Rusted - Chip-8 Emulator/Interpreter",
            64,
            32,
            10,
            Color::RGB(0x00, 0x00, 0x00),
            Color::RGB(0xff, 0xff, 0xff)
        ));
    
    // TODO: Add another method for file selection (command line?).
    let mut game_program_path = String::from(env::current_dir().unwrap().to_str().unwrap());
    game_program_path.push_str("\\game-program\\demo.ch8");
    let mut mem_ctrl = MemoryController::new(Memory::new());
    mem_ctrl.init_ram(&game_program_path);

    let mut cpu_ctrl = CpuController::new(&mem_ctrl, 7);

    window_controller.render_and_handle_inputs(&mut mem_ctrl, &mut cpu_ctrl);
}
