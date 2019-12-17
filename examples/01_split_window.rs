extern crate dardan_ui;

use dardan_ui::elements::{UiFill, UiHorizontal};
use dardan_ui::{new_ui_cell, UiApp, UiCol, UiElem, UiSizeVal};

pub fn main() {
    let mut app = UiApp::default();

    let mut main_container = UiHorizontal::default();

    let mut fst_child = UiFill::default();
    fst_child.set_background_color(UiCol::salmon());
    fst_child.set_x(UiSizeVal::Px(250));
    main_container.add_child(new_ui_cell(fst_child));

    let mut snd_child = UiFill::default();
    snd_child.set_background_color(UiCol::red());
    main_container.add_child(new_ui_cell(snd_child));

    app.set_main_element(new_ui_cell(main_container));

    app.run();
}
