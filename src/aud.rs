pub mod audio {
    use sdl2::audio::{ AudioCallback, AudioSpecDesired, AudioDevice };
    use sdl2::Sdl;

    pub struct Buzzer {
        device: AudioDevice<SquareWave>,
        is_playing: bool,
    }

    impl Buzzer {
        pub fn new_square_wave_buzzer(sdl_context: &Sdl) -> Self {
            let audio_subsystem = sdl_context.audio().unwrap();

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
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25
                }
            }).unwrap();

            Self {
                device,
                is_playing: false
            }
        }

        pub fn play_based_on_dt(&mut self, dt: u8) {
            match dt {
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

    pub struct SquareWave {
        pub phase_inc: f32,
        pub phase: f32,
        pub volume: f32
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
