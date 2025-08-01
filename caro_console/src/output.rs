use std::{io, sync::RwLock};

use crossterm;
use ratatui;

use crate::artworks::ArtDimension;

use crate::artworks;

use crate::types;

pub type Opacity = u8;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White(Opacity),
    Black(Opacity),
    Red(Opacity),
    Blue(Opacity),
    Yellow(Opacity),
    Magenta(Opacity),
    Green(Opacity),
    Cyan(Opacity),
}

#[derive(Debug, Clone)]
pub struct DrawableBox {
    pub coordinate: (types::Latitude, types::Longtitude),
    pub constraint: (types::Height, types::Width),
    pub offset: (types::Vertical, types::Horizontal),
    pub show_boundary_line: bool,
    pub art: String,
}

impl From<(String, types::Width, types::Latitude, types::Longtitude)> for DrawableBox {
    fn from((text, max_width, latitude, longtitude): (String, types::Width, types::Latitude, types::Longtitude)) -> Self {
        let art = artworks::reallign_text(text, max_width);
        let max_height = art.height();
        Self {
            coordinate: (latitude, longtitude),
            constraint: (max_height, max_width),
            offset: (0, 0),
            show_boundary_line: false,
            art,
        }
    }
}

trait TrimArt {
    fn trim(self, constraint: (types::Height, types::Width), offset: (types::Vertical, types::Horizontal)) -> Vec<String>;
}

impl TrimArt for Vec<String> {
    fn trim(self, constraint: (types::Height, types::Width), offset: (types::Vertical, types::Horizontal)) -> Vec<String> {
        let mut result = Vec::new();
        let v_off = offset.0.max(0);
        let h_off = offset.1.max(0);
        let v_con = constraint.0.max(0);
        let h_con = constraint.1.max(0);

        let lines = self.into_iter().skip(v_off).take(v_con);
        for line in lines {
            let trimmed: String = line.chars()
                .skip(h_off)
                .take(h_con)
                .collect();
            result.push(trimmed);
        }
        result
    }
}

static CURRENT_COLOR: RwLock<Color> = RwLock::new(Color::White(100));

pub fn clean_screen() {
    let mut stdout = io::stdout();
    let _ = crossterm::execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    );
}

pub fn set_pen_color(color: Color) {
    let mut stdout = io::stdout();
    let _ = match color {
        Color::White(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::White)),
        Color::Black(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Black)),
        Color::Red(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Red)),
        Color::Blue(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Blue)),
        Color::Yellow(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Yellow)),
        Color::Green(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Green)),
        Color::Cyan(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Cyan)),
        Color::Magenta(_) => crossterm::execute!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Magenta)),
    };
    let mut current_color = CURRENT_COLOR.write().unwrap();
    *current_color = color;
}

pub fn get_pen_color() -> Color {
    let current_color = CURRENT_COLOR.read().unwrap();
    *current_color
}

pub fn draw(entity: &DrawableBox) {
    let mut stdout = io::stdout();
    let (mut latitude, longtitude) = entity.coordinate;
    let lines = artworks::get_art_lines(&entity.art)
                            .trim(entity.constraint, entity.offset);
    set_pen_color(get_pen_color()); // !?? ik this seems stupid!
    if entity.show_boundary_line {
        let backend = ratatui::backend::CrosstermBackend::new(&stdout);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        let _ = terminal.draw(|frame| {
            let area = ratatui::layout::Rect::new(
                (longtitude - 1) as u16,
                (latitude - 1) as u16,
                (entity.constraint.1 + 2) as u16,
                (entity.constraint.0 + 2) as u16
            );
            let block = ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, area);
        });
    }
    for line in lines {
        set_pen_color(get_pen_color()); // !?? ik this seems stupid!
        let _ = crossterm::execute!(
            stdout,
            crossterm::cursor::MoveTo(longtitude as u16, latitude as u16),
            crossterm::style::Print(line),
        );
        latitude += 1;
    }
}

pub fn show_prompt_sign() {
    let mut stdout = io::stdout();
    let _ = crossterm::execute!(stdout, crossterm::cursor::Show);
}

pub fn hide_prompt_sign() {
    let mut stdout = io::stdout();
    let _ = crossterm::execute!(stdout, crossterm::cursor::Hide);
}