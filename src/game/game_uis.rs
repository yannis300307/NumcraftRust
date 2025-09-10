use crate::{game::*, game_ui::NeighborDirection, input_manager};

pub enum PlayerInventoryPage {
    Normal,
    Creative,
}

impl Game {
    pub fn player_inventory_loop(&mut self, page: PlayerInventoryPage) -> GameState {
        match page {
            PlayerInventoryPage::Normal => self.player_inventory_normal_loop(),
            PlayerInventoryPage::Creative => todo!(),
        }

        GameState::InGame
    }

    fn player_inventory_normal_loop(&mut self) {
        // Clear the hud
        self.renderer.draw_game(&mut self.world, &self.player, 0., &self.hud, false);


         self.player
            .inventory
            .replace_slot_item_stack(0, ItemStack::new(crate::constants::ItemType::DirtBlock, 24, false));
        self.player
            .inventory
            .replace_slot_item_stack(1, ItemStack::new(crate::constants::ItemType::DirtBlock, 1, false));
        self.player.inventory.replace_slot_item_stack(
            2,
            ItemStack::new(crate::constants::ItemType::GrassBlock, 50, false),
        );

        self.player.inventory.replace_slot_item_stack(
            12,
            ItemStack::new(crate::constants::ItemType::StoneBlock, 1, true),
        );

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
        
        loop {
            self.input_manager.update();

            if !ui.update(&self.input_manager, &mut inventories) {
                break;
            }

            self.renderer.draw_game_ui(&mut ui);

            eadk::display::wait_for_vblank();
            eadk::timing::msleep(50);
        }
    }
}
