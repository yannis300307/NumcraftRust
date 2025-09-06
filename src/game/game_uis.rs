use crate::game::*;

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
            .with_element(
                crate::game_ui::GameUIElements::create_slot(0, 0),
                Vector2::new(20, 20),
                0,
                ContainerNeighbors::new(None, None, None, Some(1)),
            )
            .with_element(
                crate::game_ui::GameUIElements::create_slot(0, 1),
                Vector2::new(68, 20),
                1,
                ContainerNeighbors::new(None, None, Some(0), Some(2)),
            )
            .with_element(
                crate::game_ui::GameUIElements::create_slot(0, 2),
                Vector2::new(116, 20),
                2,
                ContainerNeighbors::new(None, None, Some(1), None),
            )
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