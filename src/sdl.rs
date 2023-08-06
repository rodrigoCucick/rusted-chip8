// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod wrapper {
    use crate::emu::memory::memory::MemoryController;
    use crate::util::utilities::Math2d;

    use sdl2::{ AudioSubsystem, EventPump, Sdl };
    use sdl2::image::LoadSurface;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::surface::Surface;
    use sdl2::video::Window;

    pub struct CustomWindow {
        sdl_context: Sdl,
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
        pixel_color: Color,
    }

    impl CustomWindow {
        pub fn new(
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

    pub struct SDLController {
        window: CustomWindow,
    }

    impl SDLController {
        pub fn new(window: CustomWindow) -> Self {
            Self { window }
        }

        // put_pixel() is called by the Dxyn instruction.
        pub fn put_pixel(&mut self, x: u8, y: u8, mem_ctrl: &mut MemoryController){
            let corrected_x = Math2d::wrap_coord(x, self.window.win_w);
            let corrected_y = Math2d::wrap_coord(y, self.window.win_h);
            let pixel_vec_i = (self.window.win_w * corrected_y as u32 + corrected_x as u32) as usize;

            // Drawing color is white if drawing on top of an OFF pixel, black otherwise.
            let draw_color: Color =
                if self.window.pixel_vec[pixel_vec_i] == 0 {
                    self.window.pixel_color
                } else {
                    if mem_ctrl.get_v(0xf) == 0 {
                        mem_ctrl.set_v(0xf, 1); // vf = collision flag
                    }
                    self.window.bg_color
                };

            self.window.canvas.set_draw_color(draw_color);
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

        pub fn display_canvas(&mut self) {
            self.window.canvas.present();
        }

        pub fn get_audio_subsystem(&self) ->AudioSubsystem {
            self.window.sdl_context.audio().unwrap()
        }

        pub fn get_event_pump(&self) -> EventPump {
            self.window.sdl_context.event_pump().unwrap()
        }

        pub fn get_window(&self) -> &CustomWindow {
            &self.window
        }

        pub fn set_canvas_scale(&mut self) {
            self.window.canvas.set_scale(
                self.window.scale as f32,
                self.window.scale as f32)
                .unwrap()
        }
    }    
}
