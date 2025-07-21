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
    board_entities: BoardManager,
    state: ScreenState,
}

impl ScreenManager {
    pub fn new() -> Self {
        let entities_factory = entities_factory::EntitiesFactory::new();
        let menu_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::Menu);
        let room_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::InRoom);
        let game_entities_vec = entities_factory.get_screen_entities(entities_factory::ScreenType::InGame);
        Self {
            menu_entities_vec,
            room_entities_vec,
            game_entities_vec,
            board_entities: BoardManager::new(),
            state: ScreenState::Menu,
        }
    }

    pub fn clean(&self) {
        caro_console::output::clean_screen();
    }

    pub fn set_state(&mut self, state: ScreenState) {
        self.state = state;
    }

    pub fn get_state(&self) -> ScreenState {
        self.state
    }

    pub fn enable_prompt_mode_at(&mut self, latitude: Latitude, longtitude: Longtitude) {
        caro_console::output::enable_prompt_mode_at(latitude as usize, longtitude as usize);
    }

    pub fn disable_prompt_mode(&mut self) {
        caro_console::output::disable_prompt_mode();
    }

    pub fn set_cursor_pos(&mut self, latitude: Latitude, longtitude: Longtitude) {
        self.board_entities.set_cursor_pos(latitude, longtitude);
    }

    pub fn get_cursor_pos(&self) -> (Latitude, Longtitude) {
        self.board_entities.get_cursor_pos()
    }

    pub fn update_game_context(&mut self, game_context: &caro_protocol::GameContext) {
        self.board_entities.update_board_context(game_context);
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
                self.board_entities.update();
            }
        }
    }

    pub fn update_board_only(&self) {
        match self.state {
            ScreenState::InGame => self.board_entities.update(),
            _ => ()
        }
    }

}

pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_WIDTH: usize = 20;
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
}

impl BoardManager {
    fn new() -> Self {
        let vertical_range = (0, BOARD_HEIGHT-1);
        let horizontal_range = (0, BOARD_WIDTH-1);

        let entities_factory = entities_factory::EntitiesFactory::new();

        let coordinate_layout = entities_factory.get_board_entity(entities_factory::BoardEntityType::CoordinateLayout
            (vertical_range, horizontal_range));
        let player_cursor = entities_factory.get_board_entity(entities_factory::BoardEntityType::YourCursor
            (vertical_range, horizontal_range, (0, 0)));
        let player1_moves = entities_factory.get_board_entity(entities_factory::BoardEntityType::XMoveSet
            (vertical_range, horizontal_range, Vec::new()));
        let player2_moves = entities_factory.get_board_entity(entities_factory::BoardEntityType::OMoveSet
            (vertical_range, horizontal_range, Vec::new()));
        Self {
            vertical_range,
            horizontal_range,
            
            coordinate_layout,
            last_opp_move_cursor: None,
            player_cursor,
            player1_moves,
            player2_moves,
        }
    }

    fn set_cursor_pos(&mut self, latitude: i64, longtitude: i64) {
        // self.cursor_pos.0 = latitude;
        // self.cursor_pos.1 = longtitude;
        if latitude < self.vertical_range.0 as i64 {

        } else if latitude > self.vertical_range.1 as i64 {

        }
        if longtitude < self.horizontal_range.0 as i64 {

        } else if longtitude > self.horizontal_range.1 as i64 {

        }
        self.player_cursor.set_position(latitude, longtitude);
    }

    fn get_cursor_pos(&self) -> (Latitude, Longtitude) {
        // (self.cursor_pos.0, self.cursor_pos.1);
        self.player_cursor.get_position()
    }

    fn update_board_context(&mut self, game_context: &caro_protocol::GameContext) {
        let entities_factory = entities_factory::EntitiesFactory::new();

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

        self.player1_moves = entities_factory.get_board_entity(entities_factory::BoardEntityType::XMoveSet
            (self.vertical_range, self.horizontal_range, player1_moves));
        self.player2_moves = entities_factory.get_board_entity(entities_factory::BoardEntityType::OMoveSet
            (self.vertical_range, self.horizontal_range, player2_moves));
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