use sdl2;
use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod buttons;

use buttons::Button;

pub trait Drawaible {
    fn draw(&self, _: &mut Canvas<Window>) {}
}

pub struct RuiMainWindow {
    title: String,
    width: u32,
    height: u32,
    context: sdl2::Sdl,
    video: VideoSubsystem,
    buttons: Vec<Button>
}

impl RuiMainWindow {
    pub fn exec(&mut self) {
        let window = self.video.window(self.title.as_str(), self.width, self.height).build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        
        let mut event_pump = self.context.event_pump().unwrap();
        
        'running: loop {
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            
            for button in self.buttons.iter() {
                button.draw(&mut canvas);
            }

            canvas.present();
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button)
    }
}

pub fn init(title: &str, width: u32, height: u32) -> RuiMainWindow {
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
