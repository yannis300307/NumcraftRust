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
        while index < 9 {
            pattern[index % 3][index / 3] =
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
            // Get the tiniest possible rectangle in the grid

            // Get the top left corner x
            let mut x1 = 0;
            'scan: while x1 < 3 {
                for i in 0..3 {
                    if grid[x1][i] != ItemType::Air {
                        break 'scan;
                    }
                }
                x1 += 1;
            }
            if x1 == 3 {
                return false;
            }

            // Get the top left corner y
            let mut y1 = 0;
            'scan: while y1 < 3 {
                for i in 0..3 {
                    if grid[i][y1] != ItemType::Air {
                        break 'scan;
                    }
                }
                y1 += 1;
            }

            if y1 == 3 {
                return false;
            }

            // Get the bottom right corner x
            let mut x2 = 2;
            'scan: while x2 > x1 {
                for i in 0..3 {
                    if grid[x2][i] != ItemType::Air {
                        break 'scan;
                    }
                }
                x2 -= 1;
            }

            // Get the bottom right corner x
            let mut y2 = 2;
            'scan: while y2 > y1 {
                for i in 0..3 {
                    if grid[i][y2] != ItemType::Air {
                        break 'scan;
                    }
                }
                y2 -= 1;
            }

            let width = x2 - x1;
            let height = y2 - y1;

            // Check if it matches the shape
            for x in 0..3 {
                for y in 0..3 {
                    if x <= width && y <= height {
                        // Check if the pattern is right
                        if self.pattern[x][y] != grid[x1 + x][y1 + y] {
                            return false;
                        }
                    } else {
                        // Check if there is no items outside of the pattern
                        if self.pattern[x][y] != ItemType::Air {
                            return false;
                        }
                    }
                }
            }

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
            // These loop and sub loops do 162 iterations but can be optimised. TODO
            for item in items_grid.iter() {
                let count_grid = items_grid.iter().filter(|v| *v == item).count();
                let count_craft = items_craft.iter().filter(|v| *v == item).count();

                if count_grid != count_craft {
                    return false;
                }
            }

            true
        }
    }
}

const CRAFTS: [Craft; 3] = [
    Craft::new(include_bytes!("../../target/crafts/planks.bin")),
    Craft::new(include_bytes!("../../target/crafts/stone.bin")),
    Craft::new(include_bytes!("../../target/crafts/grass.bin")),
];

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
