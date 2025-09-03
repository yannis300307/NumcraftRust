use nalgebra::Vector2;

use crate::{eadk::input, input_manager::InputManager, inventory::ItemStack};

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

pub enum GameUIElements {
    /// A simple button
    Button {
        text: String,
        is_pressed: bool,
    },
    Label {
        text: String,
    },
    ItemSlot {
        item_stack: ItemStack,
    },
}

pub struct AnchorContainer {
    pub element: GameUIElements,

    pub pos: Vector2<u16>,
    pub selectable: bool,
    pub id: usize,

    pub neighbors: ContainerNeighbors,
}

pub struct ContainerNeighbors {
    pub up_id: Option<usize>,
    pub down_id: Option<usize>,
    pub left_id: Option<usize>,
    pub right_id: Option<usize>,
}

impl ContainerNeighbors {
    pub fn new(
        up_id: Option<usize>,
        down_id: Option<usize>,
        left_id: Option<usize>,
        right_id: Option<usize>,
    ) -> Self {
        ContainerNeighbors {
            up_id,
            down_id,
            left_id,
            right_id,
        }
    }
}

pub struct GameUI {
    elements: Vec<AnchorContainer>,
    pub cursor_index: usize,
    pub selected_index: Option<usize>,
    pub selected_amount: Option<usize>,
    pub need_complete_redraw: bool,
    pub blur_background: bool,
}

impl GameUI {
    pub fn new(blur_background: bool) -> Self {
        GameUI {
            elements: Vec::new(),
            cursor_index: 0,
            selected_index: None,
            selected_amount: None,
            need_complete_redraw: true,
            blur_background,
        }
    }

    pub fn get_elements(&self) -> &Vec<AnchorContainer> {
        &self.elements
    }

    pub fn with_element(
        mut self,
        element: GameUIElements,
        pos: Vector2<u16>,
        id: usize,
        neighbors: ContainerNeighbors,
    ) -> Self {
        let selectable = match element {
            GameUIElements::Button { .. } => true,
            GameUIElements::ItemSlot { .. } => true,
            GameUIElements::Label { .. } => false,
        };

        let container = AnchorContainer {
            element,
            pos,
            selectable,
            id,
            neighbors,
        };

        self.elements.push(container);

        self
    }

    fn get_element_with_id(&self, id: usize) -> Option<&AnchorContainer> {
        self.elements.iter().find(|&elem| elem.id == id)
    }

    fn move_cursor_if_possible(&mut self, input_manager: &InputManager, key: input::Key) {
        if !input_manager.is_just_pressed(key) {
            return;
        }

        let elem_or_none = self.get_element_with_id(self.cursor_index);
        if let Some(elem) = elem_or_none {
            let neighbor = match key {
                input::Key::Right => elem.neighbors.right_id,
                input::Key::Left => elem.neighbors.left_id,
                input::Key::Up => elem.neighbors.up_id,
                input::Key::Down => elem.neighbors.down_id,
                _ => None,
            };

            if let Some(neighbor_id) = neighbor {
                self.cursor_index = neighbor_id;
            }
        }
    }

    pub fn update(&mut self, input_manager: &InputManager) {
        self.move_cursor_if_possible(input_manager, input::Key::Right);
        self.move_cursor_if_possible(input_manager, input::Key::Left);
        self.move_cursor_if_possible(input_manager, input::Key::Up);
        self.move_cursor_if_possible(input_manager, input::Key::Down);

        if input_manager.is_just_pressed(input::Key::Ok) {
            self.selected_index = Some(self.cursor_index);
        }
        if input_manager.is_just_pressed(input::Key::Back) {
            self.selected_index = None;
        }
    }
}
