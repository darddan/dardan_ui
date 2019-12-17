extern crate dardan_ui;

use dardan_ui::elements::{UiFill, UiHorizontal, UiSpace, UiVertical};
use dardan_ui::{new_ui_cell, UiApp, UiCol, UiElem, UiSize, UiSizeVal};

pub fn main() {
    let mut app = UiApp::default();

    let mut container_ver = UiVertical::default();
    let mut container_hor = UiHorizontal::default();
    container_hor.set_y(UiSizeVal::Min);

    let mut centered_child = UiFill::default();
    centered_child.set_background_color(UiCol::salmon().set_transparency(150).set_green(233));
    centered_child.set_size(UiSize {
        x: UiSizeVal::Px(250),
        y: UiSizeVal::Px(250),
    });

    container_hor.add_child(new_ui_cell(UiSpace::default()));
    container_hor.add_child(new_ui_cell(centered_child));
    container_hor.add_child(new_ui_cell(UiSpace::default()));

    container_ver.add_child(new_ui_cell(UiSpace::default()));
    container_ver.add_child(new_ui_cell(container_hor));
    container_ver.add_child(new_ui_cell(UiSpace::default()));

    app.set_main_element(new_ui_cell(container_ver));

    app.run();
}
