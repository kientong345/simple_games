use super::screen_entity;

pub enum ScreenType {
    Menu,
    InRoom,
    InGame,
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
                
            },
            ScreenType::InRoom => {

            },
            ScreenType::InGame => {

            }
        }
        todo!()
    }

    pub fn get_cursor(&self) -> Box<dyn screen_entity::ScreenEntity> {
        todo!()
    }
}