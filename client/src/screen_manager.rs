use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{caro_protocol, global_state};

pub mod screen_entity;
pub mod entities_factory;
pub mod menu_entities;
pub mod room_entities;
pub mod game_entities;

const SCREEN_WIDTH: usize = 10;
const SCREEN_HEIGHT: usize = 10;

pub struct ScreenManager {
    global_state: Arc<RwLock<global_state::GolbalState>>,

    menu_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    room_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    game_entities_vec: Vec<Box<dyn screen_entity::ScreenEntity>>,
    board_entities: BoardManager,
    log_entity: Box<dyn screen_entity::ScreenEntity>,
}

impl ScreenManager {
    pub fn new(global_state: Arc<RwLock<global_state::GolbalState>>) -> Self {
        let menu_entities_vec = entities_factory::EntitiesFactory::get_screen_entities(entities_factory::ScreenType::Menu);
        let room_entities_vec = entities_factory::EntitiesFactory::get_screen_entities(entities_factory::ScreenType::InRoom);
        let game_entities_vec = entities_factory::EntitiesFactory::get_screen_entities(entities_factory::ScreenType::InGame);
        let log_entity = entities_factory::EntitiesFactory::get_log_entity("".to_string(), entities_factory::ScreenType::Menu);
        Self {
            global_state,
            menu_entities_vec,
            room_entities_vec,
            game_entities_vec,
            board_entities: BoardManager::new(),
            log_entity,
        }
    }

    pub fn clean(&self) {
        caro_console::output::clean_screen();
    }

    pub fn set_player_order(&mut self, player_order: caro_protocol::PlayerOrder) {
        self.board_entities.set_player_order(player_order);
    }

    pub async fn enable_prompt_mode(&self) {
        let player_state = self.global_state.read().await.get_player_state();
        match player_state {
            caro_protocol::PlayerState::Logged(_) => {
                caro_console::input::enable_prompt_mode_at(17, 63);
            },
            caro_protocol::PlayerState::Waiting(_) =>  {
                caro_console::input::enable_prompt_mode_at(17, 63);
            },
            caro_protocol::PlayerState::InGame(_) => {
                caro_console::input::enable_prompt_mode_at(35, 63);
            }
        }
    }

    pub fn disable_prompt_mode(&self) {
        caro_console::input::disable_prompt_mode();
    }

    pub fn set_cursor_pos(&mut self, latitude: caro_protocol::Latitude, longtitude: caro_protocol::Longtitude) {
        self.board_entities.set_cursor_pos(latitude, longtitude);
    }

    pub fn get_cursor_pos(&self) -> caro_protocol::Coordinate {
        self.board_entities.get_cursor_pos()
    }

    pub fn update_game_context(&mut self, game_context: &caro_protocol::GameContext) {
        let player1_moves = game_context.player1_move_history
                                            .iter()
                                            .filter_map(|(x, y)| {
                                                Some((*x as usize, *y as usize))
                                            }).collect();

        let player2_moves = game_context.player2_move_history
                                            .iter()
                                            .filter_map(|(x, y)| {
                                                Some((*x as usize, *y as usize))
                                            }).collect();
        self.board_entities.update_move_set(player1_moves, player2_moves);
    }

    pub async fn update(&self) {
        let player_state = self.global_state.read().await.get_player_state();
        match player_state {
            caro_protocol::PlayerState::Logged(_) => {
                for entity in self.menu_entities_vec.iter() {
                    entity.display();
                }
            },
            caro_protocol::PlayerState::Waiting(_) =>  {
                for entity in self.room_entities_vec.iter() {
                    entity.display();
                }
            },
            caro_protocol::PlayerState::InGame(_) => {
                for entity in self.game_entities_vec.iter() {
                    entity.display();
                }
                self.board_entities.update();
            }
        }
        self.log_entity.display();

        // relocate the command prompt
        if caro_console::input::is_prompt_mode() {
            self.enable_prompt_mode().await;
        }
    }

    pub async fn update_board_only(&self) {
        let player_state = self.global_state.read().await.get_player_state();
        match player_state {
            caro_protocol::PlayerState::InGame(_) => self.board_entities.update(),
            _ => ()
        }

        // relocate the command prompt
        if caro_console::input::is_prompt_mode() {
            self.enable_prompt_mode().await;
        }
    }

    pub async fn log(&mut self, content: String) {
        let player_state = self.global_state.read().await.get_player_state();
        match player_state {
            caro_protocol::PlayerState::Logged(_) => {
                self.log_entity = entities_factory::EntitiesFactory::get_log_entity(content, entities_factory::ScreenType::Menu);
            },
            caro_protocol::PlayerState::Waiting(_) =>  {
                self.log_entity = entities_factory::EntitiesFactory::get_log_entity(content, entities_factory::ScreenType::InRoom);
            },
            caro_protocol::PlayerState::InGame(_) => {
                self.log_entity = entities_factory::EntitiesFactory::get_log_entity(content, entities_factory::ScreenType::InGame);
            }
        }
        self.log_entity.display();

        // relocate the command prompt
        if caro_console::input::is_prompt_mode() {
            self.enable_prompt_mode().await;
        }
    }

}

