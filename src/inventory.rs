use core::{mem, usize};

#[cfg(target_os = "none")]
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{constants::ItemType, entity::item};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    item_type: ItemType,
    amount: u8,
    pub(crate) creative_slot: bool,
}

impl ItemStack {
    pub const fn void() -> Self {
        ItemStack {
            item_type: ItemType::Air,
            amount: 0,
            creative_slot: false,
        }
    }

    pub const fn new(item_type: ItemType, amount: u8, creative_slot: bool) -> Self {
        ItemStack {
            item_type,
            amount,
            creative_slot,
        }
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type
    }
    pub fn get_amount(&self) -> u8 {
        if self.creative_slot {
            self.item_type.get_max_stack_amount()
        } else {
            self.amount
        }
    }

    pub fn clear(&mut self) {
        self.amount = 0;
        self.item_type = ItemType::Air;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    slots: Vec<ItemStack>,
    pub modified: bool,
}

/// A generic inventory. Can be the player inventory, a chest inventory, etc...
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

    pub fn fill(&mut self, item_stack: ItemStack) {
        for i in 0..self.slots.len() {
            self.slots[i] = item_stack;
        }
    }

    pub fn move_item_in_other_inventory(
        &mut self,
        other_inventory: &mut Inventory,
        start_slot: usize,
        end_slot: usize,
        selected_amount_or_none: Option<usize>,
    ) {
        let start_slot_itemstack = self.get_ref_to_slot(start_slot).unwrap().clone();
        let end_slot_itemstack = other_inventory.get_ref_to_slot(end_slot).unwrap().clone();

        let start_max_stack_amount =
            start_slot_itemstack.get_item_type().get_max_stack_amount() as usize;
        let end_max_stack_amount =
            end_slot_itemstack.get_item_type().get_max_stack_amount() as usize;

        let selected_amount = if let Some(amount) = selected_amount_or_none {
            amount
        } else {
            start_slot_itemstack.get_amount() as usize
        };

        if start_slot_itemstack.creative_slot && !end_slot_itemstack.creative_slot {
            if end_slot_itemstack.item_type == start_slot_itemstack.item_type {
                other_inventory.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        (end_slot_itemstack.amount as usize + selected_amount)
                            .min(start_max_stack_amount) as u8,
                        false,
                    ),
                );
            } else {
                other_inventory.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(start_slot_itemstack.item_type, selected_amount as u8, false),
                );
            }
        } else if !start_slot_itemstack.creative_slot && end_slot_itemstack.creative_slot {
            if selected_amount == start_slot_itemstack.get_amount() as usize {
                self.replace_slot_item_stack(start_slot, ItemStack::void());
            } else {
                self.replace_slot_item_stack(
                    start_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        start_slot_itemstack.amount - selected_amount as u8,
                        false,
                    ),
                );
            }
        } else if !start_slot_itemstack.creative_slot && !end_slot_itemstack.creative_slot {
            if start_slot_itemstack.get_item_type() == end_slot_itemstack.get_item_type()
                && start_slot_itemstack.amount as usize != start_max_stack_amount
                && end_slot_itemstack.amount as usize != end_max_stack_amount
            {
                let total_amount = end_slot_itemstack.amount as usize + selected_amount;

                if total_amount < start_max_stack_amount {
                    if selected_amount == start_slot_itemstack.amount as usize {
                        self.replace_slot_item_stack(start_slot, ItemStack::void());
                        other_inventory.replace_slot_item_stack(
                            end_slot,
                            ItemStack::new(
                                end_slot_itemstack.item_type,
                                end_slot_itemstack.amount + selected_amount as u8,
                                false,
                            ),
                        );
                    } else {
                        self.replace_slot_item_stack(
                            start_slot,
                            ItemStack::new(
                                start_slot_itemstack.item_type,
                                start_slot_itemstack.amount - selected_amount as u8,
                                false,
                            ),
                        );
                        other_inventory.replace_slot_item_stack(
                            end_slot,
                            ItemStack::new(
                                end_slot_itemstack.item_type,
                                end_slot_itemstack.amount + selected_amount as u8,
                                false,
                            ),
                        );
                    }
                } else if total_amount == start_max_stack_amount {
                    self.replace_slot_item_stack(start_slot, ItemStack::void());
                    other_inventory.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            start_max_stack_amount as u8,
                            false,
                        ),
                    );
                } else {
                    self.replace_slot_item_stack(
                        start_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            (total_amount - start_max_stack_amount) as u8,
                            false,
                        ),
                    );
                    other_inventory.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            end_max_stack_amount as u8,
                            false,
                        ),
                    );
                }
            } else {
                if start_slot_itemstack.item_type != ItemType::Air
                    && end_slot_itemstack.item_type == ItemType::Air
                    && selected_amount != start_slot_itemstack.get_amount() as usize
                {
                    other_inventory.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            selected_amount as u8,
                            false,
                        ),
                    );
                    self.replace_slot_item_stack(
                        start_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            start_slot_itemstack.get_amount() - selected_amount as u8,
                            false,
                        ),
                    );
                } else {
                    other_inventory.replace_slot_item_stack(end_slot, start_slot_itemstack);
                    self.replace_slot_item_stack(start_slot, end_slot_itemstack);
                }
            }
        }
    }

    pub fn take_one(&mut self, index: usize) -> Option<ItemType> {
        let slot = self.slots.get_mut(index)?;
        let item_type = slot.item_type;
        if !slot.creative_slot && slot.amount == 1 {
            slot.clear();
        } else if !slot.creative_slot && slot.amount > 0 {
            slot.amount -= 1;
        }
        Some(item_type)
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

        if start_slot_itemstack.creative_slot && !end_slot_itemstack.creative_slot {
            if end_slot_itemstack.item_type == start_slot_itemstack.item_type {
                self.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        (start_slot_itemstack.amount as usize + selected_amount)
                            .min(start_max_stack_amount) as u8,
                        false,
                    ),
                );
            } else {
                self.replace_slot_item_stack(
                    end_slot,
                    ItemStack::new(start_slot_itemstack.item_type, selected_amount as u8, false),
                );
            }
        } else if !start_slot_itemstack.creative_slot && end_slot_itemstack.creative_slot {
            if selected_amount == start_slot_itemstack.get_amount() as usize {
                self.replace_slot_item_stack(start_slot, ItemStack::void());
            } else {
                self.replace_slot_item_stack(
                    start_slot,
                    ItemStack::new(
                        start_slot_itemstack.item_type,
                        start_slot_itemstack.amount - selected_amount as u8,
                        false,
                    ),
                );
            }
        } else if !start_slot_itemstack.creative_slot && !end_slot_itemstack.creative_slot {
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
                                false,
                            ),
                        );
                    } else {
                        self.replace_slot_item_stack(
                            start_slot,
                            ItemStack::new(
                                start_slot_itemstack.item_type,
                                start_slot_itemstack.amount - selected_amount as u8,
                                false,
                            ),
                        );
                        self.replace_slot_item_stack(
                            end_slot,
                            ItemStack::new(
                                end_slot_itemstack.item_type,
                                end_slot_itemstack.amount + selected_amount as u8,
                                false,
                            ),
                        );
                    }
                } else if total_amount == start_max_stack_amount {
                    self.replace_slot_item_stack(start_slot, ItemStack::void());
                    self.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            start_max_stack_amount as u8,
                            false,
                        ),
                    );
                } else {
                    self.replace_slot_item_stack(
                        start_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            (total_amount - start_max_stack_amount) as u8,
                            false,
                        ),
                    );
                    self.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            end_slot_itemstack.item_type,
                            end_max_stack_amount as u8,
                            false,
                        ),
                    );
                }
            } else {
                if start_slot_itemstack.item_type != ItemType::Air
                    && end_slot_itemstack.item_type == ItemType::Air
                    && selected_amount != start_slot_itemstack.get_amount() as usize
                {
                    self.replace_slot_item_stack(
                        end_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            selected_amount as u8,
                            false,
                        ),
                    );
                    self.replace_slot_item_stack(
                        start_slot,
                        ItemStack::new(
                            start_slot_itemstack.item_type,
                            start_slot_itemstack.get_amount() - selected_amount as u8,
                            false,
                        ),
                    );
                } else {
                    self.swap_slots(start_slot, end_slot);
                }
            }
        }
    }

    pub fn swap_item_stack(&mut self, slot_index: usize, other: &mut ItemStack) -> Option<()> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &mut self.slots[slot_index];

            if other != item_stack {
                self.modified = true;
            }

            mem::swap(other, item_stack);

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
            if first != second {
                self.modified = true;
            }
            self.slots.swap(first, second);

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
            if self.slots[slot_index] != item_stack {
                self.modified = true;
            }

            self.slots[slot_index] = item_stack;

            Some(())
        }
    }

    pub fn get_all_slots(&self) -> &Vec<ItemStack> {
        &self.slots
    }

    /// Add an item stack to the inventory. Returns the number of remaining items.
    pub fn add_item_stack(&mut self, item_stack: ItemStack) -> u8 {
        let max_stack = item_stack.get_item_type().get_max_stack_amount();
        let mut amount = item_stack.amount as usize;

        // Check incomplete stacks
        for i in 0..self.slots.len() {
            if self.slots[i].get_item_type() == item_stack.get_item_type() {
                let total = self.slots[i].get_amount() as usize + amount;

                if total <= max_stack as usize {
                    self.replace_slot_item_stack(
                        i,
                        ItemStack::new(item_stack.get_item_type(), total as u8, false),
                    );
                    return 0;
                } else {
                    self.replace_slot_item_stack(
                        i,
                        ItemStack::new(item_stack.get_item_type(), max_stack, false),
                    );
                    amount = total - max_stack as usize;
                }
            }
            if amount == 0 {
                return 0;
            }
        }

        // Check for empty slots
        for i in 0..self.slots.len() {
            if self.slots[i].get_item_type() == ItemType::Air {
                if amount <= max_stack as usize {
                    self.replace_slot_item_stack(
                        i,
                        ItemStack::new(item_stack.get_item_type(), amount as u8, false),
                    );
                    return 0;
                } else {
                    self.replace_slot_item_stack(
                        i,
                        ItemStack::new(item_stack.get_item_type(), max_stack, false),
                    );
                    amount -= max_stack as usize;
                }
            }
        }

        return amount as u8;
    }

    pub fn get_item_stack_at_slot_index(&self, index: usize) -> Option<&ItemStack> {
        self.slots.get(index)
    }

    pub fn get_item_type_at_slot_index(&self, index: usize) -> Option<ItemType> {
        Some(self.get_item_stack_at_slot_index(index)?.get_item_type())
    }
}
