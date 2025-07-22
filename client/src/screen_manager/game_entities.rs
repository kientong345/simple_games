use caro_console::artworks::ArtDimension;

use crate::screen_manager::screen_entity;

const GAME_INSTRUCTION_BOX_POS: (usize, usize) = (5, 15);
pub struct InstructionBox {
    entity: caro_console::output::DrawableBox,
}

impl InstructionBox {
    pub fn new() -> Self {
        let art = caro_console::artworks::MENU_INSTRUCTION.to_string();
        Self {
            entity: caro_console::output::DrawableBox {
                coordinate: GAME_INSTRUCTION_BOX_POS,
                constraint: (art.height(), art.width()),
                offset: (0, 0),
                show_boundary_line: true,
                art,
            }
        }
    }
}

impl screen_entity::ScreenEntity for InstructionBox {
    fn display(&self) {
        caro_console::output::set_pen_color(caro_console::output::Color::Cyan(100));
        caro_console::output::draw(&self.entity);
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (self.entity.coordinate.0 as i64, self.entity.coordinate.1 as i64)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        self.entity.coordinate.0 = latitude as usize;
        self.entity.constraint.1 = longtitude as usize;
    }
}

const GAME_PROMPT_BOX_POS: (usize, usize) = (15, 60);
pub struct PromptBox {
    entity: caro_console::output::DrawableBox,
}

impl PromptBox {
    pub fn new() -> Self {
        let art = caro_console::artworks::PROMPT_BOX.to_string();
        Self {
            entity: caro_console::output::DrawableBox {
                coordinate: GAME_PROMPT_BOX_POS,
                constraint: (art.height(), art.width()),
                offset: (0, 0),
                show_boundary_line: true,
                art,
            }
        }
    }
}

impl screen_entity::ScreenEntity for PromptBox {
    fn display(&self) {
        caro_console::output::set_pen_color(caro_console::output::Color::Yellow(100));
        caro_console::output::draw(&self.entity);
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (self.entity.coordinate.0 as i64, self.entity.coordinate.1 as i64)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        self.entity.coordinate.0 = latitude as usize;
        self.entity.constraint.1 = longtitude as usize;
    }
}


// ----------------------------------------------------------------------

const GAME_BOARD_BASE_POS: (usize, usize) = (2, 4);
pub struct CoordinateLayout {
    entities: Vec<caro_console::output::DrawableBox>,
}

impl CoordinateLayout {
    pub fn new(vertical_range: (usize, usize), horizontal_range: (usize, usize)) -> Self {
        let board_pos = caro_console::caro_art_tools::BoardPosition {
            base: GAME_BOARD_BASE_POS,
            vertical_range,
            horizontal_range,
        };
        Self {
            entities: caro_console::caro_art_tools::get_drawable_coordinate_layout(&board_pos)
        }
    }
}

impl screen_entity::ScreenEntity for CoordinateLayout {
    fn display(&self) {
        caro_console::output::set_pen_color(caro_console::output::Color::Blue(100));
        for entity in &self.entities {
            caro_console::output::draw(entity);
        }
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (0, 0)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        // locked
    }
}

pub struct Cursor {
    entity: caro_console::output::DrawableBox,
    coordinate: (usize, usize),
    are_you: bool,
}

impl Cursor {
    pub fn new(vertical_range: (usize, usize), horizontal_range: (usize, usize), coordinate: (usize, usize), are_you: bool) -> Self {
        let board_pos = caro_console::caro_art_tools::BoardPosition {
            base: GAME_BOARD_BASE_POS,
            vertical_range,
            horizontal_range,
        };
        Self {
            entity: caro_console::caro_art_tools::get_drawable_cursor(&board_pos, coordinate),
            coordinate,
            are_you,
        }
    }
}

impl screen_entity::ScreenEntity for Cursor {
    fn display(&self) {
        if self.are_you {
            caro_console::output::set_pen_color(caro_console::output::Color::Green(100));
        } else {
            caro_console::output::set_pen_color(caro_console::output::Color::Red(100));
        }
        caro_console::output::draw(&self.entity);
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        let (latitude, longtitude) = self.coordinate;
        (latitude as i64, longtitude as i64)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        self.coordinate.0 = latitude as usize;
        self.coordinate.1 = longtitude as usize;
    }
}

pub struct XMoveSet {
    entities: Vec<caro_console::output::DrawableBox>,
    are_you: bool,
}

impl XMoveSet {
    pub fn new(vertical_range: (usize, usize), horizontal_range: (usize, usize), move_set: Vec<(usize, usize)>, are_you: bool) -> Self {
        let board_pos = caro_console::caro_art_tools::BoardPosition {
            base: GAME_BOARD_BASE_POS,
            vertical_range,
            horizontal_range,
        };
        Self {
            entities: caro_console::caro_art_tools::get_drawable_x_moves(&board_pos, move_set),
            are_you,
        }
    }
}

impl screen_entity::ScreenEntity for XMoveSet {
    fn display(&self) {
        if self.are_you {
            caro_console::output::set_pen_color(caro_console::output::Color::Green(100));
        } else {
            caro_console::output::set_pen_color(caro_console::output::Color::Red(100));
        }
        for entity in &self.entities {
            caro_console::output::draw(entity);
        }
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (0, 0)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        // locked
    }
}

pub struct OMoveSet {
    entities: Vec<caro_console::output::DrawableBox>,
    are_you: bool,
}

impl OMoveSet {
    pub fn new(vertical_range: (usize, usize), horizontal_range: (usize, usize), move_set: Vec<(usize, usize)>, are_you: bool) -> Self {
        let board_pos = caro_console::caro_art_tools::BoardPosition {
            base: GAME_BOARD_BASE_POS,
            vertical_range,
            horizontal_range,
        };
        Self {
            entities: caro_console::caro_art_tools::get_drawable_o_moves(&board_pos, move_set),
            are_you,
        }
    }
}

impl screen_entity::ScreenEntity for OMoveSet {
    fn display(&self) {
        if self.are_you {
            caro_console::output::set_pen_color(caro_console::output::Color::Green(100));
        } else {
            caro_console::output::set_pen_color(caro_console::output::Color::Red(100));
        }
        for entity in &self.entities {
            caro_console::output::draw(entity);
        }
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (0, 0)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        // locked
    }
}

const GAME_LOG_BOX_POS: (usize, usize) = (34, 61);
const GAME_LOG_BOX_WIDTH: usize = 20;
pub struct LogBox {
    entity: caro_console::output::DrawableBox,
}

impl LogBox {
    pub fn new(content: String) -> Self {
        Self {
            entity: caro_console::output::DrawableBox::from((content, GAME_LOG_BOX_WIDTH, GAME_LOG_BOX_POS.0, GAME_LOG_BOX_POS.1)),
        }
    }
}

impl screen_entity::ScreenEntity for LogBox {
    fn display(&self) {
        caro_console::output::set_pen_color(caro_console::output::Color::White(100));
        caro_console::output::draw(&self.entity);
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (self.entity.coordinate.0 as i64, self.entity.coordinate.1 as i64)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        self.entity.coordinate.0 = latitude as usize;
        self.entity.constraint.1 = longtitude as usize;
    }
}