pub const BOARD_HEIGHT: usize = 15;
pub const BOARD_WIDTH: usize = 25;
pub const LATITUDE_LIMIT: usize = 1024;
pub const LONGTITUDE_LIMIT: usize = 1024;
struct BoardManager {
    vertical_range: (usize, usize),
    horizontal_range: (usize, usize),
    // cursor_pos: (Latitude, Longtitude),
    // last_opp_move: (Latitude, Longtitude),

    coordinate_layout: Box<dyn screen_entity::ScreenEntity>,
    last_opp_move_cursor: Option<Box<dyn screen_entity::ScreenEntity>>,
    player_cursor: Box<dyn screen_entity::ScreenEntity>,
    player1_moves: Box<dyn screen_entity::ScreenEntity>,
    player2_moves: Box<dyn screen_entity::ScreenEntity>,

    player_order: caro_protocol::PlayerOrder,
}

impl BoardManager {
    fn new() -> Self {
        let vertical_range = (0, BOARD_HEIGHT-1);
        let horizontal_range = (0, BOARD_WIDTH-1);

        let coordinate_layout = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::CoordinateLayout
            (vertical_range, horizontal_range));
        let player_cursor = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::Cursor
            (vertical_range, horizontal_range, (0, 0), true));
        let player1_moves = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::XMoveSet
            (vertical_range, horizontal_range, Vec::new(), false));
        let player2_moves = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::OMoveSet
            (vertical_range, horizontal_range, Vec::new(), false));
        Self {
            vertical_range,
            horizontal_range,
            
            coordinate_layout,
            last_opp_move_cursor: None,
            player_cursor,
            player1_moves,
            player2_moves,

            player_order: caro_protocol::PlayerOrder::Player1,
        }
    }

    fn set_player_order(&mut self, player_order: caro_protocol::PlayerOrder) {
        self.player_order = player_order;
    }

    fn set_cursor_pos(&mut self, latitude: i64, longtitude: i64) {
        let clamped_latitude = latitude.clamp(0, LATITUDE_LIMIT as i64);
        let clamped_longtitude = longtitude.clamp(0, LONGTITUDE_LIMIT as i64);

        let (mut new_vertical_start, mut new_vertical_end) = self.vertical_range;
        if clamped_latitude < new_vertical_start as i64 {
            new_vertical_start = clamped_latitude as usize;
            new_vertical_end = (clamped_latitude as usize + BOARD_HEIGHT - 1).min(LATITUDE_LIMIT);
        } else if clamped_latitude > new_vertical_end as i64 {
            new_vertical_end = clamped_latitude as usize;
            new_vertical_start = (clamped_latitude as usize - BOARD_HEIGHT + 1).max(0);
        }
        if self.vertical_range.0 != new_vertical_start || self.vertical_range.1 != new_vertical_end {
            self.vertical_range = (new_vertical_start, new_vertical_end);
        }

        let (mut new_horizontal_start, mut new_horizontal_end) = self.horizontal_range;
        if clamped_longtitude < new_horizontal_start as i64 {
            new_horizontal_start = clamped_longtitude as usize;
            new_horizontal_end = (clamped_longtitude as usize + BOARD_WIDTH - 1).min(LONGTITUDE_LIMIT);
        } else if clamped_longtitude > new_horizontal_end as i64 {
            new_horizontal_end = clamped_longtitude as usize;
            new_horizontal_start = (clamped_longtitude as usize - BOARD_WIDTH + 1).max(0);
        }
        if self.horizontal_range.0 != new_horizontal_start || self.horizontal_range.1 != new_horizontal_end {
            self.horizontal_range = (new_horizontal_start, new_horizontal_end);
        }

        self.player_cursor.set_position(clamped_latitude, clamped_longtitude);
    }

    fn get_cursor_pos(&self) -> caro_protocol::Coordinate {
        // (self.cursor_pos.0, self.cursor_pos.1);
        self.player_cursor.get_position()
    }

    fn update_move_set(&mut self, player1_moves: Vec<(usize, usize)>, player2_moves: Vec<(usize, usize)>) {
        let is_player1 = match self.player_order {
            caro_protocol::PlayerOrder::Player1 => true,
            caro_protocol::PlayerOrder::Player2 => false,
        };
        let last_opp_move = if is_player1 {
            &player2_moves.last()
        } else {
            &player1_moves.last()
        };
        if let Some(opp_move) = last_opp_move {
            self.last_opp_move_cursor = Some(entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::Cursor
                (self.vertical_range, self.horizontal_range, (opp_move.0, opp_move.1), false)));
        }
        self.player1_moves = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::XMoveSet
            (self.vertical_range, self.horizontal_range, player1_moves, is_player1));
        self.player2_moves = entities_factory::EntitiesFactory::get_board_entity(entities_factory::BoardEntityType::OMoveSet
            (self.vertical_range, self.horizontal_range, player2_moves, !is_player1));
    }

    fn update(&self) {
        // layer 1
        self.coordinate_layout.display();
        // layer 2
        if let Some(entity) = &self.last_opp_move_cursor {
            entity.display();
        }
        // layer 3
        self.player_cursor.display();
        // layer 4
        self.player1_moves.display();
        self.player2_moves.display();
    }
}
