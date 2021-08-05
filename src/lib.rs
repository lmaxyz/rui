use sdl2;
use sdl2::VideoSubsystem;
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

use buttons::Button;
pub use clickable::Clickable;

pub trait Drawaible {
    fn draw(&self, _: &mut Canvas<Window>) {}
}

pub struct RuiMainWindow<F: FnMut() -> ()> {
    title: String,
    width: u32,
    height: u32,
    context: sdl2::Sdl,
    video: VideoSubsystem,
    buttons: Vec<Button<F>>
}

impl<F> RuiMainWindow<F>
where F: FnMut() -> () {
    pub fn exec(&mut self) {
        let window = self.video.window(self.title.as_str(), self.width, self.height).build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
        
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
                        for button in self.buttons.iter_mut() {
                            if button.include_coords(x, y) {
                                button.on_click()
                            }
                        }
                            
                        
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();
            
            for button in self.buttons.iter() {
                button.draw(&mut canvas);
            }

            canvas.present();

            let time_end = time::SystemTime::now();
            let frame_time = time_end.duration_since(time_start).unwrap().as_secs_f64();
            second_counter += frame_time;
            
            if second_counter >= 1.0 {
                println!("{}", frame_time);
                second_counter = 0.0
            }

            if frame_time < 0.001 {
                thread::sleep(time::Duration::from_secs_f64(0.001));
                second_counter += 0.001
            }
        }
    }

    pub fn add_button(&mut self, button: Button<F>) {
        self.buttons.push(button)
    }
}

pub fn init<F: FnMut() -> ()>(title: &str, width: u32, height: u32) -> RuiMainWindow<F> {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    
    RuiMainWindow {
        title: title.to_string(),
        width,
        height,
        context,
        video,
        buttons: Vec::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
