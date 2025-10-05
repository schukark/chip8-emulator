//! Module that contains the window logic

use std::{
    env::Args,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source, source::SineWave};
use tklog::{info, trace};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{
        KeyCode,
        PhysicalKey::{Code, Unidentified},
    },
    window::{Window, WindowAttributes, WindowId},
};

use crate::machine::Chip8;

/// The time interval for 60hz (timers for chip8 operate on 60hz)
const TIMER_INTERVAL: Duration = Duration::from_micros(16667);

/// The time interval for 500hz (simulate chip8 cpu at 500hz)
const CPU_CYCLE_INTERVAL: Duration = Duration::from_micros(2000);

/// The main emulator application
struct App<'a> {
    /// Application's window
    window: Option<Arc<Window>>,
    /// Window's ID
    window_id: Option<WindowId>,
    /// Pixels struct to draw on the window
    pixels: Option<Pixels<'a>>,
    /// Chip8 instance
    chip8: Chip8,
    /// Last time the timers were ticked down
    last_ticked: Instant,
    /// Last time the cpu cycle happened
    last_cpu_cycle: Instant,
    /// Hash of the last frame to consider rerendering
    prev_display_hash: Option<u64>,
    /// Stream handle for audio
    stream_handle: Option<OutputStream>,
    /// Audio sink
    sound_sink: Option<Arc<Mutex<Sink>>>,
}

impl<'a> App<'a> {
    /// Create an application struct from a ready chip8 instance
    fn from_chip8(chip8: Chip8) -> Self {
        Self {
            window: None,
            window_id: None,
            pixels: None,
            chip8,
            last_ticked: Instant::now(),
            last_cpu_cycle: Instant::now(),
            prev_display_hash: None,
            stream_handle: None,
            sound_sink: None,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        info!("Window created!");

        // Window creation
        let window_attributes = WindowAttributes::default()
            .with_title("Chip8 emulator")
            .with_inner_size(winit::dpi::LogicalSize::new(640, 320));
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("create application window"),
        );

        self.window_id = Some(window.id());
        self.window = Some(window.clone());

        // Rendering initialization
        let surface_texture = SurfaceTexture::new(640, 320, window);
        let pixels =
            Pixels::new(640, 320, surface_texture).expect("create a surface texture to draw");

        self.pixels = Some(pixels);

        // Audio initialization
        let stream_handle =
            OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Sink::connect_new(stream_handle.mixer());
        sink.pause();

        self.stream_handle = Some(stream_handle);
        self.sound_sink = Some(Arc::new(Mutex::new(sink)));
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        let next_frame = now + CPU_CYCLE_INTERVAL;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame));

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
                self.window = None;
                self.window_id = None;
                self.pixels = None;
                self.prev_display_hash = None;
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();

                if now.duration_since(self.last_ticked) >= TIMER_INTERVAL {
                    self.tick_timers_and_sound(now);
                }

                if now.duration_since(self.last_cpu_cycle) >= CPU_CYCLE_INTERVAL {
                    self.run_cpu_cycle(now);
                }

                self.maybe_redraw_display();

                event_loop.set_control_flow(ControlFlow::WaitUntil(now + CPU_CYCLE_INTERVAL));
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                trace!(format!("Detected event: {event:#?}"));

                if let Some((key, is_pressed)) = keymap(event) {
                    self.chip8.set_key_state(key, is_pressed).unwrap();
                }
            }
            _ => (),
        }
    }
}

impl<'a> App<'a> {
    /// Decay factor for the phosphor persitence
    const DECAY: f32 = 0.25;

    /// Determine if the current display is different from the last,
    /// and if yes, rerender
    fn maybe_redraw_display(&mut self) {
        if let Some(display_state) = self.chip8.display_snapshot().cloned() {
            let mut hasher = DefaultHasher::new();
            display_state.hash(&mut hasher);
            let cur_hash = hasher.finish();

            if self.prev_display_hash != Some(cur_hash) {
                self.draw_display(display_state);
                if let Some(pixels) = &self.pixels {
                    let _ = pixels.render();
                }
                self.prev_display_hash = Some(cur_hash);
            }
        }
    }

