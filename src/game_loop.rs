//! Module with all the wiring of components, including the main game loop

use crate::machine::Chip8;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
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
        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break 'game,
                    KeyCode::Char('1') => chip8.keypad.press_key(0x1).unwrap(),
                    KeyCode::Char('!') => chip8.keypad.release_key(0x1).unwrap(),
                    _ => {}
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

            for row in current_display_state {
                for &px in row {
                    let symbol = if px { "██" } else { "  " };
                    queue!(stdout, crossterm::style::Print(symbol))?;
                }
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
