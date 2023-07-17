// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod emu;
pub mod gfx;

use emu::emulator::Memory;
use gfx::graphics::{ CustomWindow, WindowController };

fn main() {
    let custom_window = CustomWindow::create_and_display_window(
        "Rusted - Chip-8 Emulator/Interpreter",
        64,
        32,
        10);

    let mut window_controller = WindowController::new(custom_window);

    let mut mem = Memory::new();
    // TODO: Add a method for custom file selection.
    mem.init_ram("./game-program/demo.ch8");

    window_controller.render_and_handle_inputs(&mut mem);
}
