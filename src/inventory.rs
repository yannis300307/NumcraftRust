use core::{mem, usize};

#[cfg(target_os = "none")]
use alloc::vec::Vec;

use crate::{
    constants::ItemType,
    eadk::input::{Key, KeyboardState},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    item_type: ItemType,
    amount: u8,
}

impl ItemStack {
    pub const fn void() -> Self {
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
        }
    }

    pub fn move_item_in_other_inventory(&mut self, other_inventory: &mut Inventory, start_slot: usize, end_slot: usize, selected_amount_or_none: Option<usize>) {
        todo!()
    }

    pub fn move_item(
        &mut self,
        start_slot: usize,
        end_slot: usize,
        selected_amount_or_none: Option<usize>,
    ) {
        if start_slot == end_slot {
            return;
        }

        let start_slot_itemstack = self.get_ref_to_slot(start_slot).unwrap().clone();
        let end_slot_itemstack = self.get_ref_to_slot(end_slot).unwrap().clone();

        let start_max_stack_amount =
            start_slot_itemstack.get_item_type().get_max_stack_amount() as usize;
        let end_max_stack_amount =
            end_slot_itemstack.get_item_type().get_max_stack_amount() as usize;

        let selected_amount = if let Some(amount) = selected_amount_or_none {
            amount
        } else {
            start_slot_itemstack.get_amount() as usize
        };

        if start_slot_itemstack.get_item_type() == end_slot_itemstack.get_item_type()
            && start_slot_itemstack.amount as usize != start_max_stack_amount
            && end_slot_itemstack.amount as usize != end_max_stack_amount
        {
            let total_amount = end_slot_itemstack.amount as usize + selected_amount;

            if total_amount < start_max_stack_amount {
                if selected_amount == start_slot_itemstack.amount as usize {
                    self.replace_slot_item_stack(start_slot, ItemStack::void());
                    self.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            end_slot_itemstack.amount + selected_amount as u8,
                        ),
                    );
                } else {
                    self.replace_slot_item_stack(
                        start_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            start_slot_itemstack.amount - selected_amount as u8,
                        ),
                    );
                    self.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            end_slot_itemstack.amount + selected_amount as u8,
                        ),
                    );
                }
            } else if total_amount == start_max_stack_amount {
                self.replace_slot_item_stack(start_slot, ItemStack::void());
                self.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(end_slot_itemstack.item_type, start_max_stack_amount as u8),
                );
            } else {
                self.replace_slot_item_stack(
                    start_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        (total_amount - start_max_stack_amount) as u8,
                    ),
                );
                self.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(end_slot_itemstack.item_type, end_max_stack_amount as u8),
                );
            }
        } else {
            if start_slot_itemstack.item_type != ItemType::Air
                && end_slot_itemstack.item_type == ItemType::Air
                && selected_amount != start_slot_itemstack.get_amount() as usize
            {
                self.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(start_slot_itemstack.item_type, selected_amount as u8),
                );
                self.replace_slot_item_stack(
                    start_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        start_slot_itemstack.get_amount() - selected_amount as u8,
                    ),
                );
            } else {
                self.swap_slots(start_slot, end_slot);
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
