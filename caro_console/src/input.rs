use std::{io, sync::atomic::{AtomicBool, Ordering}};

use tokio::{self, io::AsyncBufReadExt};

use crate::types;

#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    Up,
    Down,
    Left,
    Right,
    Esc,
    Invalid,
}

#[derive(Debug, Clone)]
pub enum InputType {
    Text(String),
    Key(KeyType),
}

static IS_PROMPT_MODE: AtomicBool = AtomicBool::new(true);

pub fn enable_prompt_mode_at(latitude: types::Latitude, longtitude: types::Longtitude) {
    let mut stdout = io::stdout();
    let latitude = latitude.max(0) as u16;
    let longtitude = longtitude.max(0) as u16;
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = crossterm::execute!(
        stdout,
        crossterm::cursor::Show,
        crossterm::cursor::MoveTo(longtitude, latitude),
    );
    IS_PROMPT_MODE.store(true, Ordering::Release);
}

pub fn disable_prompt_mode() {
    let mut stdout = io::stdout();
    let _ = crossterm::terminal::enable_raw_mode();
    let _ = crossterm::execute!(stdout, crossterm::cursor::Hide);
    IS_PROMPT_MODE.store(false, Ordering::Release);
}

pub fn is_prompt_mode() -> bool {
    IS_PROMPT_MODE.load(Ordering::Acquire)
}

pub async fn get_user_input() -> InputType {
    if is_prompt_mode() {
        let mut reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();
        if let Some(line) = reader.next_line().await.unwrap() {
            InputType::Text(line)
        } else {
            InputType::Text("".to_string())
        }
    } else {
        // Use crossterm to read a key event in raw mode
        loop {
            if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap() {
                    if key_event.kind == crossterm::event::KeyEventKind::Press {
                        return InputType::Key(match key_event.code {
                            crossterm::event::KeyCode::Up => KeyType::Up,
                            crossterm::event::KeyCode::Down => KeyType::Down,
                            crossterm::event::KeyCode::Left => KeyType::Left,
                            crossterm::event::KeyCode::Right => KeyType::Right,
                            crossterm::event::KeyCode::Esc => KeyType::Esc,
                            _ => KeyType::Invalid,
                        });
                    }
                }
            } else {
                // Yield to the async runtime
                tokio::task::yield_now().await;
            }
        }
    }
}