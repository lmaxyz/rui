use sdl2::render::Texture;
use sdl2::sys::SDL_Texture;
use sdl2::{rect::{Point, Rect}, render::Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::RuiObject;

const FONT_SIZE: u16 = 24;


pub struct ButtonBuilder {
    border: Rect,
    inner: Rect,

    _is_clicked: bool,
    is_enabled: bool,

    pub text: String,
    pub text_wrapper: Rect,
    pub text_texture: *mut SDL_Texture,

    on_click_fn: Option<Box<dyn FnMut() -> ()>>
}

impl ButtonBuilder {
    fn new(text: &str, width: u32, height: u32, x: i32, y: i32, canvas: &Canvas<Window>) -> ButtonBuilder {
        let ttf_context = sdl2::ttf::init().unwrap();
    
        let mut font = ttf_context.load_font("fonts\\Default.ttf", FONT_SIZE * 2).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        
        
        let surface = font
            .render(text)
            .blended(Color::RGBA(130, 80, 130, 255))
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        let text_width = (text.chars().count() * FONT_SIZE as usize / 2) as u32;
        let text_height = FONT_SIZE;
        let text_x_position = ((width - text_width) as i32 / 2) + x;
        let text_y_position = ((height - text_height as u32) as i32 / 2) + y;



        ButtonBuilder {
            border: Rect::new(x, y, width, height),
            inner: Rect::new(x+2, y+2, width-4, height-4),
            _is_clicked: false,
            is_enabled: true,
            
            text: text.to_string(),
            text_wrapper: Rect::new(text_x_position, text_y_position, text_width, text_height as u32),
            text_texture: texture.raw(),
            
            on_click_fn: None,
        }
    }

    pub fn on_click(mut self, f: Box<dyn FnMut() -> ()>) -> ButtonBuilder {
        self.on_click_fn = Some(f);
        self
    }

    pub fn build(self) -> Button {
        Button {
            border: self.border,
            inner: self.inner,
            _is_clicked: self._is_clicked,
            is_enabled: self.is_enabled,
            
            text: self.text,
            text_wrapper: self.text_wrapper,
            text_texture: self.text_texture,

            on_click_fn: self.on_click_fn
        }
    }
}


pub struct Button {
    border: Rect,
    inner: Rect,

    _is_clicked: bool,
    is_enabled: bool,

    pub text: String,
    pub text_wrapper: Rect,
    pub text_texture: *mut SDL_Texture,

    on_click_fn: Option<Box<dyn FnMut() -> ()>>
}

impl Button {
    pub fn new(text: &str, width: u32, height: u32, x: i32, y: i32, canvas: &Canvas<Window>) -> ButtonBuilder {
        ButtonBuilder::new(text, width, height, x, y, canvas)
    }

    pub fn click(&mut self) {
        self.on_click()
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = String::from(new_text)
    }

    fn get_text_texture(&self, canvas: &Canvas<Window>) -> Texture {
        unsafe {
            canvas.raw_create_texture(self.text_texture)
        }
    }
}

impl RuiObject for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // ToDo: Переделать отрисовку текста, текстура не создавалась заново каждый раз

        canvas.set_draw_color(Color::RGB(211, 167, 177));
        canvas.draw_rect(self.border).unwrap();
        canvas.set_draw_color(Color::RGB(200, 162, 200));
        canvas.fill_rect(self.inner).unwrap(); 
        
        let texture = self.get_text_texture(&canvas);
        canvas.copy(&texture, None, Some(self.text_wrapper)).unwrap();
    }

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
