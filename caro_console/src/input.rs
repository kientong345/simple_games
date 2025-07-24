use std::{io, sync::atomic::{AtomicBool, Ordering}};

use tokio::{self, io::AsyncBufReadExt};

use crate::types;

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

pub async fn get_user_input() -> String {
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    if let Some(line) = reader.next_line().await.unwrap() {
        line
    } else {
        "".to_string()
    }
}