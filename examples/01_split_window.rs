extern crate dardan_ui;

use std::rc::Rc;
use std::cell::RefCell;

use dardan_ui::{new_ui_cell, UiApp, UiCell, UiCol, UiPair, UiRelSize};
use dardan_ui::combos::UiRowCombo;

pub fn main() {
    let mut app = UiApp::new();

    let mut main_container = UiRowCombo::new();

    let mut fst_child = UiRowCombo::new();
    fst_child.set_background_color(UiCol::salmon());
    main_container.add_child(UiRelSize::Px(200), new_ui_cell(fst_child));

    let mut snd_child = UiRowCombo::new();
    snd_child.set_background_color(UiCol::red());
    main_container.add_child(UiRelSize::Max, new_ui_cell(snd_child));

    app.set_main_element(new_ui_cell(main_container));

    app.run();
}
