extern crate dardan_ui;

use dardan_ui::{new_ui_cell, UiApp, UiCol, UiElem, UiSizeVal};
use dardan_ui::elements::{UiFill, UiHorizontal};

pub fn main() {
    let mut app = UiApp::new();

    let mut main_container = UiHorizontal::new();

    let mut fst_child = UiFill::new();
    fst_child.set_background_color(UiCol::salmon());
    fst_child.set_x(UiSizeVal::Px(250));
    main_container.add_child(new_ui_cell(fst_child));

    let mut snd_child = UiFill::new();
    snd_child.set_background_color(UiCol::red());
    main_container.add_child(new_ui_cell(snd_child));

    app.set_main_element(new_ui_cell(main_container));

    app.run();
}
