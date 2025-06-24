use alloc::{string::String, vec::Vec};
use nalgebra::Vector2;

pub enum MenuElement {
    /// A simple button
    Button { text: String, is_pressed: bool },
    /// A slider giving values between 0 and 1
    Slider {
        text: String,
        value: f32,
        step_size: f32,
    },
    /// A simple text
    Label {
        text: String,
        text_anchor: TextAnchor,
    },
}

pub enum TextAnchor {
    Left,
    Right,
    Center,
}

pub struct Menu {
    elements: Vec<MenuElement>,
    pub pos: Vector2<usize>,
    pub width: usize,
    pub selected_index: usize,
}

impl Menu {
    pub fn new(pos: Vector2<usize>, width: usize) -> Self {
        Menu {
            elements: Vec::new(),
            pos,
            width,
            selected_index: 0,
        }
    }

    pub fn add_element(mut self, element: MenuElement) -> Self {
        self.elements.push(element);
        self
    }

    pub fn cursor_down(&mut self) { // Remake this to avoid selecting labels
        if self.selected_index == self.elements.len() - 1 {
            self.selected_index = 0;
        } else {
            self.selected_index += 1;
        }
    }

    pub fn cursor_up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.elements.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn get_elements(&self) -> &Vec<MenuElement> {
        &self.elements
    }
}
