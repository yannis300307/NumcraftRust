use crate::{game::*, game_ui::NeighborDirection};

impl Game {
    pub fn player_inventory_loop(&mut self) {
        self.player
            .inventory
            .replace_slot_item_stack(0, ItemStack::new(crate::constants::ItemType::DirtBlock, 24));
        self.player
            .inventory
            .replace_slot_item_stack(1, ItemStack::new(crate::constants::ItemType::DirtBlock, 1));
        self.player.inventory.replace_slot_item_stack(
            2,
            ItemStack::new(crate::constants::ItemType::GrassBlock, 50),
        );

        let mut inventories = [&mut self.player.inventory];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(22, 30), 6, 3, 0, 0, 0)
            .with_slot_grid(Vector2::new(22, 184), 6, 1, 0, 18, 18)
            .with_links(&[
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
            ])
            .sync(&inventories);

        ui.selected_amount = None;
        loop {
            self.input_manager.update();

            ui.update(&self.input_manager, &mut inventories);

            self.renderer.draw_game_UI(&mut ui);

            eadk::display::wait_for_vblank();
            eadk::timing::msleep(50);
        }
    }
}
