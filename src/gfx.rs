// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod graphics {
    use crate::aud::audio::Buzzer;
    use crate::cpu::cpu::CpuController;
    use crate::kbrd::keyboard::KeyboardController;
    use crate::mem::memory::MemoryController;
    use crate::util::utilities::Math2d;

    use sdl2::event::Event;
    use sdl2::image::LoadSurface;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::surface::Surface;
    use sdl2::video::Window;
    use std::time::Duration;

    pub struct CustomWindow {
        sdl_context: sdl2::Sdl,
        win_w: u32,
        win_h: u32,
        scale: u32,
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
            let sdl_context = sdl2::init().unwrap();
            let win_w_scaled = win_w * scale;
            let win_h_scaled = win_h * scale;

            let mut canvas = sdl_context.video().unwrap()
                .window(win_title, win_w_scaled, win_h_scaled).position_centered().build().unwrap()
                .into_canvas().build().unwrap();

            if let Ok(win_icon) = Surface::from_file(".\\assets\\img\\icon-64x64.png") {
                canvas.window_mut().set_icon(win_icon);
            }
            
            Self {
                sdl_context,
                win_w,
                win_h,
                scale,
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

        // TODO: Refactor to detach functionalities.
        pub fn render_and_listen_events(
            &mut self,
            mem_ctrl: &mut MemoryController,
            keyboard_ctrl: &mut KeyboardController,
            cpu_ctrl: &mut CpuController,
            dt_equals_buzzer: bool) {
            let mut event_pump = self.window.sdl_context.event_pump().unwrap();
            let mut buzzer = Buzzer::new_square_wave_buzzer(&self.window.sdl_context);

            self.window.canvas.set_scale(self.window.scale as f32, self.window.scale as f32).unwrap();
            self.clear_screen();

            'running: loop {
                for _ in 0..cpu_ctrl.get_cycles_per_frame() {
                    for event in event_pump.poll_iter() {
                        match event {
                            // TODO: Maybe this logic could be more generic.
                            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                            Event::KeyDown { keycode: Some(Keycode::Kp0), .. } => keyboard_ctrl.set_key_down_x(0),
                            Event::KeyDown { keycode: Some(Keycode::Kp1), .. } => keyboard_ctrl.set_key_down_x(1),
                            Event::KeyDown { keycode: Some(Keycode::Kp2), .. } => keyboard_ctrl.set_key_down_x(2),
                            Event::KeyDown { keycode: Some(Keycode::Kp3), .. } => keyboard_ctrl.set_key_down_x(3),
                            Event::KeyDown { keycode: Some(Keycode::Kp4), .. } => keyboard_ctrl.set_key_down_x(4),
                            Event::KeyDown { keycode: Some(Keycode::Kp5), .. } => keyboard_ctrl.set_key_down_x(5),
                            Event::KeyDown { keycode: Some(Keycode::Kp6), .. } => keyboard_ctrl.set_key_down_x(6),
                            Event::KeyDown { keycode: Some(Keycode::Kp7), .. } => keyboard_ctrl.set_key_down_x(7),
                            Event::KeyDown { keycode: Some(Keycode::Kp8), .. } => keyboard_ctrl.set_key_down_x(8),
                            Event::KeyDown { keycode: Some(Keycode::Kp9), .. } => keyboard_ctrl.set_key_down_x(9),
                            Event::KeyDown { keycode: Some(Keycode::A), .. } =>   keyboard_ctrl.set_key_down_x(0xa),
                            Event::KeyDown { keycode: Some(Keycode::B), .. } =>   keyboard_ctrl.set_key_down_x(0xb),
                            Event::KeyDown { keycode: Some(Keycode::C), .. } =>   keyboard_ctrl.set_key_down_x(0xc),
                            Event::KeyDown { keycode: Some(Keycode::D), .. } =>   keyboard_ctrl.set_key_down_x(0xd),
                            Event::KeyDown { keycode: Some(Keycode::E), .. } =>   keyboard_ctrl.set_key_down_x(0xe),
                            Event::KeyDown { keycode: Some(Keycode::F), .. } =>   keyboard_ctrl.set_key_down_x(0xf),
                            Event::KeyUp { keycode: Some(Keycode::Kp0), .. } =>   keyboard_ctrl.set_key_up_x(0),
                            Event::KeyUp { keycode: Some(Keycode::Kp1), .. } =>   keyboard_ctrl.set_key_up_x(1),
                            Event::KeyUp { keycode: Some(Keycode::Kp2), .. } =>   keyboard_ctrl.set_key_up_x(2),
                            Event::KeyUp { keycode: Some(Keycode::Kp3), .. } =>   keyboard_ctrl.set_key_up_x(3),
                            Event::KeyUp { keycode: Some(Keycode::Kp4), .. } =>   keyboard_ctrl.set_key_up_x(4),
                            Event::KeyUp { keycode: Some(Keycode::Kp5), .. } =>   keyboard_ctrl.set_key_up_x(5),
                            Event::KeyUp { keycode: Some(Keycode::Kp6), .. } =>   keyboard_ctrl.set_key_up_x(6),
                            Event::KeyUp { keycode: Some(Keycode::Kp7), .. } =>   keyboard_ctrl.set_key_up_x(7),
                            Event::KeyUp { keycode: Some(Keycode::Kp8), .. } =>   keyboard_ctrl.set_key_up_x(8),
                            Event::KeyUp { keycode: Some(Keycode::Kp9), .. } =>   keyboard_ctrl.set_key_up_x(9),
                            Event::KeyUp { keycode: Some(Keycode::A), .. } =>     keyboard_ctrl.set_key_up_x(0xa),
                            Event::KeyUp { keycode: Some(Keycode::B), .. } =>     keyboard_ctrl.set_key_up_x(0xb),
                            Event::KeyUp { keycode: Some(Keycode::C), .. } =>     keyboard_ctrl.set_key_up_x(0xc),
                            Event::KeyUp { keycode: Some(Keycode::D), .. } =>     keyboard_ctrl.set_key_up_x(0xd),
                            Event::KeyUp { keycode: Some(Keycode::E), .. } =>     keyboard_ctrl.set_key_up_x(0xe),
                            Event::KeyUp { keycode: Some(Keycode::F), .. } =>     keyboard_ctrl.set_key_up_x(0xf),
                            _ => {}
                        }
                    }

                    cpu_ctrl.fetch_exec(keyboard_ctrl, mem_ctrl, self);

                    if dt_equals_buzzer {
                        buzzer.play_based_on_dt(mem_ctrl.get_dt());
                    }
                }
                
                mem_ctrl.dec_all_timers();

                self.window.canvas.present();
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }

        // put_pixel() is called by the Dxyn instruction.
        pub fn put_pixel(&mut self, x: u8, y: u8, vf: &mut u8){
            let corrected_x = Math2d::wrap_coord(x, self.window.win_w);
            let corrected_y = Math2d::wrap_coord(y, self.window.win_h);
            let pixel_vec_i = (self.window.win_w * corrected_y as u32 + corrected_x as u32) as usize;

            // Drawing color is white if drawing on top of an OFF pixel, black otherwise.
            self.window.canvas.set_draw_color(
                if self.window.pixel_vec[pixel_vec_i] == 0 {
                    self.window.pixel_color
                } else {
                    *vf = 1; // Set collision flag to 1 on the current sprite drawing routine.
                    self.window.bg_color
                }
            );

            self.window.canvas.draw_point(Point::new(corrected_x as i32, corrected_y as i32)).unwrap();

            self.window.canvas.set_draw_color(self.window.bg_color);

            // Flips the current pixel's virtual representation.
            self.window.pixel_vec[pixel_vec_i] = if self.window.pixel_vec[pixel_vec_i] == 0 { 1 } else { 0 };
        }

        pub fn clear_screen(&mut self) {
            self.window.pixel_vec.fill(0);
            self.window.canvas.set_draw_color(self.window.bg_color);
            self.window.canvas.clear();
        }
    }    
}
