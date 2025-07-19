use crate::screen_manager::screen_entity;

pub struct WaitingRoomBox {
    latitude: screen_entity::Latitude,
    longtitude: screen_entity::Longtitude,
}

impl WaitingRoomBox {
    pub fn new() -> Self {
        Self {
            latitude: -1,
            longtitude: -1,
        }
    }
}

impl screen_entity::ScreenEntity for WaitingRoomBox {
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

pub struct RoomInfoBox {
    latitude: screen_entity::Latitude,
    longtitude: screen_entity::Longtitude,
}

impl RoomInfoBox {

}

impl screen_entity::ScreenEntity for RoomInfoBox {
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

pub struct InstructionBox {
    latitude: screen_entity::Latitude,
    longtitude: screen_entity::Longtitude,
}

impl InstructionBox {
    pub fn new() -> Self {
        Self {
            latitude: -1,
            longtitude: -1,
        }
    }
}

impl screen_entity::ScreenEntity for InstructionBox {
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