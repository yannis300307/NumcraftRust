use crate::{entity::Entity, inventory::ItemStack};

pub struct ItemEntityCustomData {
    pub item_stack: ItemStack,
}

impl ItemEntityCustomData {
    pub fn get_item_data(entity: &Entity) -> Option<&Self> {
        let custom_data = entity.custom_data.as_ref()?;
        let item_data = custom_data
            .downcast_ref::<ItemEntityCustomData>()
            .expect("Item Entity custom data must be an instance of struct ItemEntityCustomData.");

        Some(item_data)
    }
}