    /// Redraw the display depending on the current chip8 display state
    fn draw_display(&mut self, display_state: [[bool; 64]; 32]) {
        let scale_x = 10;
        let scale_y = 10;

        let frame = self.pixels.as_mut().unwrap().frame_mut();

        for (y, row) in display_state.iter().enumerate() {
            for (x, &on) in row.iter().enumerate() {
                let color = if on {
                    [0xFF, 0xFF, 0xFF, 0xFF]
                } else {
                    [0x00, 0x00, 0x00, 0xFF]
                };

                for dy in 0..scale_y {
                    for dx in 0..scale_x {
                        let px = x * scale_x + dx;
                        let py = y * scale_y + dy;
                        let i = (py * 640 + px) * 4;

                        // Phosphor persistence
                        for c in 0..3 {
                            let old = frame[i + c] as f32;
                            frame[i + c] =
                                ((old * App::DECAY) + color[c] as f32 * (1.0 - App::DECAY)) as u8;
                        }
                        frame[i + 3] = 0xFF; // Alpha channel
                    }
                }
            }
        }
    }

    /// Tick the cpu timers and play sound if needed
    fn tick_timers_and_sound(&mut self, now: Instant) {
        self.last_ticked = now;
        self.chip8.tick_timers();

        if !self.chip8.is_sound_playing() {
            self.pause_sound();
            return;
        }

        self.play_sound();
    }

    /// Play the sound
    fn play_sound(&self) {
        if let Some(lock) = &self.sound_sink
            && let Ok(sink) = lock.lock()
        {
            if sink.is_paused() {
                sink.play();
            }
            if sink.empty() {
                sink.append(SineWave::new(440.0).amplify(0.2).repeat_infinite());
            }
        }
    }

    /// Pause the current sound
    fn pause_sound(&self) {
        if let Some(lock) = &self.sound_sink
            && let Ok(sink) = lock.lock()
        {
            sink.pause();
        }
    }

    /// Run one cpu cycle
    fn run_cpu_cycle(&mut self, now: Instant) {
        if let Err(e) = self.chip8.step() {
            eprintln!("CHIP-8 execution error: {e}");
        }
        self.last_cpu_cycle = now;
    }
}

/// Runs the main application of the emulator
pub fn run_app(mut args: Args) -> anyhow::Result<()> {
    let program_path = args.nth(1).unwrap();
    let chip8 = load_program(program_path)?;

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + TIMER_INTERVAL));

    let mut app = App::from_chip8(chip8);
    let _ = event_loop.run_app(&mut app);

    Ok(())
}

/// Load the program from path and return a ready chip8 instance
fn load_program(path: String) -> anyhow::Result<Chip8> {
    let program = std::fs::read(path).expect("Error occured when opening rom");
    let mut chip8 = Chip8::new();
    chip8.load_program(&program)?;

    Ok(chip8)
}

/// Map the real input to hex keyboard of chip8
///
/// If the input is not present on the keyboard, returns Option::None
/// Also returns if the key is pressed (true) or not (false) as a second tuple argument
fn keymap(event: KeyEvent) -> Option<(u8, bool)> {
    let is_pressed = event.state.is_pressed();

    if let Unidentified(_) = event.physical_key {
        return None;
    }

    match event.physical_key {
        Code(key_code) => {
            let mapped = match key_code {
                KeyCode::Digit1 => Some(0x1),
                KeyCode::Digit2 => Some(0x2),
                KeyCode::Digit3 => Some(0x3),
                KeyCode::Digit4 => Some(0xC),

                KeyCode::KeyQ => Some(0x4),
                KeyCode::KeyW => Some(0x5),
                KeyCode::KeyE => Some(0x6),
                KeyCode::KeyR => Some(0xD),

                KeyCode::KeyA => Some(0x7),
                KeyCode::KeyS => Some(0x8),
                KeyCode::KeyD => Some(0x9),
                KeyCode::KeyF => Some(0xE),

                KeyCode::KeyZ => Some(0xA),
                KeyCode::KeyX => Some(0x0),
                KeyCode::KeyC => Some(0xB),
                KeyCode::KeyV => Some(0xF),

                _ => None,
            };
            mapped.map(|k| (k, is_pressed))
        }
        _ => None,
    }
}
