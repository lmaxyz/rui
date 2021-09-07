use std::{cell::RefCell, rc::Rc};

use sdl2::rect::{Rect, Point};

use crate::RuiObject;


pub struct RuiCheckbox {
    rect: Rect,
    checker: [Point; 3],
    
    pub checked: bool   // State
}

impl RuiCheckbox {
    pub fn new(width: u32, x: i32 , y: i32) -> Rc<RefCell<RuiCheckbox>> {
        let point_1 = Point::new(x+(width as i32/4), y+(width as i32/2));
        let point_2 = Point::new(x+(width as i32/2), y+((width as i32/4)*3));
        let point_3 = Point::new(x+((width as i32/4)*3), y+(width as i32/4));

        Rc::new(RefCell::new(RuiCheckbox {
            rect: Rect::new(x, y, width, width),
            checker: [point_1, point_2, point_3],
            checked: false,
        }))
    }

    fn draw_checker(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.draw_lines(&self.checker[..]).unwrap();
    }

    pub fn set_active(&mut self, active: bool) {
        self.checked = active
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked
    }
}

impl RuiObject for RuiCheckbox {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.draw_rect(self.rect).unwrap();

        if self.checked {
            self.draw_checker(canvas)
        }
    }

    fn include_coords(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point(Point::new(x, y))
    }

    fn on_click(&mut self) {
        self.checked = !self.checked
    }
}
