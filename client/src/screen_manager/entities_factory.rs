use crate::screen_manager::{game_entities, menu_entities, room_entities};

use super::screen_entity;

#[derive(Debug, Clone, Copy)]
pub enum ScreenType {
    Menu,
    InRoom,
    InGame,
}

#[derive(Debug, Clone)]
pub enum BoardEntityType {
    CoordinateLayout((usize, usize), (usize, usize)),
    OppCursor((usize, usize), (usize, usize), (usize, usize)),
    YourCursor((usize, usize), (usize, usize), (usize, usize)),
    XMoveSet((usize, usize), (usize, usize), Vec<(usize, usize)>),
    OMoveSet((usize, usize), (usize, usize), Vec<(usize, usize)>),
}

pub struct EntitiesFactory {

}

impl EntitiesFactory {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn get_screen_entities(&self, screen_type: ScreenType) -> Vec<Box<dyn screen_entity::ScreenEntity>> {
        match screen_type {
            ScreenType::Menu => {
                let menu_instruction_box = menu_entities::InstructionBox::new();
                let menu_prompt_box = menu_entities::PromptBox::new();
                vec![
                    Box::new(menu_instruction_box),
                    Box::new(menu_prompt_box)
                ]
            },
            ScreenType::InRoom => {
                let room_instruction_box = room_entities::InstructionBox::new();
                let room_prompt_box = room_entities::PromptBox::new();
                vec![
                    Box::new(room_instruction_box),
                    Box::new(room_prompt_box)
                ]
            },
            ScreenType::InGame => {
                let game_instruction_box = game_entities::InstructionBox::new();
                let game_prompt_box = game_entities::PromptBox::new();
                vec![
                    Box::new(game_instruction_box),
                    Box::new(game_prompt_box)
                ]
            }
        }
    }

    pub fn get_board_entity(&self, entity_type: BoardEntityType) -> Box<dyn screen_entity::ScreenEntity> {
        match entity_type {
            BoardEntityType::CoordinateLayout(vertical_range, horizontal_range) => {
                Box::new(game_entities::CoordinateLayout::new(vertical_range, horizontal_range))
            },
            BoardEntityType::OppCursor(vertical_range, horizontal_range, coordinate) => {
                Box::new(game_entities::OppCursor::new(vertical_range, horizontal_range, coordinate))
            },
            BoardEntityType::YourCursor(vertical_range, horizontal_range, coordinate) => {
                Box::new(game_entities::YourCursor::new(vertical_range, horizontal_range, coordinate))
            },
            BoardEntityType::XMoveSet(vertical_range, horizontal_range, move_set ) => {
                Box::new(game_entities::XMoveSet::new(vertical_range, horizontal_range, move_set))
            },
            BoardEntityType::OMoveSet(vertical_range, horizontal_range, move_set ) => {
                Box::new(game_entities::OMoveSet::new(vertical_range, horizontal_range, move_set))
            },
        }
    }
}