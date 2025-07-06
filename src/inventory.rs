use core::{mem, usize};

use alloc::vec::{self, Vec};

use crate::constants::ItemType;

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
    selected_slot: usize,
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
            selected_slot: 9,
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
