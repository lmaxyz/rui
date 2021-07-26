mod buttons;

use rui::init;
use rui::buttons::Button;
pub use rui::Drawaible;

fn main() {
    let btn = Button::new("My btn", 120, 40, 20, 10);

    let mut main_window = init("Title", 800, 600);
    main_window.add_button(btn);

    main_window.exec()
}
