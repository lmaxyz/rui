use std::thread;

use sdl2::{rect::{Point, Rect}, render::Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::{Clickable, Drawaible};


pub struct Button<F: FnMut() -> ()> {
    _text: String,
    border: Rect,
    inner: Rect,
    _is_clicked: bool,
    is_enabled: bool,
    on_click_fn: F
}

impl<F> Button<F>
where F: FnMut() -> () {
    pub fn new(text: &str, width: u32, height: u32, x: i32, y: i32, on_click_fn: F) -> Button<F> {

        Button {
            _text: text.to_string(),
            border: Rect::new(x, y, width, height),
            inner: Rect::new(x+2, y+2, width-4, height-4),
            
            _is_clicked: false,
            is_enabled: true,

            on_click_fn
        }
    }

    pub fn include_coords(&self, x: i32, y: i32) -> bool {
        self.border.contains_point(Point::new(x, y))
    }

}

impl<F> Drawaible for Button<F>
where F: FnMut() -> () {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(211, 167, 177));
        canvas.draw_rect(self.border).unwrap();
        canvas.set_draw_color(Color::RGB(200, 162, 200));
        canvas.fill_rect(self.inner).unwrap();  
    }
}

impl<F> Clickable for Button<F>
where F: FnMut() -> () {
    fn on_click(&mut self) {
        if self.is_enabled {
            (self.on_click_fn)()
        }
    }
}
