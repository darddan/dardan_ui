# dardan_ui

**dardan_ui** is a simple GUI Toolkit based on [SDL2](https://github.com/Rust-SDL2/rust-sdl2). The library should be simple to use and be similar to a GUI Toolkit of an OO language.

#### Why am I creating this library?

* I have started learning Rust, and this seems like a good project to improve my skills
* The gui libraries that exist for now (AFAIK) are complex, complicated or need to be run as a server.
* (Subjective) The other libraries are not intuitive.
* (Subjective) Creating custom *ui elements* in other libraries is complicated.

## How to use **(goal)**

This is how I intend for the ibrary to e used when it is done. Here are some simple use cases, but the idea is not complete yet.

#### Simple Ui Application:

This is what you need to run an application, which seems pretty simple

```rust
extern crate dardan_ui;

use dardan_ui::UiApp;

pub fn main() {
    UiApp::new().run();
}
```

#### Responsive window with sidebar:

```rust
extern crate dardan_ui;

use std::rc::Rc;
use std::cell::RefCell;

use dardan_ui::{UiApp, UiColor, UiPair, UiRelSize, UiUnit};
use dardan_ui::combos::UiRowCombo;

pub fn main() {
    let mut app = UiApp::new();

    let mut main_container = UiRowCombo::new();

    let mut fst_child = UiRowCombo::new();
    fst_child.set_background_color(UiColor::salmon());
    main_container.add_child(UiRelSize::Px(200), Rc::new(RefCell::new(fst_child)));

    let mut snd_child = UiRowCombo::new();
    snd_child.set_background_color(UiColor::red());
    main_container.add_child(UiRelSize::Max, Rc::new(RefCell::new(snd_child)));

    app.set_main_element(Rc::new(RefCell::new(main_container)));

    app.run();
}
```

Even though this isn't that hard to understand, this doesn't seem that simple.
By using macros it could look like this:
```rust
extern crate dardan_ui;

ui!( 
    define UiRowCombo : main_container {
        children : [
            RefCell : fst_child {
                size_x : 200 px,
                size_y : Max,
                background_color : salmon
            },
            RefCell : snd_child {
                size_x : Max,
                size_y : Max,
                background_color : red
            }
        ]
    }
);

pub fn main() {
    let mut app = UiApp::new();

    ui!(set_main app main_container);
    
    app.run();
}
```

And if we don't want to specify anything in main, then we could replace the whole main function with `ui![run main_container]`

## Contribute

Because this is my first project in rust, there will be a lot of mistakes and a lot of things you could improve on. Some of them are:

* Rust best-practices (My first time programming in Rust)
* Better use of SDL2 and improvments (My first time using SDL2 too)
* Documentation (Even in the places it is documented, my english is not that good)
* If you find bugs and have new ideas just create issues for them
* Implement tasks from issues