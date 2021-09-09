use rui::init;
use rui::button::Button;
use std::rc::Rc;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let main_window_rc = init("Buttons test", WIDTH, HEIGHT);
    let mut main_window = main_window_rc.lock().unwrap();

    let btn = Button::new("Start", 120, 40, 160, 50, &main_window.canvas)
        .build();

    let btn_ref = Rc::clone(&btn);
    
    let btn2 = Button::new("End", 120, 40, 160, 10, &main_window.canvas)
        .on_click(Box::new(move || {
            let mut btn = btn_ref.borrow_mut();
            let new_x_position = sum(btn.pos_x(), 10);
            btn.set_x(new_x_position);
        }))
        .build();

    main_window.add_child(btn);
    main_window.add_child(btn2);

    main_window.exec()
}