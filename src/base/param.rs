use {UiAttr, UiCell, UiElem};

pub enum UiParam {
    Attr(UiAttr),
    Child(UiCell<UiElem>),
}
