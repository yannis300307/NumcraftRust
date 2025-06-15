// In src/inventory.rs

// Import from the correct constants module
use crate::constants::{BlockType, rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, QuadDir};
// Import Color and the display module, and Rect from eadk
use crate::eadk::{Color, Rect, display}; // Corrected: import `Rect`
// Import the public function from constants
use crate::constants::get_quad_color_from_texture_id;
// Import UI colors from constants
// use crate::constants::{UI_LIGHT_GREY}; // No use


// Max slots in our hotbar
pub const INVENTORY_HOTBAR_SIZE: usize = 9;

pub struct Inventory {
    pub slots: [BlockType; INVENTORY_HOTBAR_SIZE],
    pub selected_slot_index: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            // Initialize with some default blocks from your BlockType enum
            slots: [
                BlockType::Stone,
                BlockType::Grass,
                BlockType::Dirt,
                BlockType::Air, // Empty slot
                BlockType::Air,
                BlockType::Air,
                BlockType::Air,
                BlockType::Air,
                BlockType::Air,
            ],
            selected_slot_index: 0, // Start with the first slot selected
        }
    }

    pub fn get_selected_block_type(&self) -> BlockType {
        self.slots[self.selected_slot_index]
    }

    // Function to change the selected slot
    pub fn select_next_slot(&mut self) {
        self.selected_slot_index = (self.selected_slot_index + 1) % INVENTORY_HOTBAR_SIZE;
    }

    pub fn select_previous_slot(&mut self) {
        if self.selected_slot_index == 0 {
            self.selected_slot_index = INVENTORY_HOTBAR_SIZE - 1;
        } else {
            self.selected_slot_index -= 1;
        }
    }

    
}
