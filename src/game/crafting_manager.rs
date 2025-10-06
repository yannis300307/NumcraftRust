use crate::inventory::Inventory;

pub struct CraftingManager {
    pub crafting_inventory_2x2: Inventory,
    pub crafting_inventory_3x3: Inventory,
}

impl CraftingManager {
    pub fn new() -> Self {
        let crafting_inventory_2x2 = Inventory::new(5);
        let crafting_inventory_3x3 = Inventory::new(10);
        CraftingManager { crafting_inventory_2x2, crafting_inventory_3x3 }
    }

    pub fn update(&mut self) {
        
    }
}