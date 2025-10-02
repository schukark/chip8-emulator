//! Module with all the wiring of components, including the main game loop

use crate::machine::Chip8;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    terminal::{self, ClearType},
};
use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};

/// Run the game loop
pub fn run_game(chip8: &mut Chip8) -> io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut last_timer_tick = Instant::now();
    let mut last_display_state = [[false; 64]; 32];
    'game: loop {
        // Read input

        for k in 0..16 {
            chip8.keypad.set_key_state(k as u8, false).unwrap();
        }

        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(event_key) = event::read()? {
                if let KeyCode::Enter = event_key.code {
                    break 'game;
                }

                if let Some((key, true)) = keymap(event_key) {
                    chip8.keypad.set_key_state(key, true).unwrap();
                }
            }
        }

        chip8.step().unwrap();

        if last_timer_tick.elapsed() >= Duration::from_millis(16) {
            chip8.tick_timers();
            last_timer_tick = Instant::now();
        }

        let current_display_state = chip8.display_snapshot();

        if *current_display_state != last_display_state {
            queue!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;

            queue!(stdout, crossterm::style::Print("\r\n"))?;
            for row in current_display_state {
                queue!(stdout, crossterm::style::Print(" "))?;
                for &px in row {
                    let symbol = if px { "██" } else { "  " };
                    queue!(stdout, crossterm::style::Print(symbol))?;
                }
                queue!(stdout, crossterm::style::Print(" "))?;
                queue!(stdout, crossterm::style::Print("\r\n"))?;
            }

            last_display_state = *current_display_state;
        }

        stdout.flush()?;
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Helper function to load the program and return a ready Chip8 instance
pub fn load_program(program: &[u8]) -> Result<Chip8, String> {
    let mut chip8 = Chip8::new();

    match chip8.load_program(program) {
        Ok(_) => Ok(chip8),
        Err(e) => Err(e.to_string()),
    }
}

/// Map the real input to Chip8 Hex keyboard inputs
///
/// Returns None if the event is not of relevance to the hex keyboard,
/// Some((key, value)) if key was influenced,
/// value shows if it was pressed (true) or released (false)
pub fn keymap(event: KeyEvent) -> Option<(u8, bool)> {
    let pressed = match event.kind {
        KeyEventKind::Press => true,
        KeyEventKind::Release => false,
        KeyEventKind::Repeat => return None, // ignore repeats
    };

    match event.code {
        KeyCode::Char('1') => Some((0x1, pressed)),
        KeyCode::Char('2') => Some((0x2, pressed)),
        KeyCode::Char('3') => Some((0x3, pressed)),
        KeyCode::Char('4') => Some((0xC, pressed)),

        KeyCode::Char('q') => Some((0x4, pressed)),
        KeyCode::Char('w') => Some((0x5, pressed)),
        KeyCode::Char('e') => Some((0x6, pressed)),
        KeyCode::Char('r') => Some((0xD, pressed)),

        KeyCode::Char('a') => Some((0x7, pressed)),
        KeyCode::Char('s') => Some((0x8, pressed)),
        KeyCode::Char('d') => Some((0x9, pressed)),
        KeyCode::Char('f') => Some((0xE, pressed)),

        KeyCode::Char('z') => Some((0xA, pressed)),
        KeyCode::Char('x') => Some((0x0, pressed)),
        KeyCode::Char('c') => Some((0xB, pressed)),
        KeyCode::Char('v') => Some((0xF, pressed)),

        _ => None,
    }
}
