use std::{io, sync::{atomic::{AtomicBool, Ordering}, Mutex}};

use crossterm;
use ratatui;

use crate::artworks::ArtDimension;

pub mod artworks;

pub type Height = usize;
pub type Width = usize;
pub type Vertical = usize;
pub type Horizontal = usize;

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

pub struct DrawableBox {
    pub constraint: (Height, Width),
    pub offset: (Vertical, Horizontal),
    pub show_boundary_line: bool,
    pub art: String,
}

impl From<(String, Width)> for DrawableBox {
    fn from((text, max_width): (String, Width)) -> Self {
        let art = artworks::reallign_text(text, max_width);
        let max_height = art.height();
        Self {
            constraint: (max_height, max_width),
            offset: (0, 0),
            show_boundary_line: false,
            art,
        }
    }
}

trait TrimArt {
    fn trim(self, constraint: (Height, Width), offset: (Vertical, Horizontal)) -> Vec<String>;
}

impl TrimArt for Vec<String> {
    fn trim(self, constraint: (Height, Width), offset: (Vertical, Horizontal)) -> Vec<String> {
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

static IS_PROMPT_MODE: AtomicBool = AtomicBool::new(true);
static CURRENT_COLOR: Mutex<Color> = Mutex::new(Color::White(100));

pub fn init() {
    
}

pub fn deinit() {

}

pub fn enable_prompt_mode_at(latitude: i64, longtutude: i64) {
    let mut stdout = io::stdout();
    let latitude = latitude.max(0) as u16;
    let longtitude = longtutude.max(0) as u16;
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
    let mut current_color = CURRENT_COLOR.lock().unwrap();
    *current_color = color;
}

pub fn get_pen_color() -> Color {
    let current_color = CURRENT_COLOR.lock().unwrap();
    *current_color
}

pub fn draw_at(entity: DrawableBox, latitude: i64, longtutude: i64) {
    let mut stdout = io::stdout();
    let mut latitude = latitude.max(0) as u16;
    let longtitude = longtutude.max(0) as u16;
    let lines = artworks::get_art_lines(entity.art)
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
            crossterm::cursor::MoveTo(longtitude, latitude),
            crossterm::style::Print(line),
        );
        latitude += 1;
    }
}

// pub fn get_empty_drawable_caro_board(height: Height, width: Width) -> DrawableBox {
//     // Simple ASCII grid: '+' corners, '-' top/bottom, '|' sides, ' ' inside
//     let mut art = String::new();
//     for h in 0..height {
//         for w in 0..width {
//             let ch = if (h == 0 || h == height - 1) && (w == 0 || w == width - 1) {
//                 '+'
//             } else if h == 0 || h == height - 1 {
//                 '-'
//             } else if w == 0 || w == width - 1 {
//                 '|'
//             } else {
//                 ' '
//             };
//             art.push(ch);
//         }
//         art.push('\n');
//     }
//     // Leak the string to get a &'static str (for demo, not ideal for production)
//     let static_art: &'static str = Box::leak(art.into_boxed_str());
//     DrawableBox {
//         dimension: (height, width),
//         offset: (1, 1),
//         art: static_art,
//     }
// }
