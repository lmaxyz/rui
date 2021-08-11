use rui::init;
use rui::buttons::Button;

fn sum() {
    println!("Yeah!!!")
}

fn main() {
    let mut main_window = init("Fantasy World", 800, 600);

    
    let btn2 = Button::new("End", 120, 40, 160, 10, &main_window.canvas, None);

    for i in 0..3 {
        let btn_text = format!("Button {}", i);
        let btn = Button::new(btn_text.as_str(), 120, 40, 20, 10 + 45 * i, &main_window.canvas, Some(Box::new(sum)));
        main_window.add_child(Box::new(btn));
    }

    main_window.add_child(Box::new(btn2));

    main_window.exec()
}
