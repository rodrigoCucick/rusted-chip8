// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

#![allow(warnings)]
pub mod graphics {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::video::Window;
    use std::time::Duration;

    use crate::emu::emulator::{ Cpu, Memory };

    pub struct CustomWindow {
        sdl_context: sdl2::Sdl,
        win_w: u32,
        win_h: u32,
        scale: u32,
        win_w_scaled: u32,
        win_h_scaled: u32,
        canvas: Canvas<Window>,
        // pixel_vec is used to represent the pixels on the screen
        // for extremely fast collision checking (renderer independent).
        // Since it's a 1d vector and the canvas is 2d, it's always indexed with the following formula:
        // window width * y + x
        pixel_vec: Vec<u8> 
    }

    impl CustomWindow {
        pub fn create_and_display_window(window_title: &str, window_width: u32, window_height: u32, scale: u32) -> Self {
            let sdl_context = match sdl2::init() {
                Ok(sdl) => sdl,
                Err(_) => panic!("Couldn't initialize SDL2.")
            };

            let window_width_scaled = window_width * scale;
            let window_height_scaled = window_height * scale;

            let canvas = match sdl_context.video() {
                Ok(video_subsystem) => {
                    match video_subsystem.window(window_title, window_width_scaled, window_height_scaled)
                        .position_centered()
                        .build() {
                            Ok(window_builder) => {
                                match window_builder.into_canvas().build() {
                                    Ok(canvas) => canvas,
                                    Err(_) => panic!("Couldn't build the canvas/renderer.")
                                }
                            },
                            Err(_) => panic!("Couldn't build the window.")
                    }
                },
                Err(_) => panic!("Couldn't initialize SDL2 video subsystem.")
            };
            
            Self {
                sdl_context,
                win_w: window_width,
                win_h: window_height,
                scale,
                win_w_scaled: window_width_scaled,
                win_h_scaled: window_height_scaled,
                canvas,
                pixel_vec: {
                    let mut vec: Vec<u8> = Vec::new();
                    vec.resize((window_width * window_height) as usize, 0);
                    vec.fill(0);
                    vec
                }
            }
        }
    }

    pub struct WindowController {
        window: CustomWindow,
    }

    impl WindowController {
        pub fn new(window: CustomWindow) -> Self {
            Self { window }
        }

        pub fn render_and_handle_inputs(&mut self, memory: &mut Memory) {
            let mut event_pump = self.window.sdl_context.event_pump().unwrap();

            self.window.canvas.set_scale(self.window.scale as f32, self.window.scale as f32).unwrap();
            self.window.canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
            self.window.canvas.clear();

            // Game program loop.
            'running: loop {
                Cpu::execute_next_instruction(memory, self);

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        // TODO: KeyDown events need to be added for the system's hex keyboard (keys 0 to f).
                        _ => {}
                    }
                }
        
                self.window.canvas.present();
                std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
            }
        }

        // put_pixel() is called by the Dxyn (DRW Vx, Vy, nibble) instruction.
        pub fn put_pixel(&mut self, x: u8, y: u8, vf: &mut u8){
            let corrected_x = WindowController::wrap_coord(x, self.window.win_w);
            let corrected_y = WindowController::wrap_coord(y, self.window.win_h);
            let pixel_vec_i = (self.window.win_w * corrected_y as u32 + corrected_x as u32) as usize;

            // Drawing color is white if drawing on top of an OFF pixel, black otherwise (enforces XOR).
            self.window.canvas.set_draw_color(
                if self.window.pixel_vec[pixel_vec_i] == 0 {
                    Color::RGB(0xff, 0xff, 0xff)
                } else {
                    *vf = 1; // Set collision flag to 1 on the current sprite drawing routine.
                    Color::RGB(0x00, 0x00, 0x00)
                }
            );

            self.window.canvas.draw_point(Point::new(corrected_x as i32, corrected_y as i32)).unwrap();

            // Flips the current pixel's virtual representation.
            self.window.pixel_vec[pixel_vec_i] = if self.window.pixel_vec[pixel_vec_i] == 0 { 1 } else { 0 };
        }

        // FIXME: resize_window() was originally called on the F1 key KeyDown event,
        //        but it produces some artifacts due to the screen not being cleared/redrawn on each frame.
        //        It will be adjusted to independently redraw the scaled canvas (or it will be removed).
        fn resize_window(&mut self) {
            self.window.scale = if self.window.scale > 20 { 10 } else { self.window.scale + 2 };
            self.window.win_w_scaled = self.window.win_w * self.window.scale;
            self.window.win_h_scaled = self.window.win_h * self.window.scale;

            if let Err(_) = self.window.canvas.set_scale(
                self.window.scale as f32,
                self.window.scale as f32) {
                panic!("Couldn't set the canvas' new scale value.");
            }

            if let Err(_) = self.window.canvas.window_mut().set_size(
                self.window.win_w_scaled,
                self.window.win_h_scaled) {
                panic!("Couldn't resize the window.");
            }
        }

        fn wrap_coord(axis: u8, win_size: u32) -> u8 {
            if axis as u32 > win_size - 1 { axis % win_size as u8 } else { axis }
        }
    }    
}
