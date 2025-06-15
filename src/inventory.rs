// In src/inventory.rs

// Import from the correct constants module
use crate::constants::{BlockType, rendering::{SCREEN_HEIGHT, SCREEN_WIDTH}, QuadDir};
// Import Color and the display module, and Rect from eadk
use crate::eadk::{Color, Rect, display}; // Corrected: import `Rect`
// Import the public function from constants
use crate::constants::get_quad_color_from_texture_id;
// Import UI colors from constants
use crate::constants::{UI_BLACK, UI_LIGHT_GREY, UI_RED};


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

    // Drawing the hotbar on the screen
    // Removed `display: &mut Display` parameter as eadk::display functions are global
    pub fn draw(&self) {
        let slot_size: u16 = 20; // Size of each inventory slot square
        let padding: u16 = 2; // Padding between slots
        let border_thickness: u16 = 1; // Border for slots

        // Calculate total width and starting X position to center the hotbar
        // Cast to u16 for calculations to avoid usize/u16 mismatch
        let total_width = (slot_size + padding) * (INVENTORY_HOTBAR_SIZE as u16) - padding;
        let start_x = (SCREEN_WIDTH as u16 - total_width) / 2; // Use SCREEN_WIDTH from constants
        let start_y = SCREEN_HEIGHT as u16 - slot_size - 5; // Use SCREEN_HEIGHT from constants

        // Refactored to use `enumerate()` for a "foreach" like iteration
        for (i, &block_type) in self.slots.iter().enumerate() {
            let x = start_x + (i as u16) * (slot_size + padding);
            let y = start_y;

            // Draw background for the slot using `eadk::display::push_rect_uniform`
            display::push_rect_uniform(
                Rect { x, y, width: slot_size, height: slot_size },
                UI_LIGHT_GREY
            );

            // Draw block if not air
            if block_type != BlockType::Air {
                let texture_id = block_type.get_texture_id(QuadDir::Top);
                let block_color = get_quad_color_from_texture_id(texture_id);

                // Draw a smaller rectangle inside for the block color using `eadk::display::push_rect_uniform`
                display::push_rect_uniform(
                    Rect {
                        x: x + border_thickness,
                        y: y + border_thickness,
                        width: slot_size - 2 * border_thickness,
                        height: slot_size - 2 * border_thickness,
                    },
                    block_color,
                );
            }

            // Draw selection border if this slot is selected using `eadk::display::push_rect_uniform`
            if i == self.selected_slot_index {
                display::push_rect_uniform(Rect { x, y, width: slot_size, height: slot_size }, UI_RED);
            } else {
                display::push_rect_uniform(Rect { x, y, width: slot_size, height: slot_size }, UI_BLACK);
            }
        }
    }
}
