mod buttons;

use rui::init;
use rui::buttons::Button;
use rui::Clickable;
pub use rui::Drawaible;

fn sum() {
    println!("Yeah!!!")
}

fn main() {
    let btn = Button::new("My btn", 120, 40, 20, 10, sum);
    let btn2 = Button::new("My btn2", 120, 40, 160, 10, sum);

    let mut main_window = init("Title", 800, 600);
    main_window.add_button(btn);
    main_window.add_button(btn2);

    main_window.exec()
}
