use nalgebra::Vector2;

use crate::{
    constants::ItemType,
    eadk::input,
    input_manager::InputManager,
    inventory::{Inventory, ItemStack},
};

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
        inventory_id: usize,
        inventory_slot_index: usize,
    },
}

impl GameUIElements {
    pub fn create_slot(inventory_id: usize, inventory_slot_index: usize) -> Self {
        Self::ItemSlot {
            item_stack: ItemStack::void(),
            inventory_id,
            inventory_slot_index,
        }
    }
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

pub enum NeighborDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct GameUI {
    elements: Vec<AnchorContainer>,
    pub cursor_id: usize,
    pub selected_id: Option<usize>,
    pub selected_amount: Option<usize>,
    pub need_complete_redraw: bool,
    pub blur_background: bool,

    pub is_selecting_amount: bool,
}

impl GameUI {
    pub fn new(blur_background: bool) -> Self {
        let ui = GameUI {
            elements: Vec::new(),
            cursor_id: 0,
            selected_id: None,
            selected_amount: None,
            need_complete_redraw: true,
            blur_background,
            is_selecting_amount: false,
        };
        ui
    }

    /// Sync the the slots with the inventories at the end of the game ui creation pipeline
    pub fn sync(mut self, inventories: &[&mut Inventory]) -> Self {
        self.update_slots(inventories);
        self
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

    pub fn with_links(mut self, links: &[(usize, usize, NeighborDirection)]) -> Self {
        for id in links {
            let elem1 = self.get_element_with_id_mut(id.0).expect("Invalid ID1.");

            match id.2 {
                NeighborDirection::Top => elem1.neighbors.up_id = Some(id.1),
                NeighborDirection::Bottom => elem1.neighbors.down_id = Some(id.1),
                NeighborDirection::Left => elem1.neighbors.left_id = Some(id.1),
                NeighborDirection::Right => elem1.neighbors.right_id = Some(id.1),
            }

            let elem2 = self.get_element_with_id_mut(id.1).expect("Invalid ID2.");

             match id.2 {
                NeighborDirection::Top => elem2.neighbors.down_id = Some(id.0),
                NeighborDirection::Bottom => elem2.neighbors.up_id = Some(id.0),
                NeighborDirection::Left => elem2.neighbors.right_id = Some(id.0),
                NeighborDirection::Right => elem2.neighbors.left_id = Some(id.0),
            }
        }
        self
    }

    pub fn add_element(
        &mut self,
        element: GameUIElements,
        pos: Vector2<u16>,
        id: usize,
        neighbors: ContainerNeighbors,
    ) {
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
    }

    fn get_element_with_id(&self, id: usize) -> Option<&AnchorContainer> {
        self.elements.iter().find(|&elem| elem.id == id)
    }

    fn get_element_with_id_mut(&mut self, id: usize) -> Option<&mut AnchorContainer> {
        self.elements.iter_mut().find(|elem| elem.id == id)
    }

    fn move_cursor_if_possible(&mut self, input_manager: &InputManager, key: input::Key) {
        if !input_manager.is_just_pressed(key) {
            return;
        }

        let elem_or_none = self.get_element_with_id(self.cursor_id);
        if let Some(elem) = elem_or_none {
            let neighbor = match key {
                input::Key::Right => elem.neighbors.right_id,
                input::Key::Left => elem.neighbors.left_id,
                input::Key::Up => elem.neighbors.up_id,
                input::Key::Down => elem.neighbors.down_id,
                _ => None,
            };

            if let Some(neighbor_id) = neighbor {
                self.cursor_id = neighbor_id;
            }
        }
    }

    pub fn update(&mut self, input_manager: &InputManager, inventories: &mut [&mut Inventory]) {
        if self.is_selecting_amount
            && let Some(amount) = &mut self.selected_amount
        {
            if input_manager.is_just_pressed(input::Key::Right) {
                *amount += 1;
            } else if input_manager.is_just_pressed(input::Key::Left) {
                *amount -= 1;
            } else if input_manager.is_just_pressed(input::Key::Up) {
                *amount += 4;
            } else if input_manager.is_just_pressed(input::Key::Down) {
                if *amount > 4 {
                    *amount -= 4;
                } else {
                    *amount = 1;
                }
            }
        } else {
            self.move_cursor_if_possible(input_manager, input::Key::Right);
            self.move_cursor_if_possible(input_manager, input::Key::Left);
            self.move_cursor_if_possible(input_manager, input::Key::Up);
            self.move_cursor_if_possible(input_manager, input::Key::Down);
        }

        // Check for stack overflow
        if let Some(index) = self.selected_id
            && let Some(element) = self.get_element_with_id(index)
            && let GameUIElements::ItemSlot { item_stack, .. } = &element.element
        {
            if self.selected_amount.is_some_and(|v| v < 1) {
                self.selected_amount = Some(1);
            } else if self
                .selected_amount
                .is_some_and(|v| v > item_stack.get_amount() as usize)
            {
                self.selected_amount = Some(item_stack.get_amount() as usize);
            }
        }

        if input_manager.is_just_pressed(input::Key::Ok) {
            if self
                .selected_id
                .is_some_and(|index| index == self.cursor_id)
                && !self.is_selecting_amount
            {
                if let Some(element) = self.get_element_with_id(self.cursor_id)
                    && let GameUIElements::ItemSlot { item_stack, .. } = &element.element
                {
                    self.selected_amount = Some(item_stack.get_amount() as usize / 2);
                    self.is_selecting_amount = true;
                }
            } else {
                if self.is_selecting_amount {
                    self.is_selecting_amount = false;
                } else if let Some(selected_id) = self.selected_id
                    && selected_id != self.cursor_id
                    && let Some(start_elem) = self.get_element_with_id(selected_id)
                    && let Some(end_elem) = self.get_element_with_id(self.cursor_id)
                    && let GameUIElements::ItemSlot {
                        inventory_id: start_inventory_id,
                        inventory_slot_index: start_inventory_slot_index,
                        ..
                    } = start_elem.element
                    && let GameUIElements::ItemSlot {
                        inventory_id: end_inventory_id,
                        inventory_slot_index: end_inventory_slot_index,
                        ..
                    } = end_elem.element
                {
                    if start_inventory_id == end_inventory_id {
                        inventories[start_inventory_id].move_item(
                            start_inventory_slot_index,
                            end_inventory_slot_index,
                            self.selected_amount,
                        );
                    } else {
                        todo!();
                    }

                    self.selected_id = None;
                    self.selected_amount = None;
                    self.is_selecting_amount = false;

                    self.update_slots(inventories);
                } else if let Some(element) = self.get_element_with_id(self.cursor_id)
                    && let GameUIElements::ItemSlot { item_stack, .. } = &element.element
                    && item_stack.get_item_type() != ItemType::Air
                {
                    self.selected_id = Some(self.cursor_id);
                }
            }
        }
        if input_manager.is_just_pressed(input::Key::Back) {
            self.selected_id = None;
            self.selected_amount = None;
            self.is_selecting_amount = false;
        }
    }

    /// Sync the GameUI slots elements with the matching inventories slots
    fn update_slots(&mut self, inventories: &[&mut Inventory]) {
        for element in &mut self.elements {
            if let GameUIElements::ItemSlot {
                item_stack,
                inventory_id,
                inventory_slot_index,
            } = &mut element.element
            {
                *item_stack = inventories[*inventory_id]
                    .get_ref_to_slot(*inventory_slot_index)
                    .unwrap()
                    .clone();
            }
        }
    }

    pub fn with_slot_grid(
        mut self,
        pos: Vector2<u16>,
        width: u16,
        height: u16,
        inventory_id: usize,
        start_id: usize,
        start_inventory_index: usize,
    ) -> Self {
        let mut last_inventory_index = start_inventory_index;
        let mut last_element_id = start_id;

        for y in 0..height {
            for x in 0..width {
                let slot = GameUIElements::create_slot(inventory_id, last_inventory_index);
                let neighbors = ContainerNeighbors {
                    up_id: if y != 0 {
                        Some(last_element_id - width as usize)
                    } else {
                        None
                    },
                    down_id: if y != height - 1 {
                        Some(last_element_id + width as usize)
                    } else {
                        None
                    },
                    left_id: if x != 0 {
                        Some(last_element_id - 1)
                    } else {
                        None
                    },
                    right_id: if x != width - 1 {
                        Some(last_element_id + 1)
                    } else {
                        None
                    },
                };

                let slot_pos = Vector2::new(48 * x, 48 * y) + pos;
                self.add_element(slot, slot_pos, last_element_id, neighbors);

                last_inventory_index += 1;
                last_element_id += 1;
            }
        }

        self
    }
}
