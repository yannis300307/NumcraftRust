use nalgebra::Vector2;

use crate::{eadk::input, input_manager::InputManager, inventory::ItemStack};

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
        selected_amount: usize,
    },
}

struct AnchorContainer {
    element: GameUIElements,

    pos: Vector2<u16>,
    selectable: bool,
    id: usize,

    neighbors: ContainerNeighbors,
}

pub struct ContainerNeighbors {
    up_id: Option<usize>,
    down_id: Option<usize>,
    left_id: Option<usize>,
    right_id: Option<usize>,
}

pub struct GameUI {
    elements: Vec<AnchorContainer>,
    pub selected_index: usize,
    pub need_redraw: bool,
    pub blur_background: bool,
}

impl GameUI {
    fn new(blur_background: bool) -> Self {
        GameUI {
            elements: Vec::new(),
            selected_index: 0,
            need_redraw: true,
            blur_background,
        }
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
        if !input_manager.is_just_pressed(input::Key::Right) {
            return;
        }

        let elem_or_none = self.get_element_with_id(self.selected_index);
        if let Some(elem) = elem_or_none && elem.neighbors.right_id.is_some() {
            let neighbor = match key {
                input::Key::Right => elem.neighbors.right_id,
                input::Key::Left => elem.neighbors.right_id,
                input::Key::Up => elem.neighbors.right_id,
                input::Key::Down => elem.neighbors.right_id,
                _ => None,
            };

            if let Some(neighbor_id) = neighbor {
                self.selected_index = neighbor_id
            }
        }
    }

    pub fn update(&mut self, input_manager: &InputManager) {
        self.move_cursor_if_possible(input_manager, input::Key::Right);
        self.move_cursor_if_possible(input_manager, input::Key::Left);
        self.move_cursor_if_possible(input_manager, input::Key::Up);
        self.move_cursor_if_possible(input_manager, input::Key::Down);
    }
}
