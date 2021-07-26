use sdl2::{rect::Rect, render::Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::Drawaible;


pub struct Button {
    _text: String,
    border: Rect,
    inner: Rect,
    _is_clicked: bool
}

impl Button {
    pub fn new(text: &str, width: u32, height: u32, x: i32, y: i32) -> Button {
        Button {
            _text: text.to_string(),
            border: Rect::new(x, y, width, height),
            inner: Rect::new(x+1, y+1, width-2, height-2),
            _is_clicked: false
        }
    }
}

impl Drawaible for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(self.border).unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 240));
        canvas.fill_rect(self.inner).unwrap();  
    }
}
