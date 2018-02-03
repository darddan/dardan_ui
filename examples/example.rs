extern crate dardan_ui;

use dardan_ui::UiApp;
use dardan_ui::containers::UiFreeContainer;

pub fn main() {
    let mut app = UiApp::new(String::from("Title"));

    app.set_main_element(Box::new(UiFreeContainer::new()));

    app.run();
}
