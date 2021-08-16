use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;

use std::time;
use std::thread;

pub mod buttons;
pub mod clickable;
pub mod widget;
pub mod checkbox;

pub use clickable::Clickable;

pub trait RuiObject {
    fn draw(&self, _: &mut Canvas<Window>) {}
    fn include_coords(&self, x: i32, y: i32) -> bool;
    fn on_click(&mut self) {}
}

pub struct RuiMainWindow {
    width: u16,
    height: u16,
    context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    childs: Vec<Box<dyn RuiObject>>
}

impl RuiMainWindow {
    pub fn exec(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        self.canvas.clear();
        self.canvas.present();
        
        let mut event_pump = self.context.event_pump().unwrap();
        
        let mut second_counter = 0.0;

        'running: loop {    
            let time_start = time::SystemTime::now();
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::MouseButtonDown {mouse_btn: MouseButton::Left, x, y, ..} => {
                        for child in self.childs.iter_mut() {
                            if child.include_coords(x, y) {
                                child.on_click()
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();
            
            for child in self.childs.iter() {
                child.draw(&mut self.canvas);
            }

            self.canvas.present();

            let time_end = time::SystemTime::now();
            let frame_time = time_end.duration_since(time_start).unwrap().as_secs_f64();
            second_counter += frame_time;
            
            if second_counter >= 1.0 {
                println!("{}", frame_time);
                second_counter = 0.0
            }

            if frame_time < 0.01 {
                thread::sleep(time::Duration::from_secs_f64(0.05));
                second_counter += 0.05
            }
        }
    }

    pub fn add_child(&mut self, child: Box<dyn RuiObject>) {
        self.childs.push(child)
    }
}

impl RuiObject for RuiMainWindow {
    fn include_coords(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width as i32 && y > 0 && y < self.height as i32
    }
}

pub fn init(title: &str, width: u16, height: u16) -> RuiMainWindow {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window(title, width as u32, height as u32).build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    
    RuiMainWindow {
        width,
        height,
        context,
        canvas,
        childs: Vec::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
