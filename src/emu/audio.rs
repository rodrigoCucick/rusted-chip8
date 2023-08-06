// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2023 Rodrigo M. Cucick <r_monfredini@hotmail.com>

pub mod buzzer {
    use sdl2::audio::{ AudioCallback, AudioDevice, AudioSpecDesired };
    use sdl2::AudioSubsystem;

    pub struct BuzzerController {
        device: AudioDevice<SquareWave>,
        st_equals_buzzer: bool,
        is_playing: bool,
    }

    impl BuzzerController {
        pub fn new_square_wave_buzzer(audio_subsystem: AudioSubsystem, st_equals_buzzer: bool) -> Self {
            let desired_spec = AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: None
            };
    
            let device = audio_subsystem.open_playback(
                None,
                &desired_spec,
                |spec| {
                SquareWave {
                    phase: 0.0,
                    phase_inc: 440.0 / spec.freq as f32,
                    volume: 0.25
                }
            }).unwrap();

            Self {
                device,
                st_equals_buzzer,
                is_playing: false,
            }
        }

        pub fn play_based_on_st(&mut self, st: u8) {
            if !self.st_equals_buzzer {
                return;
            }

            match st {
                0 => self.pause(),
                _ => self.play()
            }
        }

        fn play(&mut self) {
            if !self.is_playing {
                self.device.resume();
                self.is_playing = true;
            }
        }

        fn pause(&mut self) {
            if self.is_playing {
                self.device.pause();
                self.is_playing = false;
            }
        }
    }

    struct SquareWave {
        phase: f32,
        phase_inc: f32,
        volume: f32,
    }
    
    impl AudioCallback for SquareWave {
        type Channel = f32;
    
        fn callback(&mut self, out: &mut [f32]) {
            for x in out.iter_mut() {
                *x = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }
}
