use crate::{UiAttr, UiCell, UiElem};

pub enum UiParam {
    Attr(UiAttr),
    Child(UiCell<dyn UiElem>),
}
