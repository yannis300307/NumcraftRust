use core::{mem, usize};

use alloc::vec::{self, Vec};

use crate::{
    constants::ItemType,
    eadk::input::{Key, KeyboardState},
};

pub struct ItemStack {
    item_type: ItemType,
    amount: u8,
}

impl ItemStack {
    pub fn void() -> Self {
        ItemStack {
            item_type: ItemType::Air,
            amount: 0,
        }
    }
}

pub struct Inventory {
    slots: Vec<ItemStack>,
    pub modified: bool,
    selected_slot: Option<usize>,
}

/// A generic inventory. Can be the player inventory, a chest inventory, etc... All operations works by swaping items to avoid duplication.
impl Inventory {
    pub fn new(size: usize) -> Self {
        let mut slots = Vec::with_capacity(size);
        for _ in 0..size {
            slots.push(ItemStack::void());
        }
        Inventory {
            slots: slots,
            modified: true,
            selected_slot: Some(9),
        }
    }

    pub fn get_selected_slot_index(&self) -> Option<usize> {
        self.selected_slot
    }

    pub fn update(&mut self, just_pressed_keyboard: KeyboardState) {
        if let Some(selected_slot) = &mut self.selected_slot {
            if just_pressed_keyboard.key_down(Key::Right) {
                if *selected_slot == self.slots.len() - 1
                {
                    *selected_slot = 0;
                } else {
                    *selected_slot += 1;
                }
                self.modified = true;
            }

            if just_pressed_keyboard.key_down(Key::Left) {
                if *selected_slot == 0 {
                    *selected_slot = self.slots.len() - 1;
                } else {
                    *selected_slot -= 1;
                }
                self.modified = true;
            }

            if just_pressed_keyboard.key_down(Key::Up) {
                if *selected_slot < 6 {
                    *selected_slot = 0;
                } else {
                    *selected_slot -= 6;
                }
                self.modified = true;
            }
            if just_pressed_keyboard.key_down(Key::Down) {
                if *selected_slot >= self.slots.len()-6 {
                    *selected_slot = self.slots.len() - 1;
                } else {
                    *selected_slot += 6;
                }
                self.modified = true;
            }
        }
    }

    pub fn swap_item_stack(&mut self, slot_index: usize, other: &mut ItemStack) -> Option<()> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &mut self.slots[slot_index];

            mem::swap(other, item_stack);
            self.modified = true;

            Some(())
        }
    }

    pub fn get_ref_to_slot(&mut self, slot_index: usize) -> Option<&mut ItemStack> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &mut self.slots[slot_index];

            Some(item_stack)
        }
    }

    pub fn swap_slots(&mut self, first: usize, second: usize) -> Option<()> {
        if first >= self.slots.len() || second >= self.slots.len() {
            None
        } else {
            self.slots.swap(first, second);
            self.modified = true;

            Some(())
        }
    }

    pub fn replace_slot_item_stack(
        &mut self,
        slot_index: usize,
        item_stack: ItemStack,
    ) -> Option<()> {
        if slot_index >= self.slots.len() {
            None
        } else {
            self.slots[slot_index] = item_stack;
            self.modified = true;

            Some(())
        }
    }

    pub fn get_all_slots(&self) -> &Vec<ItemStack> {
        &self.slots
    }
}
