use crate::{
    constants::ItemType,
    inventory::{Inventory, ItemStack},
};

struct Craft {
    pattern: [[ItemType; 3]; 3],
    strict_shape: bool,
    result: ItemStack,
}

impl Craft {
    pub const fn new(data: &'static [u8]) -> Self {
        let mut pattern = [const { [ItemType::Air; 3] }; 3];

        // 9 bytes : pattern
        let mut index = 0;

        // We can't use a for loop because it is not available in const context
        while index <= 9 {
            pattern[index % 3][index / 4] =
                ItemType::get_from_id(data[index]).expect("Invalid item id in craft.");
            index += 1;
        }

        // 1 byte : strict_shape
        let strict_shape = data[9] == 1;

        // 1 byte : result
        let result_type = ItemType::get_from_id(data[10]).expect("Invalid item id in craft.");

        // 1 byte : result amount
        let result_amount = data[11];

        let result = ItemStack::new(result_type, result_amount, false);

        Craft {
            pattern,
            strict_shape,
            result,
        }
    }

    pub fn matches(&self, grid: [[ItemType; 3]; 3]) -> bool {
        if self.strict_shape {
            // TODO
            true
        } else {
            // The max size is 3*3
            let mut items_craft: heapless::Vec<ItemType, 9> = heapless::Vec::new();
            let mut items_grid: heapless::Vec<ItemType, 9> = heapless::Vec::new();

            for x in 0..3 {
                for y in 0..3 {
                    let item = self.pattern[x][y];
                    // 3 * 3 = 9 so if it fails, mathematics are broken
                    items_craft.push(item).unwrap();
                    let item = grid[x][y];
                    items_grid.push(item).unwrap();
                }
            }

            // Check if the items are the same
            for item in items_grid.iter() {
                let index = items_craft.iter().position(|other| *other == *item);

                if let Some(index) = index {
                    items_craft.remove(index);
                } else {
                    return false;
                }
            }

            true
        }
    }
}

const CRAFTS: [Craft; 1] = [Craft::new(include_bytes!("../../target/crafts/planks.bin"))];

pub struct CraftingManager {
    pub crafting_inventory_2x2: Inventory,
    pub crafting_inventory_3x3: Inventory,
    valid_craft: bool,
}

impl CraftingManager {
    pub fn new() -> Self {
        let crafting_inventory_2x2 = Inventory::new(5);
        let crafting_inventory_3x3 = Inventory::new(10);
        CraftingManager {
            crafting_inventory_2x2,
            crafting_inventory_3x3,
            valid_craft: false,
        }
    }

    pub fn update_2x2(&mut self) {
        // Remove the recipies if the player picked up the item
        if self.valid_craft
            && self
                .crafting_inventory_2x2
                .get_item_type_at_slot_index(4)
                .unwrap()
                == ItemType::Air
        {
            for i in 0..=3 {
                self.crafting_inventory_2x2.take_one(i);
            }
        }

        let mut grid = [[ItemType::Air; 3]; 3];

        // The inventory slots indexes must be from 0 to 3 included
        for x in 0..2 {
            for y in 0..2 {
                grid[x][y] = self
                    .crafting_inventory_2x2
                    .get_item_type_at_slot_index(x + y * 2)
                    .unwrap(); // If it fails, a cosmic particle just hit the calculators Ram! Incredible!
            }
        }

        // Check for all crafts to match with our grid
        if self.crafting_inventory_2x2.modified {
            let mut found_craft = false;
            for craft in CRAFTS {
                if craft.matches(grid) {
                    self.crafting_inventory_2x2
                        .replace_slot_item_stack(4, craft.result);
                    //self.crafting_inventory_2x2.modified = false;
                    self.valid_craft = true;
                    found_craft = true;
                    break;
                }
            }
            if !found_craft {
                self.crafting_inventory_2x2
                    .replace_slot_item_stack(4, ItemStack::void());
                self.valid_craft = false;
            }
        }
    }
}
