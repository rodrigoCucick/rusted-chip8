// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod graphics {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::video::Window;
    use std::time::Duration;

    use crate::emu::emulator::{ CpuController, MemoryController };
    use crate::util::utilities::Math2d;

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
        pixel_vec: Vec<u8>,
        bg_color: Color,
        pixel_color: Color
    }

    impl CustomWindow {
        pub fn create_and_display_window(
            win_title: &str,
            win_w: u32,
            win_h: u32,
            scale: u32,
            bg_color: Color,
            pixel_color: Color) -> Self {
            let sdl_context = match sdl2::init() {
                Ok(sdl) => sdl,
                Err(_) => panic!("Couldn't initialize SDL2.")
            };

            let win_w_scaled = win_w * scale;
            let win_h_scaled = win_h * scale;

            let canvas = match sdl_context.video() {
                Ok(video_subsystem) => {
                    match video_subsystem.window(win_title, win_w_scaled, win_h_scaled)
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
                win_w,
                win_h,
                scale,
                win_w_scaled,
                win_h_scaled,
                canvas,
                pixel_vec: {
                    let mut vec: Vec<u8> = Vec::new();
                    vec.resize((win_w * win_h) as usize, 0);
                    vec.fill(0);
                    vec
                },
                bg_color,
                pixel_color,
            }
        }
    }

    pub struct CustomWindowController {
        window: CustomWindow,
    }

    impl CustomWindowController {
        pub fn new(window: CustomWindow) -> Self {
            Self { window }
        }

        pub fn render_and_handle_inputs(&mut self, mem_ctrl: &mut MemoryController, cpu_ctrl: &mut CpuController) {
            let mut event_pump = match self.window.sdl_context.event_pump() {
                Ok(event_pump) => event_pump,
                Err(_) => panic!("Couldn't obtain the EventPump.")
            };

            if let Err(_) = self.window.canvas.set_scale(self.window.scale as f32, self.window.scale as f32) {
                panic!("Couldn't set the canvas' drawing scale.");
            };


            self.clear_screen();

            // Game program loop.
            'running: loop {
                // TODO: Review cycles per frame and framerate logic.
                for _ in 0..cpu_ctrl.get_cycles_per_frame() {
                    cpu_ctrl.exec_next_instr(mem_ctrl, self);
                }
                
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        // TODO: KeyDown events need to be added for the system's hex keyboard (keys 0 to f).
                        _ => {}
                    }
                }
        
                self.window.canvas.present();
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }

        // put_pixel() is called by the Dxyn instruction.
        pub fn put_pixel(&mut self, x: u8, y: u8, vf: &mut u8){
            let corrected_x = Math2d::wrap_coord(x, self.window.win_w);
            let corrected_y = Math2d::wrap_coord(y, self.window.win_h);
            let pixel_vec_i = (self.window.win_w * corrected_y as u32 + corrected_x as u32) as usize;

            // Drawing color is white if drawing on top of an OFF pixel, black otherwise (enforces XOR).
            self.window.canvas.set_draw_color(
                if self.window.pixel_vec[pixel_vec_i] == 0 {
                    self.window.pixel_color
                } else {
                    *vf = 1; // Set collision flag to 1 on the current sprite drawing routine.
                    self.window.bg_color
                }
            );

            if let Err(_) = self.window.canvas.draw_point(Point::new(corrected_x as i32, corrected_y as i32)) {
                panic!("Couldn't draw on the canvas!");
            };

            self.window.canvas.set_draw_color(self.window.bg_color);

            // Flips the current pixel's virtual representation.
            self.window.pixel_vec[pixel_vec_i] = if self.window.pixel_vec[pixel_vec_i] == 0 { 1 } else { 0 };
        }

        pub fn clear_screen(&mut self) {
            self.window.canvas.set_draw_color(self.window.bg_color);
            self.window.canvas.clear();
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
    }    
}
