use {UiAttr, UiCell, UiElem, UiRelSize};

pub enum UiParam {
    Attr(UiAttr),
    Child(UiCell<UiElem>),
    RelChild(UiRelSize, UiCell<UiElem>),
}
