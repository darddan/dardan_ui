use {UiAttr, UiCol, UiDirection, UiPair, UiRelSize, UiParam, UiElem, UiCell, new_ui_cell};

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub struct UiRowCombo {
    elements: Vec<UiCell<UiElem>>,
    elements_rel_size: Vec<UiRelSize>,
    elements_fix_size: Vec<u32>,
    direction: UiDirection,
    background_color: Color,
    size: UiPair<u32>,
    canvas_pos: UiPair<i32>,
    local_pos: UiPair<i32>,
}

impl UiElem for UiRowCombo {
    fn draw(&self, canvas: &mut Canvas<Window>, cv_pos: &UiPair<i32>) {
        canvas.set_draw_color(self.background_color);
        let rect = Rect::new(cv_pos.x, cv_pos.y, self.size.x, self.size.y);
        let _ = canvas.fill_rect(rect);
        // TODO : Delete next line
        println!("Painted: {}, {}, {}, {}", self.background_color.r, self.background_color.g, self.background_color.b, self.background_color.a);
        println!("\tover {} {} {} {}", cv_pos.x, cv_pos.y, self.size.x, self.size.y);
        if self.direction == UiDirection::Horizontal {
            let mut sum = 0; 
            for (i, item) in self.elements.iter().enumerate() {
                let elem = item.read().unwrap();
                let elem_size = elem.get_size();
                let elem_space = self.elements_fix_size[i];

                if sum + elem_space > self.size.x {
                    break;
                }

                let diff = if elem_size.x < elem_space {
                    cv_pos.x + sum as i32 + ((elem_space - elem_size.x) / 2) as i32
                } else {
                    cv_pos.x + sum as i32
                };
                
                elem.draw(canvas, &UiPair { x: diff , y : cv_pos.y});
                

                sum += elem_space;
            }
        } else {
            let mut sum = 0; 
            for (i, item) in self.elements.iter().enumerate() {
                let elem = item.read().unwrap();
                let elem_size = elem.get_size();
                let elem_space = self.elements_fix_size[i];

                if sum + elem_space > self.size.y {
                    break;
                }

                let diff = if elem_size.y < elem_space {
                    cv_pos.y + sum as i32 + ((elem_space - elem_size.y) / 2) as i32
                } else {
                    cv_pos.y + sum as i32
                };
                
                elem.draw(canvas, &UiPair { x: cv_pos.x , y : diff});
                

                sum += elem_space;
            }
            
        }
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::BackgroundColor(color) => self.set_background_color(color),
            UiAttr::Size(size) => self.set_size(size),
            UiAttr::Direction(dir) => self.set_direction(dir),
            _ => (),
        }
    }

    fn set_value(&mut self, value: UiParam) {
        match value {
            UiParam::Attr(attr) => self.set_attribute(attr),
            UiParam::RelChild(rel_size, child) => self.add_child(rel_size, child),
            _ => (),
        }
    }

    fn get_size(&self) -> UiPair<u32> {
        self.size.clone()
    }

    fn set_size(&mut self, size: UiPair<u32>) {
        self.size = size;
        if self.elements.len() != 0 {
            self.calculate_elements_fix_size();
        }
    }
}

impl UiRowCombo {
    pub fn new() -> Self {
        UiRowCombo {
            elements: vec![],
            elements_rel_size: vec![],
            elements_fix_size: vec![],
            direction: UiDirection::Horizontal,
            background_color: Color::RGB(0, 0, 0),
            size: UiPair::new_u32(),
            canvas_pos: UiPair::new_i32(),
            local_pos: UiPair::new_i32(),
        }
    }

    pub fn set_background_color(&mut self, color: UiCol) {
        self.background_color = color.sdl2();
    }

    pub fn set_direction(&mut self, dir: UiDirection) {
        self.direction = dir;
    }

    pub fn add_child(&mut self, rel_size: UiRelSize, child: UiCell<UiElem>) {
        self.elements.push(child);
        if let UiRelSize::Px(fix_size) = rel_size {
            self.elements_fix_size.push(fix_size);
        } else {
            self.elements_fix_size.push(0);
        }
        
        self.elements_rel_size.push(rel_size);
    }

    fn calculate_elements_fix_size(&mut self) {
        println!("Size of array start: {}", self.elements.len());
        let mut track : Vec<usize> = vec![];

        let mut count_fix_values = 0;
        let mut count_rel_values = 0;

        let mut count_max_elements = 0;
        let mut count_rel_elements = 0;

        for (i, item) in self.elements_rel_size.iter().enumerate() {
            match item {
                &UiRelSize::Max => {
                    track.push(i);
                    count_max_elements += 1;
                },
                &UiRelSize::Inherit => {
                    let mut elem = self.elements[i].write().unwrap();
                    elem.set_size(self.size.clone());

                    let elem_size = if self.direction == UiDirection::Horizontal { elem.get_size().x } else { elem.get_size().y };
                    self.elements_fix_size[i] = elem_size;
                    count_fix_values += elem_size;
                },
                &UiRelSize::Px(val) => count_fix_values += val,
                &UiRelSize::Rel(val) => {
                    track.push(i);
                    count_rel_values += val as u32;
                    count_rel_elements += 1;
                },
            }
        }

        if count_rel_values > 100 {
            count_fix_values = 100;
        }
        println!("The values: {} {} {} {}",count_fix_values, count_rel_values, count_max_elements, count_rel_elements);
        let draw_size = if self.direction == UiDirection::Horizontal { self.size.x } else { self.size.y };
        println!("\t\tDraw size: {}", draw_size);
        let size_left_for_rel = (draw_size - count_fix_values) as f32;
        println!("\t\tsize_left_for_rel: {}", size_left_for_rel);
        let rel_to_fix = if draw_size <= count_fix_values { 0.0 } else { size_left_for_rel * (count_rel_values as f32 / 100.0) / count_rel_elements as f32 };
        println!("\t\trel_to_fix: {}", rel_to_fix);
        let size_left_for_auto = draw_size - rel_to_fix as u32 - count_fix_values;
        println!("\t\tsize_left_for_auto: {}", size_left_for_auto);
        let auto_size = size_left_for_auto / count_max_elements;
        println!("\t\tauto_size: {}", auto_size);

        for i in track {
            match self.elements_rel_size[i] {
                UiRelSize::Max => {
                    println!("Set elem {} size from {} to {}", i, self.elements_fix_size[i], auto_size);
                    self.elements_fix_size[i] = auto_size;
                },
                UiRelSize::Rel(val) => self.elements_fix_size[i] = (size_left_for_rel / val as f32) as u32,
                _ => (),
            }
        }
        println!("Size of array end: {}", self.elements.len());
        
        if self.direction == UiDirection::Horizontal {
            let mut it = 0;
            for elem in &mut self.elements {
                println!("\t\t\t el {}: {}", it, self.elements_fix_size[it]);
                let new_size = UiPair {
                    x: self.elements_fix_size[it],
                    y: self.size.y,
                };
                elem.write().unwrap().set_size(new_size);
                it += 1;
            }
        } else {
            let mut it = 0;
            for elem in &mut self.elements {
                let new_size = UiPair {
                    x: self.size.x,
                    y: self.elements_fix_size[it],
                };
                elem.write().unwrap().set_size(new_size);
                it += 1;
            }
        }
    }
}
