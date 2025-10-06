use crate::{constants::ItemType, inventory::Inventory};

struct Craft {
    pattern: [[ItemType; 3]; 3],
    strict_shape: bool,
    result: ItemType,
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
        let result = ItemType::get_from_id(data[10]).expect("Invalid item id in craft.");

        Craft {
            pattern,
            strict_shape,
            result,
        }
    }
}

const CRAFTS: [Craft; 1] = [Craft::new(include_bytes!("../../target/crafts/planks.bin"))];

pub struct CraftingManager {
    pub crafting_inventory_2x2: Inventory,
    pub crafting_inventory_3x3: Inventory,
}

impl CraftingManager {
    pub fn new() -> Self {
        let crafting_inventory_2x2 = Inventory::new(5);
        let crafting_inventory_3x3 = Inventory::new(10);
        CraftingManager {
            crafting_inventory_2x2,
            crafting_inventory_3x3,
        }
    }

    pub fn update(&mut self) {}
}
