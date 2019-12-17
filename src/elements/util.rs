macro_rules! ui_define_size_functions {
    (Size: $struct_var:ident) => (
        ui_define_size_functions!(Size: $struct_var _no_self {});
    );
    (Size: $struct_var:ident $self_var:ident $run_on_set:block) => (
        fn set_size(&mut self, size: UiSize) {
            self.$struct_var = size;
            let $self_var = self;
            $run_on_set;
        }
        fn set_x(&mut self, x: UiSizeVal) {
            self.$struct_var.x = x;
            let $self_var = self;
            $run_on_set;
        }
        fn set_y(&mut self, y: UiSizeVal) {
            self.$struct_var.y = y;
            let $self_var = self;
            $run_on_set;
        }
        fn get_size(&self) -> UiSize {
            self.$struct_var.clone()
        }
        fn get_x(&self) -> UiSizeVal {
            self.$struct_var.x.clone()
        }
        fn get_y(&self) -> UiSizeVal {
            self.$struct_var.y.clone()
        }
    );
    (FixSize: $struct_var:ident) => (
        ui_define_size_functions!(FixSize: $struct_var _no_self {});
    );
    (FixSize: $struct_var:ident $self_var:ident $run_on_set:block) => (
        fn set_fix_size(&mut self, size: UiFixSize) {
            self.$struct_var = size;
            let $self_var = self;
            $run_on_set;
        }
        fn set_fix_x(&mut self, x: u32) {
            self.$struct_var.x = x;
            let $self_var = self;
            $run_on_set;
        }
        fn set_fix_y(&mut self, y: u32) {
            self.$struct_var.y = y;
            let $self_var = self;
            $run_on_set;
        }
        fn get_fix_size(&self) -> UiFixSize {
            self.$struct_var.clone()
        }
        fn get_fix_x(&self) -> u32 {
            self.$struct_var.x
        }
        fn get_fix_y(&self) -> u32 {
            self.$struct_var.y
        }
    );
    (NeededSize: $struct_var:ident) => (
        fn get_needed_size(&self) -> UiFixSize {
            self.$struct_var.clone()
        }
        fn get_needed_x(&self) -> u32 {
            self.$struct_var.x
        }
        fn get_needed_y(&self) -> u32 {
            self.$struct_var.y
        }
    );
    ($fn_type:ty : $struct_var:ident) => ();
}

#[inline(always)]
pub fn get_needed_val(size: crate::UiSizeVal) -> u32 {
    if let crate::UiSizeVal::Px(val) = size {
        val
    } else {
        0
    }
}
