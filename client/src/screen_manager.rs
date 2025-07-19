use crate::caro_protocol;
use std::cmp::min;

pub mod screen_entity;
pub mod entities_factory;
pub mod menu_entities;
pub mod room_entities;
pub mod game_entities;

const SCREEN_WIDTH: usize = 10;
const SCREEN_HEIGHT: usize = 10;

pub type Latitude = i64;
pub type Longtitude = i64;

#[derive(Debug, Clone, Copy)]
pub enum ScreenState {
    Menu,
    InRoom,
    InGame,
}

pub struct ScreenManager {
    menu_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    room_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    game_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    cursor_entity: Option<Box<dyn screen_entity::ScreenEntity>>,
    state: ScreenState,
}

impl ScreenManager {
    pub fn new() -> Self {
        let entities_factory = entities_factory::EntitiesFactory::new();
        let menu_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::Menu);
        let room_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::InRoom);
        let game_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::InGame);
        let cursor_entity = None;
        Self {
            menu_entities_vec,
            room_entities_vec,
            game_entities_vec,
            cursor_entity,
            state: ScreenState::Menu,
        }
    }

    pub fn set_state(&mut self, state: ScreenState) {
        self.state = state;
    }

    pub fn get_state(&self) -> ScreenState {
        self.state
    }

    pub fn show_cursor(&mut self) {
        let entities_factory = entities_factory::EntitiesFactory::new();
        if self.cursor_entity.is_none() {
            self.cursor_entity = Some(entities_factory.get_cursor());
        }
    }

    pub fn hide_cursor(&mut self) {
        self.cursor_entity = None;
    }

    pub fn set_cursor_pos(&mut self, latitude: Latitude, longtitude: Longtitude) {
        if let Some(cursor) = self.cursor_entity.as_mut() {
            cursor.set_position(latitude, longtitude);
        }
    }

    pub fn get_cursor_pos(&self) -> Option<(Latitude, Longtitude)> {
        if let Some(cursor) = self.cursor_entity.as_ref() {
            Some(cursor.get_position())
        } else {
            None
        }
    }

    pub fn set_game_context() {

    }

    pub fn update(&self) {
        match self.state {
            ScreenState::Menu => {
                for entity in self.menu_entities_vec.iter() {
                    entity.display();
                }
            },
            ScreenState::InRoom =>  {
                for entity in self.room_entities_vec.iter() {
                    entity.display();
                }
            },
            ScreenState::InGame => {
                for entity in self.game_entities_vec.iter() {
                    entity.display();
                }
            }
        }
        if let Some(cursor) = self.cursor_entity.as_ref() {
            cursor.display();
        }
    }

}

pub fn print_caro_board(board: Vec<caro_protocol::Row>) {
    if board.len() == 0 || board[0].len() == 0 {
        return;
    }
    let max_height = min(board.len(), SCREEN_HEIGHT);
    let max_width = min(board[0].len(), SCREEN_WIDTH);
    for row in &board[..max_height] {
        print!("[");
        for tile in &row[..max_width] {
            match tile {
                caro_protocol::TileState::Empty => print!(" ."),
                caro_protocol::TileState::Player1 => print!(" X"),
                caro_protocol::TileState::Player2 => print!(" O"),
            }
        }
        println!("]");
    }
}

pub fn print_caro_context(context: caro_protocol::GameContext) {
    if context.board_height <= 0 || context.board_width <= 0 {
        return;
    }

    let player1_move_history = context.player1_move_history;
    let player1_occupied = move |latitude: i64, longtitude: i64| -> bool {
        let target = player1_move_history.iter().find(|(llatitude, llongtitude)| {
            latitude == *llatitude && longtitude == *llongtitude
        });
        if let Some(_coor) = target {
            true
        } else {
            false
        }
    };
    let player2_move_history = context.player2_move_history;
    let player2_occupied = move |latitude: i64, longtitude: i64| -> bool {
        let target = player2_move_history.iter().find(|(llatitude, llongtitude)| {
            latitude == *llatitude && longtitude == *llongtitude
        });
        if let Some(_coor) = target {
            true
        } else {
            false
        }
    };

    let max_height = min(context.board_height, SCREEN_HEIGHT);
    let max_width = min(context.board_width, SCREEN_WIDTH);
    println!("======================");
    for latitude in 0..max_height {
        print!("[");
        for longtitude in 0..max_width {
            if player1_occupied(latitude as i64, longtitude as i64) {
                print!("X ");
            } else if player2_occupied(latitude as i64, longtitude as i64) {
                print!("O ");
            } else {
                print!(". ");
            }
        }
        println!("]");
    }
    println!("======================");
}

pub fn print_notification(message: &str) {
    println!("{}", message);
}