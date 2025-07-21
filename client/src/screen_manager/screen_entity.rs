pub type Latitude = i64;
pub type Longtitude = i64;

pub trait ScreenEntity : Send + 'static {
    fn display(&self);
    fn set_position(&mut self, latitude: Latitude, longtitude: Longtitude);
    fn get_position(&self) -> (Latitude, Longtitude);
}