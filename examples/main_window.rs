use rui::init;
use rui::buttons::Button;

fn main() {
    let main_window = init("Main window test", 800, 600);
    let mw_clone = main_window.clone();
    let mut main_window_rc = main_window.borrow_mut();
    
    let button = Button::new("Exit", 120, 40, 160, 10, &main_window_rc.canvas)
        .on_click(Box::new(move || {
            let mut main_window = mw_clone.borrow_mut();
            // ToDo: borrow_mut вылетает в панике, т.к. выше уже захвачено мутабельно - надо переделать
            main_window.close()
        }))
        .build();

    main_window_rc.add_child(button);

    main_window_rc.exec()
}
