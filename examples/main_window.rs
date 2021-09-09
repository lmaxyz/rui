use rui::init;
use rui::button::Button;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

fn main() {
    let main_window = init("Main window test", WIDTH, HEIGHT);
    let mw_clone = main_window.clone();
    let mut main_window_rc = main_window.lock().unwrap();
    
    let btn_pos_x = (WIDTH/2 - 60) as i32;
    let btn_pos_y = (HEIGHT/2 - 20) as i32;
    let button = Button::new("Exit", 120, 40, btn_pos_x, btn_pos_y, &main_window_rc.canvas)
        .on_click(Box::new(move || {
            let main_window = mw_clone.lock().unwrap();
            // ToDo: borrow_mut вылетает в панике, т.к. выше уже захвачено мутабельно - надо переделать
            main_window.close()
        }))
        .build();

    main_window_rc.add_child(button);

    main_window_rc.exec()
}
