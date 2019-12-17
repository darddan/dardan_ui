#[allow(dead_code)]
pub enum UiSizeUnit {
    Min,
    Max,
    Px(u32),
    Pt(u32),
    Rel(f32),
    RelParent(f32),
    RelParentWidth(f32),
    RelParentHeight(f32),
}
