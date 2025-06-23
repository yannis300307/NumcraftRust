use alloc::{string::String, vec::Vec};

enum MenuElement {
    Button {
        text: String,
        is_pressed: bool
    },

}

pub struct Menu {
    elements: Vec<MenuElement>
}