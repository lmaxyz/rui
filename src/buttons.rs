use sdl2::{rect::{Point, Rect}, render::Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::{Child, Clickable, Drawaible};


pub struct Button {
    border: Rect,
    inner: Rect,

    _is_clicked: bool,
    is_enabled: bool,

    pub text: String,
    pub text_wrapper: Rect,

    on_click_fn: Option<Box<dyn FnMut() -> ()>>
}

impl Button {
    pub fn new(text: &str, width: u32, height: u32, x: i32, y: i32, on_click_fn: Option<Box<dyn FnMut() -> ()>>) -> Button {
        Button {
            border: Rect::new(x, y, width, height),
            inner: Rect::new(x+2, y+2, width-4, height-4),
            _is_clicked: false,
            is_enabled: true,
            
            text: text.to_string(),
            text_wrapper: Rect::new(x+20, y+5, width-40, height-10),
            
            on_click_fn,
        }
    }

    pub fn click(&mut self) {
        self.on_click()
    }

    pub fn change_text(&mut self, new_text: &str) {
        self.text = String::from(new_text)
    }

}

impl Drawaible for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // ToDo: Переделать отрисовку текста, текстура не создавалась заново каждый раз
        let ttf_context = sdl2::ttf::init().unwrap();
    
        let mut font = ttf_context.load_font("fonts\\Default.ttf", 64).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        
        let surface = font
            .render(self.text.as_str())
            .blended(Color::RGBA(130, 80, 130, 255))
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();

        canvas.set_draw_color(Color::RGB(211, 167, 177));
        canvas.draw_rect(self.border).unwrap();
        canvas.set_draw_color(Color::RGB(200, 162, 200));
        canvas.fill_rect(self.inner).unwrap(); 

        canvas.copy(&texture, None, Some(self.text_wrapper)).unwrap();
    }
}

impl Clickable for Button {
    fn on_click(&mut self) {
        if self.is_enabled {
            match &mut self.on_click_fn {
                Some(f) => f(),
                None => {}
            }
        }
    }

    fn include_coords(&self, x: i32, y: i32) -> bool {
        self.border.contains_point(Point::new(x, y))
    }
}

impl Child for Button {}
