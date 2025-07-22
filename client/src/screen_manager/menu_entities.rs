use caro_console::artworks::ArtDimension;

use crate::screen_manager::screen_entity;

pub struct ServerInfoBox {
    ipaddress: String,
    port: String,
    latitude: screen_entity::Latitude,
    longtitude: screen_entity::Longtitude,
}

impl ServerInfoBox {
    pub fn new(ipaddress: String, port: String) -> Self {
        Self {
            ipaddress,
            port,
            latitude: -1,
            longtitude: -1,
        }
    }
}

impl screen_entity::ScreenEntity for ServerInfoBox {
    fn display(&self) {
        
    }

    fn get_position(&self) -> (screen_entity::Latitude, screen_entity::Longtitude) {
        (self.latitude, self.longtitude)
    }

    fn set_position(&mut self, latitude: screen_entity::Latitude, longtitude: screen_entity::Longtitude) {
        self.latitude = latitude;
        self.longtitude = longtitude;
    }
}

const MENU_INSTRUCTION_BOX_POS: (usize, usize) = (5, 15);
pub struct InstructionBox {
    entity: caro_console::output::DrawableBox,
}

impl InstructionBox {
    pub fn new() -> Self {
        let art = caro_console::artworks::MENU_INSTRUCTION.to_string();
        Self {
            entity: caro_console::output::DrawableBox {
                coordinate: MENU_INSTRUCTION_BOX_POS,
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

const MENU_PROMPT_BOX_POS: (usize, usize) = (15, 60);
pub struct PromptBox {
    entity: caro_console::output::DrawableBox,
}

impl PromptBox {
    pub fn new() -> Self {
        let art = caro_console::artworks::PROMPT_BOX.to_string();
        Self {
            entity: caro_console::output::DrawableBox {
                coordinate: MENU_PROMPT_BOX_POS,
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
        caro_console::output::set_pen_color(caro_console::output::Color::Green(100));
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

const MENU_LOG_BOX_POS: (usize, usize) = (19, 61);
const MENU_LOG_BOX_WIDTH: usize = 20;
pub struct LogBox {
    entity: caro_console::output::DrawableBox,
}

impl LogBox {
    pub fn new(content: String) -> Self {
        Self {
            entity: caro_console::output::DrawableBox::from((content, MENU_LOG_BOX_WIDTH, MENU_LOG_BOX_POS.0, MENU_LOG_BOX_POS.1)),
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