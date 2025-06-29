use alloc::{string::String, vec::Vec};
use nalgebra::Vector2;

use crate::eadk::{self, input::KeyboardState};

pub enum MenuElement {
    /// A simple button
    Button {
        text: String,
        is_pressed: bool,
        allow_margin: bool,
        id: usize,
    },
    /// A slider giving values between 0 and 1
    Slider {
        text_fn: fn(f32) -> String,
        value: f32,
        step_size: f32,
        allow_margin: bool,
        id: usize,
    },
    /// A simple text
    Label {
        text: String,
        text_anchor: TextAnchor,
        allow_margin: bool,
    },
    /// A space
    Void { allow_margin: bool },

    ButtonOption {
        text: String,
        is_pressed: bool,
        id: usize,
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
    pub need_redraw: bool,
}

impl Menu {
    pub fn check_inputs(
        &mut self,
        keyboard_state: KeyboardState,
        just_pressed_keyboard_state: KeyboardState,
    ) {
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Down) {
            self.cursor_down();
        }
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Up) {
            self.cursor_up();
        }
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Right) {
            self.cursor_right();
        }
        if just_pressed_keyboard_state.key_down(eadk::input::Key::Left) {
            self.cursor_left();
        }

        if just_pressed_keyboard_state.key_down(eadk::input::Key::Ok) {
            self.set_pressed(true);
        }
    }
    pub fn new(pos: Vector2<usize>, width: usize, start_index: usize) -> Self {
        Menu {
            elements: Vec::new(),
            pos,
            width,
            selected_index: start_index,
            need_redraw: true,
        }
    }

    pub fn set_pressed(&mut self, state: bool) {
        let element = &mut self.elements[self.selected_index];
        if let MenuElement::Button { is_pressed, .. } | MenuElement::ButtonOption { is_pressed, .. } = element {
            *is_pressed = state
        }
    }

    pub fn with_element(mut self, element: MenuElement) -> Self {
        self.elements.push(element);
        self
    }

    pub fn add_element(&mut self, element: MenuElement) {
        self.elements.push(element);
    }

    pub fn finish_buttons_handling(&mut self) {
        for element in self.get_elements_mut() {
            if let MenuElement::Button {
                // Disable all buttons
                is_pressed,
                ..
            } = element
            {
                *is_pressed = false;
            }
        }
    }

    pub fn cursor_down(&mut self) {
        if let MenuElement::Button { is_pressed, .. } = &mut self.elements[self.selected_index] {
            *is_pressed = false
        }

        // Check if we are at the bottom, go to the top
        if self.selected_index == self.elements.len() - 1 {
            self.selected_index = 0;
        } else {
            self.selected_index += 1;
        }

        // Counter to avoid infinite loop
        let mut counter = 0;

        // Iterate unless the element is not a Label or we made a complete loop
        while matches!(
            self.elements[self.selected_index],
            MenuElement::Label { .. } | MenuElement::Void { .. } | MenuElement::ButtonOption { .. }
        ) && counter != self.elements.len()
        {
            // If we get to the bottom, we go at the top of the elements
            if self.selected_index == self.elements.len() - 1 {
                self.selected_index = 0;
            } else {
                self.selected_index += 1;
            }
            counter += 1;
        }

        self.need_redraw = true;
    }

    pub fn cursor_up(&mut self) {
        if let MenuElement::Button { is_pressed, .. } = &mut self.elements[self.selected_index] {
            *is_pressed = false
        }

        // Check if we are at the top, go to the bottom
        if self.selected_index == 0 {
            self.selected_index = self.elements.len() - 1;
        } else {
            self.selected_index -= 1;
        }

        // Counter to avoid infinite loop
        let mut counter = 0;

        // Iterate unless the element is not a Label or we made a complete loop
        while matches!(
            self.elements[self.selected_index],
            MenuElement::Label { .. } | MenuElement::Void { .. } | MenuElement::ButtonOption { .. }
        ) && counter != self.elements.len()
        {
            // If we reach the top, go back to the bottom
            if self.selected_index == 0 {
                self.selected_index = self.elements.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            counter += 1;
        }

        self.need_redraw = true;
    }

    pub fn cursor_right(&mut self) {
        if let MenuElement::Slider { // If we have a cursor
            value, step_size, ..
        } = &mut self.elements[self.selected_index]
        {
            *value += *step_size;
            if *value > 1. {
                *value = 1.;
            }
            self.need_redraw = true;
        } else if matches!(self.elements[self.selected_index], MenuElement::Button { .. }) // If the element after the button is a button option
            && self.selected_index < self.elements.len() - 1
            && matches!(self.elements[self.selected_index + 1], MenuElement::ButtonOption { .. })
        {
            self.selected_index += 1;
            self.need_redraw = true;
        }
    }

    pub fn cursor_left(&mut self) {
        if let MenuElement::Slider {
            value, step_size, ..
        } = &mut self.elements[self.selected_index]
        {
            *value -= *step_size;
            if *value < 0. {
                *value = 0.;
            }
            self.need_redraw = true;
        } else if matches!(self.elements[self.selected_index], MenuElement::ButtonOption { .. }) // If the element after the button is a button option
            && self.selected_index > 0
            && matches!(self.elements[self.selected_index - 1], MenuElement::Button { .. })
        {
            self.selected_index -= 1;
            self.need_redraw = true;
        }
    }

    pub fn get_elements(&self) -> &Vec<MenuElement> {
        &self.elements
    }
    pub fn get_elements_mut(&mut self) -> &mut Vec<MenuElement> {
        &mut self.elements
    }
}
