use rui::init;
use rui::buttons::Button;

fn sum() {
    println!("Yeah!!!")
}

fn ok() {
    println!("This is another func")
}

fn main() {
    let btn = Button::new("Start", 120, 40, 20, 10, Box::new(sum));
    let btn2 = Button::new("End", 120, 40, 160, 10, Box::new(ok));

    let mut main_window = init("Title", 800, 600);
    main_window.add_child(Box::new(btn));
    main_window.add_child(Box::new(btn2));

    main_window.exec()
}
