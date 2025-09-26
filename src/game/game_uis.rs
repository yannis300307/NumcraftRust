use crate::{game::*, game_ui::NeighborDirection, inventory::Inventory};

pub enum PlayerInventoryPage {
    Survival,
    Creative,
}

impl Game {
    pub fn player_inventory_loop(&mut self, page: PlayerInventoryPage) -> GameState {
        match page {
            PlayerInventoryPage::Survival => self.player_inventory_survival_loop(),
            PlayerInventoryPage::Creative => self.player_inventory_creative_loop(),
        }

        GameState::InGame
    }

    fn player_inventory_survival_loop(&mut self) {
        // Clear the hud
        self.renderer
            .draw_game(&mut self.world, &self.player, 0., &self.hud, false);

        let mut inventories = [&mut self.player.inventory];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(65, 56), 6, 3, 0, 0, 0)
            .with_slot_grid(Vector2::new(65, 154), 6, 1, 0, 18, 18)
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

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            if !ui.update(&self.input_manager, &mut inventories) {
                break;
            }

            self.renderer.draw_game_ui(&mut ui);

            eadk::display::wait_for_vblank();
            eadk::timing::msleep(50);
        }
    }

    fn player_inventory_creative_loop(&mut self) {
        // Clear the hud
        self.renderer
            .draw_game(&mut self.world, &self.player, 0., &self.hud, false);

        let mut creative_inventory = Inventory::new(21);

        creative_inventory.fill(ItemStack::new(crate::constants::ItemType::Air, 0, true));

        creative_inventory.replace_slot_item_stack(
            0,
            ItemStack::new(crate::constants::ItemType::StoneBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            1,
            ItemStack::new(crate::constants::ItemType::DirtBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            2,
            ItemStack::new(crate::constants::ItemType::GrassBlock, 1, true),
        );

        let mut inventories = [&mut self.player.inventory, &mut creative_inventory];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(10, 41), 6, 3, 0, 0, 0)
            .with_slot_grid(Vector2::new(10, 139), 6, 1, 0, 18, 18)
            .with_links(&[
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
            ])
            .with_slot_grid(Vector2::new(218, 9), 3, 7, 1, 24, 0)
            .with_links(&[
                (5, 27, NeighborDirection::Right),
                (11, 30, NeighborDirection::Right),
                (17, 33, NeighborDirection::Right),
                (23, 36, NeighborDirection::Right),
            ])
            .sync(&inventories);

        ui.selected_amount = None;

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            if !ui.update(&self.input_manager, &mut inventories) {
                break;
            }

            self.renderer.draw_game_ui(&mut ui);

            eadk::display::wait_for_vblank();
            eadk::timing::msleep(50);
        }
    }
}
