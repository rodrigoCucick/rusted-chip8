// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod emulator {
    use crate::emu::audio::buzzer::BuzzerController;
    use crate::emu::input::keyboard::{ CustomKeyEvent, KeyboardController };
    use crate::emu::logic::cpu::CpuController;
    use crate::emu::memory::memory::MemoryController;
    use crate::sdl::wrapper::SDLController;

    use std::time::Duration;

    pub struct EmuController;

    impl EmuController {
        pub fn run_emulator(
            sdl_ctrl: &mut SDLController,
            mem_ctrl: &mut MemoryController,
            cpu_ctrl: &mut CpuController,
            keyboard_ctrl: &mut KeyboardController,
            buzzer_ctrl: &mut BuzzerController) {

            let mut event_pump = sdl_ctrl.get_event_pump();

            sdl_ctrl.set_canvas_scale();
            sdl_ctrl.clear_screen();

            // TODO - At the moment, the timer variables are only being used to display information.
            //        I plan to change from std::thread::sleep() to a more fine control
            //        using the timer variables.

            let initial_time = std::time::Instant::now();
            loop {
                let frame_start_time = std::time::Instant::now();

                if let Some(CustomKeyEvent::Quit) = keyboard_ctrl.check_input_events(&mut event_pump) {
                    return;
                };

                for _ in 0..cpu_ctrl.get_cycles_per_frame() {
                    cpu_ctrl.fetch_exec(sdl_ctrl, mem_ctrl, keyboard_ctrl);
                }

                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

                buzzer_ctrl.play_based_on_st(mem_ctrl.get_st());
                mem_ctrl.dec_all_timers();
                sdl_ctrl.display_canvas();

                let frame_end_time = std::time::Instant::now();
                let total_frame_time = frame_end_time - frame_start_time;
                let total_running_time = frame_start_time - initial_time;
                
                print!("Frame time: {}ms\tElapsed time: {}s\r", total_frame_time.as_millis(), total_running_time.as_secs());
            }
        }
    }
}
