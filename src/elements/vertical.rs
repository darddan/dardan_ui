use crate::{UiAttr, UiCell, UiCol, UiElem, UiFixSize, UiParam, UiPos, UiSize, UiSizeVal};

pub struct UiVertical {
    elements: Vec<UiCell<dyn UiElem>>,
    background_color: ::sdl2::pixels::Color,
    size: UiSize,
    fix_size: UiFixSize,
    needed_size: UiFixSize,
}

impl UiVertical {
    pub fn new() -> Self {
        UiVertical {
            elements: vec![],
            background_color: ::sdl2::pixels::Color::RGBA(0, 0, 0, 0),
            size: UiSize::new(),
            fix_size: UiFixSize::new(),
            needed_size: UiFixSize::new(),
        }
    }

    pub fn add_child(&mut self, child: UiCell<dyn UiElem>) {
        self.elements.push(child);
        self.calculate_children_size();
    }

    fn calculate_children_size(&mut self) {
        let mut max_x_value = 0;
        let mut num_of_elements_rel: u32 = 0;

        let mut sum_of_elements_px: u32 = 0;
        let mut sum_of_elements_rel: u32 = 0;

        let mut elem_queue = vec![];

        for elem in &self.elements {
            let mut borrowed_element = elem.write().unwrap();
            let elem_size = borrowed_element.get_size().y;

            match elem_size {
                UiSizeVal::Max => {
                    elem_queue.push(elem.clone());
                    num_of_elements_rel += 1;
                    sum_of_elements_rel += 1;
                }
                UiSizeVal::Rel(val) => {
                    elem_queue.push(elem.clone());
                    num_of_elements_rel += 1;
                    sum_of_elements_rel += val as u32;
                }
                UiSizeVal::Min => {
                    let val = borrowed_element.get_needed_y();
                    borrowed_element.set_fix_size(UiFixSize {
                        x: self.fix_size.x,
                        y: val,
                    });
                    sum_of_elements_px += val;
                }
                UiSizeVal::Px(val) => {
                    borrowed_element.set_fix_size(UiFixSize {
                        x: self.fix_size.x,
                        y: val,
                    });
                    sum_of_elements_px += val;
                }
            }

            if let UiSizeVal::Px(val) = borrowed_element.get_size().x {
                if val > max_x_value {
                    max_x_value = val;
                }
            }
        }

        let size_left_for_relatives = if self.fix_size.y <= sum_of_elements_px {
            0
        } else {
            self.fix_size.y - sum_of_elements_px
        };

        if size_left_for_relatives == 0 {
            let y_val = self.fix_size.y;
            self.set_my_sizes(y_val, max_x_value);
            return;
        }

        if num_of_elements_rel == 0 {
            self.set_my_sizes(sum_of_elements_px, max_x_value);
            return;
        }

        let rel_multiplier: f32 = size_left_for_relatives as f32 / sum_of_elements_rel as f32;

        for elem_iter in elem_queue {
            let mut elem = elem_iter.write().unwrap();
            let elem_size = elem.get_size().y;
            match elem_size {
                UiSizeVal::Max => {
                    let new_y = rel_multiplier as u32;
                    elem.set_fix_size(UiFixSize {
                        x: self.fix_size.x,
                        y: new_y,
                    });
                    sum_of_elements_px += new_y;
                }
                UiSizeVal::Rel(val) => {
                    let new_y = (val as f32 * rel_multiplier) as u32;
                    elem.set_fix_size(UiFixSize {
                        x: self.fix_size.x,
                        y: new_y,
                    });
                    sum_of_elements_px += new_y;
                }
                _ => (),
            }
        }
        self.set_my_sizes(sum_of_elements_px, max_x_value);
    }

    fn set_my_sizes(&mut self, sum_of_elements_px: u32, max_x_value: u32) {
        let mut needed_size = UiFixSize::new();
        needed_size.x = crate::elements::get_needed_val(self.size.x);
        needed_size.y = crate::elements::get_needed_val(self.size.y);

        if needed_size.x == 0 {
            self.needed_size.x = max_x_value;
        }

        if needed_size.y == 0 {
            self.needed_size.y = sum_of_elements_px;
        }
    }

    pub fn set_background_color(&mut self, color: UiCol) {
        self.background_color = color.sdl2();
    }
}

impl UiElem for UiVertical {
    fn draw(&self, canvas: &mut ::sdl2::render::Canvas<::sdl2::video::Window>, cv_pos: &UiPos) {
        crate::util::draw_rect(canvas, cv_pos, &self.fix_size, self.background_color);

        let mut count_elements_size = 0;
        for elem_iter in &self.elements {
            let elem = elem_iter.read().unwrap();
            let elem_size = elem.get_fix_size().y;
            if count_elements_size + elem_size > self.fix_size.y {
                break;
            }

            elem.draw(
                canvas,
                &UiPos {
                    x: cv_pos.x,
                    y: cv_pos.y + count_elements_size as i32,
                },
            );

            count_elements_size += elem_size;
        }
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::BackgroundColor(val) => self.set_background_color(val),
            UiAttr::FixSize(val) => self.set_fix_size(val),
            UiAttr::Size(val) => self.set_size(val),
            _ => (),
        }
    }

    fn set_value(&mut self, param: UiParam) {
        match param {
            UiParam::Attr(val) => self.set_attribute(val),
            UiParam::Child(val) => self.add_child(val),
        }
    }

    ui_define_size_functions!(Size: size myself {
        if myself.elements.len() != 0 {
            let used_var = crate::elements::get_needed_val(myself.size.x);
            if used_var != 0 { myself.needed_size.x = used_var }

            let used_var = crate::elements::get_needed_val(myself.size.y);
            if used_var != 0 { myself.needed_size.y = used_var }
        }
    });

    ui_define_size_functions!(FixSize: fix_size myself {
        if myself.elements.len() == 0 {
            myself.fix_size.x = 0;
        } else {
            myself.calculate_children_size();
        }
    });

    ui_define_size_functions!(NeededSize: needed_size);
}
