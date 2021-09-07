use rui::init;
use rui::buttons::Button;
use rui::checkbox::RuiCheckbox;


fn main() {
    let main_window = init("Main window test", 800, 600);
    let mut main_window_rc = main_window.borrow_mut();
    
    let chbx = RuiCheckbox::new(16, 385, 285);

    for i in 0..3 {
        let btn_text = &format!("Button {}", i)[..];
        let checkbx_rc = chbx.clone();
        
        let btn = Button::new(btn_text, 120, 40, 20, 10 + 45 * i, &main_window_rc.canvas)
            .on_click(Box::new(move || {
                let mut checkbox = checkbx_rc.borrow_mut();
                checkbox.toggle()
            }))
            .build();
        
            main_window_rc.add_child(btn);
    }
    
    main_window_rc.add_child(chbx);

    main_window_rc.exec()
}
