extern crate dardan_ui;

use std::rc::Rc;
use std::cell::RefCell;

use dardan_ui::{UiApp, UiCol, UiPair, UiRelSize, UiCell, new_ui_cell};
use dardan_ui::combos::UiRowCombo;

pub fn main() {
    let mut app = UiApp::new();

    let mut main_container = UiRowCombo::new();
    
    let mut fst_child = UiRowCombo::new();
    fst_child.set_background_color(UiCol::salmon());
    main_container.add_child(UiRelSize::Px(200), Rc::new(RefCell::new(fst_child)));
    
    let mut snd_child = UiRowCombo::new();
    snd_child.set_background_color(UiCol::red());
    main_container.add_child(UiRelSize::Max, Rc::new(RefCell::new(snd_child)));
    
    app.set_main_element(new_ui_cell(main_container));

    app.run();
}
