use core::{mem, usize};

#[cfg(target_os = "none")]
use alloc::vec::Vec;

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

    pub fn new(item_type: ItemType, amount: u8) -> Self {
        ItemStack { item_type, amount }
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type
    }
    pub fn get_amount(&self) -> u8 {
        self.amount
    }
}

pub struct Inventory {
    slots: Vec<ItemStack>,
    pub modified: bool,
    cursor_slot: Option<usize>,
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
            cursor_slot: Some(0),
            selected_slot: None,
        }
    }

    pub fn get_cursor_slot_index(&self) -> Option<usize> {
        self.cursor_slot
    }

    pub fn get_selected_slot_index(&self) -> Option<usize> {
        self.selected_slot
    }

    fn move_item(&mut self, start_slot: usize, end_slot: usize) {
        if start_slot == end_slot {
            self.selected_slot = None;
            return;
        }

        let start_slot_itemstack = self.get_ref_to_slot(start_slot).unwrap();
        let end_slot_itemstack = self.get_ref_to_slot(end_slot).unwrap();

        let start_max_stack_amount =
            start_slot_itemstack.get_item_type().get_max_stack_amount() as usize;
        let end_max_stack_amount =
            end_slot_itemstack.get_item_type().get_max_stack_amount() as usize;

        if start_slot_itemstack.get_item_type() == end_slot_itemstack.get_item_type()
            && start_slot_itemstack.amount as usize != start_max_stack_amount
            && end_slot_itemstack.amount as usize != end_max_stack_amount
        {
            let item_type = start_slot_itemstack.get_item_type();
            let total_amount =
                end_slot_itemstack.amount as usize + start_slot_itemstack.amount as usize;
            let bigger_amount = total_amount.min(start_max_stack_amount);
            let remaining_amount = total_amount - bigger_amount;
            self.replace_slot_item_stack(end_slot, ItemStack::new(item_type, bigger_amount as u8));
            if remaining_amount > 0 {
                self.replace_slot_item_stack(
                    start_slot,
                    ItemStack::new(item_type, remaining_amount as u8),
                );
            } else {
                self.replace_slot_item_stack(start_slot, ItemStack::new(ItemType::Air, 0));
            }
        } else {
            self.swap_slots(start_slot, end_slot);
        }
        self.selected_slot = None;
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

    pub fn get_ref_to_slot_mut(&mut self, slot_index: usize) -> Option<&mut ItemStack> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &mut self.slots[slot_index];

            Some(item_stack)
        }
    }
    
    pub fn get_ref_to_slot(&self, slot_index: usize) -> Option<&ItemStack> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &self.slots[slot_index];

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
