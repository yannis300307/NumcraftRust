use crate::{
    constants::ItemType,
    game::*,
    game_ui::{ContainerNeighbors, GameUIElements, NeighborDirection},
    inventory::Inventory,
};

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

        let inventories = [
            &mut self.player.inventory,
            &mut self.crafting_manager.crafting_inventory_2x2,
        ];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(65, 86), 6, 3, 0, 0, 6)
            .with_slot_grid(Vector2::new(65, 184), 6, 1, 0, 18, 0)
            .with_slot_grid(Vector2::new(97, 16), 2, 2, 1, 24, 0)
            .with_element(
                GameUIElements::create_one_way_slot_slot(1, 4),
                Vector2::new(193, 32),
                28,
                ContainerNeighbors::default(),
            )
            .with_element(
                GameUIElements::Arrow { filling: 0. },
                Vector2::new(161, 32),
                29,
                ContainerNeighbors::default(),
            )
            .with_links(&[
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
                (26, 1, NeighborDirection::Bottom),
                (27, 2, NeighborDirection::Bottom),
                (28, 4, NeighborDirection::Bottom),
                (25, 28, NeighborDirection::Right),
                (27, 28, NeighborDirection::Right),
            ])
            .sync(&inventories);

        ui.selected_amount = None;

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);
            self.crafting_manager.update_2x2();

            let mut inventories = [
                &mut self.player.inventory,
                &mut self.crafting_manager.crafting_inventory_2x2,
            ];

            if !ui.update(&self.input_manager, &mut inventories) {
                // Bring the items back in the inventory
                for slot in 0..4 {
                    let item_stack = inventories[1].get_all_slots()[slot].clone();
                    if item_stack.get_item_type() != ItemType::Air {
                        let remaining = inventories[0].add_item_stack(item_stack);
                        if remaining != 0 {
                            // Hum... wait?!
                            // I have no choice... Spawn the item.
                            // I should be carreful about duplication here...
                            let pos = self.world.get_player_entity().pos;
                            self.world.spawn_item_entity(
                                pos,
                                ItemStack::new(item_stack.get_item_type(), remaining, false),
                            );
                        }
                    }
                }

                // Then clear the crafting inventory.
                inventories[1].fill(ItemStack::void());
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
        creative_inventory.replace_slot_item_stack(
            3,
            ItemStack::new(crate::constants::ItemType::SandBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            4,
            ItemStack::new(crate::constants::ItemType::CobblestoneBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            5,
            ItemStack::new(crate::constants::ItemType::BorderBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            6,
            ItemStack::new(crate::constants::ItemType::LogBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            7,
            ItemStack::new(crate::constants::ItemType::LeavesBlock, 1, true),
        );
        creative_inventory.replace_slot_item_stack(
            8,
            ItemStack::new(crate::constants::ItemType::PlanksBlock, 1, true),
        );

        let mut inventories = [&mut self.player.inventory, &mut creative_inventory];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(10, 41), 6, 3, 0, 0, 6)
            .with_slot_grid(Vector2::new(10, 139), 6, 1, 0, 18, 0)
            .with_slot_grid(Vector2::new(218, 9), 3, 7, 1, 24, 0)
            .with_links(&[
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
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
