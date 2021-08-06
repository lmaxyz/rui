

pub trait Clickable {

    fn on_click(&mut self);
    
    fn include_coords(&self, x: i32, y: i32) -> bool;

}