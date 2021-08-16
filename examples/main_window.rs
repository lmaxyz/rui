use rui::init;
use rui::buttons::Button;
use rui::checkbox::RuiCheckbox;

fn sum() {
    println!("Yeah!!!")
}

fn main() {
    let mut main_window = init("Fantasy World", 800, 600);

    
    let btn2 = Button::new("End", 120, 40, 160, 10, &main_window.canvas)
        .build();

    let chbx = RuiCheckbox::new(16, 385, 285);

    for i in 0..3 {
        let btn_text = &format!("Button {}", i)[..];
        
        let btn = Button::new(btn_text, 120, 40, 20, 10 + 45 * i, &main_window.canvas)
            .on_click(Box::new(sum))
            .build();
        
            main_window.add_child(Box::new(btn));
    }

    main_window.add_child(Box::new(btn2));
    main_window.add_child(Box::new(chbx));

    main_window.exec()
}
