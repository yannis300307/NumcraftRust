#[cfg(target_os = "none")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use nalgebra::Vector2;

use crate::{eadk::input::Key, input_manager::InputManager};

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

    /// An additional button that can be added at the left of an existing button. Must be placed after a Button.
    ButtonOption {
        text: String,
        is_pressed: bool,
        id: usize,
    },

    Entry {
        placeholder_text: String,
        value: String,
        allow_margin: bool,
        max_len: u8,
        digits_only: bool,
        id: usize,
    },
}

#[allow(dead_code)]
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
    alpha_active: bool,
    shift_active: bool,
}

impl Menu {
    pub fn check_inputs(&mut self, input_manager: &InputManager) {
        if input_manager.is_just_pressed(Key::Down) {
            self.cursor_down();
        }
        if input_manager.is_just_pressed(Key::Up) {
            self.cursor_up();
        }
        if input_manager.is_just_pressed(Key::Right) {
            self.cursor_right();
        }
        if input_manager.is_just_pressed(Key::Left) {
            self.cursor_left();
        }

        if input_manager.is_just_pressed(Key::Ok) {
            self.set_pressed(true);
        }

        if input_manager.is_just_pressed(Key::Alpha) {
            self.alpha_active = !self.alpha_active;
        }

        if input_manager.is_just_pressed(Key::Shift) {
            self.shift_active = !self.shift_active;
        }

        if let MenuElement::Entry {
            value,
            max_len,
            digits_only,
            ..
        } = &mut self.elements[self.selected_index]
        {
            if input_manager.is_impulsed_key(Key::Backspace) && value.len() > 0 {
                value.truncate(value.len() - 1);
                self.need_redraw = true;
            }
            for key in [
                Key::Xnt,
                Key::Var,
                Key::Toolbox,
                Key::Backspace,
                Key::Exp,
                Key::Ln,
                Key::Log,
                Key::Imaginary,
                Key::Comma,
                Key::Power,
                Key::Sine,
                Key::Cosine,
                Key::Tangent,
                Key::Pi,
                Key::Sqrt,
                Key::Square,
                Key::Seven,
                Key::Eight,
                Key::Nine,
                Key::LeftParenthesis,
                Key::RightParenthesis,
                Key::Four,
                Key::Five,
                Key::Six,
                Key::Multiplication,
                Key::Division,
                Key::One,
                Key::Two,
                Key::Three,
                Key::Plus,
                Key::Minus,
                Key::Zero,
                Key::Dot,
            ] {
                if input_manager.is_impulsed_key(key) && value.len() < *max_len as usize {
                    if self.alpha_active && !*digits_only {
                        let mut letter = match key {
                            Key::Exp => "a",
                            Key::Ln => "b",
                            Key::Log => "c",
                            Key::Imaginary => "d",
                            Key::Comma => "e",
                            Key::Power => "f",
                            Key::Sine => "g",
                            Key::Cosine => "h",
                            Key::Tangent => "i",
                            Key::Pi => "j",
                            Key::Sqrt => "k",
                            Key::Square => "l",
                            Key::Seven => "m",
                            Key::Eight => "n",
                            Key::Nine => "o",
                            Key::LeftParenthesis => "p",
                            Key::RightParenthesis => "q",
                            Key::Four => "r",
                            Key::Five => "s",
                            Key::Six => "t",
                            Key::Multiplication => "u",
                            Key::Division => "v",
                            Key::One => "w",
                            Key::Two => "x",
                            Key::Three => "y",
                            Key::Plus => "z",
                            Key::Minus => " ",
                            Key::Zero => "?",
                            Key::Dot => "!",
                            _ => "",
                        }
                        .to_string();

                        if self.shift_active {
                            letter = letter.to_uppercase();
                        }

                        value.push_str(&letter);
                        self.need_redraw = true;
                    } else {
                        let letter = match key {
                            Key::One => "1",
                            Key::Two => "2",
                            Key::Three => "3",
                            Key::Four => "4",
                            Key::Five => "5",
                            Key::Six => "6",
                            Key::Seven => "7",
                            Key::Eight => "8",
                            Key::Nine => "9",
                            Key::Zero => "0",
                            _ => "",
                        };
                        value.push_str(&letter);
                        self.need_redraw = true;
                    }
                }
            }
        }
    }
    pub fn new(pos: Vector2<usize>, width: usize, start_index: usize) -> Self {
        Menu {
            elements: Vec::new(),
            pos,
            width,
            selected_index: start_index,
            need_redraw: true,
            alpha_active: true,
            shift_active: false,
        }
    }

    pub fn set_pressed(&mut self, state: bool) {
        let element = &mut self.elements[self.selected_index];
        if let MenuElement::Button { is_pressed, .. }
        | MenuElement::ButtonOption { is_pressed, .. } = element
        {
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
        let mut changed = false;
        for element in self.get_elements_mut() {
            if let MenuElement::Button {
                // Disable all buttons
                is_pressed,
                ..
            } = element
            {
                if *is_pressed {
                    changed = true;
                }

                *is_pressed = false;
            }
        }
        if changed {
            self.need_redraw = true;
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
        if let MenuElement::Slider {
            // If we have a cursor
            value,
            step_size,
            ..
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
