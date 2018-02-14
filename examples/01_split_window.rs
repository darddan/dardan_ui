extern crate dardan_ui;

use dardan_ui::{new_ui_cell, UiApp, UiCol, UiElem, UiSize, UiSizeVal};
use dardan_ui::elements::{Horizontal, UiFill};

pub fn main() {
    let mut app = UiApp::new();

    let mut main_container = Horizontal::new();
    main_container.set_background_color(UiCol::blue());

    let mut fst_child = UiFill::new();
    fst_child.set_background_color(UiCol::salmon());
    fst_child.set_size(UiSize {
        x: UiSizeVal::Px(250),
        y: UiSizeVal::Max,
    });
    main_container.add_child(new_ui_cell(fst_child));

    let mut snd_child = UiFill::new();
    snd_child.set_background_color(UiCol::red());
    main_container.add_child(new_ui_cell(snd_child));

    app.set_main_element(new_ui_cell(main_container));

    app.run();
}
