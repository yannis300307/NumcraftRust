use nalgebra::Vector2;

use crate::{
    constants::ItemType,
    eadk::input,
    input_manager::InputManager,
    inventory::{Inventory, ItemStack},
};

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

#[allow(unused)]
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
    Arrow {
        filling: f32,
    },
    OneWayItemSlot {
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

    pub fn create_one_way_slot_slot(inventory_id: usize, inventory_slot_index: usize) -> Self {
        Self::OneWayItemSlot {
            item_stack: ItemStack::void(),
            inventory_id,
            inventory_slot_index,
        }
    }
}

pub struct AnchorContainer {
    pub element: GameUIElements,

    pub pos: Vector2<u16>,
    pub id: usize,

    pub neighbors: ContainerNeighbors,
}

pub struct ContainerNeighbors {
    pub up_id: Option<usize>,
    pub down_id: Option<usize>,
    pub left_id: Option<usize>,
    pub right_id: Option<usize>,
}

impl Default for ContainerNeighbors {
    fn default() -> Self {
        ContainerNeighbors {
            up_id: None,
            down_id: None,
            left_id: None,
            right_id: None,
        }
    }
}

#[allow(unused)]
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
    pub need_redraw: bool,
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
            need_redraw: true,
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
        let container = AnchorContainer {
            element,
            pos,
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
        let container = AnchorContainer {
            element,
            pos,
            id,
            neighbors,
        };

        self.elements.push(container);
    }

    pub fn get_element_with_id(&self, id: usize) -> Option<&AnchorContainer> {
        self.elements.iter().find(|&elem| elem.id == id)
    }

    pub fn get_element_with_id_mut(&mut self, id: usize) -> Option<&mut AnchorContainer> {
        self.elements.iter_mut().find(|elem| elem.id == id)
    }

    fn move_cursor_if_possible(&mut self, input_manager: &InputManager, key: input::Key) {
        if !input_manager.is_impulsed_key(key) {
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
                self.ask_redraw();
            }
        }
    }

    pub fn ask_redraw(&mut self) {
        self.need_redraw = true;
    }

    pub fn update(
        &mut self,
        input_manager: &InputManager,
        inventories: &mut [&mut Inventory],
    ) -> bool {
        if self.is_selecting_amount
            && let Some(amount) = &mut self.selected_amount
        {
            if input_manager.is_impulsed_key(input::Key::Right) {
                *amount += 1;
                self.ask_redraw();
            } else if input_manager.is_impulsed_key(input::Key::Left) {
                *amount -= 1;
                self.ask_redraw();
            } else if input_manager.is_impulsed_key(input::Key::Up) {
                *amount += 4;
                self.ask_redraw();
            } else if input_manager.is_impulsed_key(input::Key::Down) {
                if *amount > 4 {
                    *amount -= 4;
                } else {
                    *amount = 1;
                }
                self.ask_redraw();
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
                self.ask_redraw();
            } else if self
                .selected_amount
                .is_some_and(|v| v > item_stack.get_amount() as usize)
            {
                self.selected_amount = Some(item_stack.get_amount() as usize);
                self.ask_redraw();
            }
        }

        // Mainly for crafting
        for inventory in inventories.iter() {
            if inventory.modified {
                self.update_slots(inventories);
            }
        }

        for inventory in &mut *inventories {
            inventory.modified = false;
        }

        if input_manager.is_just_pressed(input::Key::Ok) {
            // If the selected element is the same as the one at the position of the cursor and that the user is not currently selecting an item amount
            if self
                .selected_id
                .is_some_and(|index| index == self.cursor_id)
                && !self.is_selecting_amount
            {
                // If the selected element is an ItemSlot
                if let Some(element) = self.get_element_with_id(self.cursor_id)
                    && let GameUIElements::ItemSlot { item_stack, .. } = &element.element
                {
                    self.selected_amount = Some(item_stack.get_amount() as usize / 2);
                    self.is_selecting_amount = true;
                    self.ask_redraw();
                }
            } else {
                // If the user was selecting an amount, we disable the amount selection
                if self.is_selecting_amount {
                    self.is_selecting_amount = false;
                    self.ask_redraw();

                // If an element is selected but it is not the one at the position of the cursor and that both are ItemSlots
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
                    // If the 2 item slots are in the same inventories
                    if start_inventory_id == end_inventory_id {
                        inventories[start_inventory_id].move_item(
                            start_inventory_slot_index,
                            end_inventory_slot_index,
                            self.selected_amount,
                        );
                        self.ask_redraw();
                    } else {
                        // Trick to have 2 mutable references to elements in the slice
                        let (first, second) = if start_inventory_id < end_inventory_id {
                            let (first, second) = inventories.split_at_mut(end_inventory_id);
                            (&mut first[start_inventory_id], &mut second[0])
                        } else {
                            let (first, second) = inventories.split_at_mut(start_inventory_id);
                            (&mut second[0], &mut first[end_inventory_id])
                        };

                        first.move_item_in_other_inventory(
                            second,
                            start_inventory_slot_index,
                            end_inventory_slot_index,
                            self.selected_amount,
                        );
                        self.ask_redraw();
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
                    self.ask_redraw();
                } else if let Some(element) = self.get_element_with_id(self.cursor_id)
                    && let GameUIElements::OneWayItemSlot {
                        item_stack,
                        inventory_id,
                        inventory_slot_index,
                        ..
                    } = &element.element
                    && item_stack.get_item_type() != ItemType::Air
                {
                    // Here, I concider the inventory at index 0 as the player's inventory or the main inventory.
                    if *inventory_id == 1 {
                        inventories[0].add_item_stack(item_stack.clone());
                        inventories[1]
                            .replace_slot_item_stack(*inventory_slot_index, ItemStack::void());
                    }
                }
            }
        }
        if input_manager.is_just_pressed(input::Key::Back) {
            if self.selected_id.is_some() {
                self.selected_id = None;
                self.selected_amount = None;
                self.is_selecting_amount = false;
            } else {
                return false;
            }
        }

        if input_manager.is_just_pressed(input::Key::Var) {
            return false;
        }
        
        true
    }

    /// Sync the GameUI slots elements with the matching inventories slots
    fn update_slots(&mut self, inventories: &[&mut Inventory]) {
        for element in &mut self.elements {
            if let GameUIElements::ItemSlot {
                item_stack,
                inventory_id,
                inventory_slot_index,
                ..
            }
            | GameUIElements::OneWayItemSlot {
                item_stack,
                inventory_id,
                inventory_slot_index,
                ..
            } = &mut element.element
            {
                let new_item_stack = inventories[*inventory_id]
                    .get_ref_to_slot(*inventory_slot_index)
                    .unwrap()
                    .clone();
                *item_stack = new_item_stack;
            }
        }
        self.ask_redraw();
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

                let slot_pos = Vector2::new(32 * x, 32 * y) + pos;
                self.add_element(slot, slot_pos, last_element_id, neighbors);

                last_inventory_index += 1;
                last_element_id += 1;
            }
        }

        self
    }
}
