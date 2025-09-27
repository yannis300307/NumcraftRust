use crate::{input_manager::InputManager, inventory::ItemStack, player::Player};

pub struct Hud {
    slots: [ItemStack; 6],
    pub selected_slot: usize,
}

impl Hud {
    pub fn new() -> Self {
        Hud {
            slots: [const { ItemStack::void() }; 6],
            selected_slot: 0,
        }
    }
    pub fn update(&mut self, input_manager: &InputManager) {
        if input_manager.is_just_pressed(crate::eadk::input::Key::LeftParenthesis) {
            if self.selected_slot == 0 {
                self.selected_slot = 5
            } else {
                self.selected_slot -= 1
            }
        }
        if input_manager.is_just_pressed(crate::eadk::input::Key::RightParenthesis) {
            if self.selected_slot == 5 {
                self.selected_slot = 0
            } else {
                self.selected_slot += 1
            }
        }
    }

    pub fn sync(&mut self, player: &Player) {
        let inventory_slots = player.inventory.get_all_slots();
        for i in 0..6 {
            self.slots[i] = inventory_slots[0 + i];
        }
    }

    pub fn get_slots(&self) -> &[ItemStack; 6] {
        &self.slots
    }
}
