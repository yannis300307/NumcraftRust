use crate::{
    constants::{
        ItemType,
        color_palette::{GAMEUI_SLOT_COLOR, GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR},
    },
    eadk::{
        self, COLOR_BLACK, Point, Rect,
        display::{draw_string, pull_rect, push_rect, push_rect_uniform},
    },
    game_ui::{GameUI, GameUIElements},
    renderer::*,
};

impl Renderer {
    pub fn blur_screen(&self) {
        const BLURING_TILE_WIDTH: usize = SCREEN_WIDTH / BLURING_SCREEN_SUBDIVISION;
        const BLURING_TILE_HEIGHT: usize = SCREEN_HEIGHT / BLURING_SCREEN_SUBDIVISION;

        for x in 0..BLURING_SCREEN_SUBDIVISION {
            for y in 0..BLURING_SCREEN_SUBDIVISION {
                let tile_x = BLURING_TILE_WIDTH * x;
                let tile_y = BLURING_TILE_HEIGHT * y;
                let rect = Rect {
                    x: tile_x as u16,
                    y: tile_y as u16,
                    width: BLURING_TILE_WIDTH as u16,
                    height: BLURING_TILE_HEIGHT as u16,
                };
                let pixels = pull_rect(rect);

                let mut new_pixels = [COLOR_BLACK; BLURING_TILE_WIDTH * BLURING_TILE_HEIGHT];
                for p_x in 0..BLURING_TILE_WIDTH {
                    for p_y in 0..BLURING_TILE_HEIGHT {
                        let mut total_color = (0, 0, 0);
                        let mut pixels_count = 0;
                        for neighbor_x in (p_x as isize - BLURING_RADIUS).max(0) as usize
                            ..(p_x as isize + BLURING_RADIUS).min(BLURING_TILE_WIDTH as isize)
                                as usize
                        {
                            for neighbor_y in (p_y as isize - BLURING_RADIUS).max(0) as usize
                                ..(p_y as isize + BLURING_RADIUS).min(BLURING_TILE_HEIGHT as isize)
                                    as usize
                            {
                                let components = pixels
                                    [neighbor_x + neighbor_y * BLURING_TILE_WIDTH]
                                    .get_components();
                                total_color.0 += components.0 as usize;
                                total_color.1 += components.1 as usize;
                                total_color.2 += components.2 as usize;

                                pixels_count += 1;
                            }
                        }

                        new_pixels[p_x + p_y * BLURING_TILE_WIDTH] = Color::from_components(
                            (total_color.0 / pixels_count) as u16,
                            (total_color.1 / pixels_count) as u16,
                            (total_color.2 / pixels_count) as u16,
                        );
                    }
                }

                push_rect(rect, &new_pixels);
            }
        }
    }

    fn draw_scalled_tile_on_screen(&mut self, texture_id: u8, pos: Vector2<u16>, scale: usize) {
        let tileset_x = (texture_id % 16) as usize * 8;
        let tileset_y = (texture_id / 16) as usize * 8;
        //let size = (8*scale).pow(2);

        //let pixels: Vec<Color> = Vec::with_capacity(size);

        for x in 0..8 {
            for y in 0..8 {
                let texture_pixel_index = ((tileset_x + x) + (tileset_y + y) * 128) * 2;
                let pixel = u16::from_be_bytes([
                    TILESET_DATA[texture_pixel_index],
                    TILESET_DATA[texture_pixel_index + 1],
                ]);

                push_rect_uniform(
                    Rect {
                        x: pos.x + (x * scale) as u16,
                        y: pos.y + (y * scale) as u16,
                        width: scale as u16,
                        height: scale as u16,
                    },
                    Color { rgb565: pixel },
                );
            }
        }
    }

    pub fn draw_game_ui_container(&mut self, game_ui: &mut GameUI, element_id: usize) {
        let element = &game_ui.get_elements()[element_id];

        if !(game_ui.need_redraw || game_ui.need_complete_redraw) {
            return;
        }

        let x = element.pos.x;
        let y = element.pos.y;

        match &element.element {
            GameUIElements::ItemSlot { item_stack, .. } => {
                // Background
                if item_stack.get_item_type() == ItemType::Air {
                    push_rect_uniform(
                        Rect {
                            x: x + 3,
                            y: y + 3,
                            width: 24,
                            height: 24,
                        },
                        GAMEUI_SLOT_COLOR,
                    );
                }

                let color = if game_ui.selected_id.is_some_and(|id| id == element.id) {
                    Color::from_888(255, 242, 0)
                } else if game_ui.cursor_id == element.id {
                    Color::from_888(255, 0, 0)
                } else {
                    GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR
                };
                // Outline
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y,
                        width: 3,
                        height: 30,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y,
                        width: 30,
                        height: 3,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y + 27,
                        width: 30,
                        height: 3,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x + 27,
                        y: y,
                        width: 3,
                        height: 30,
                    },
                    color,
                );

                // Item texture
                let texture_id = item_stack.get_item_type().get_texture_id();

                if texture_id != 0 {
                    self.draw_scalled_tile_on_screen(texture_id, Vector2::new(3 + x, 3 + y), 3);

                    if !item_stack.creative_slot
                        || (game_ui.selected_amount.is_some()
                            && game_ui.selected_id.is_some_and(|id| id == element_id))
                    {
                        // Item amount
                        let amount_text = if let Some(selected_id) = game_ui.selected_id
                            && selected_id == element.id
                            && let Some(amount) = game_ui.selected_amount
                        {
                            format!("{}", amount)
                        } else {
                            format!("{}", item_stack.get_amount())
                        };

                        draw_string(
                            amount_text.as_str(),
                            Point {
                                x: (30 - 7 * amount_text.len() + x as usize) as u16,
                                y: y,
                            },
                            false,
                            Color::from_888(255, 255, 255),
                            GAMEUI_SLOT_COLOR,
                        );
                    }
                }

                // Amount selection bar
                if let Some(amount) = game_ui.selected_amount
                    && game_ui.selected_id.is_some_and(|id| id == element.id)
                    && item_stack.get_amount() != 0
                {
                    let amount_bar_lenght = 30 * amount / item_stack.get_amount() as usize;

                    push_rect_uniform(
                        Rect {
                            x: x,
                            y: y + 27,
                            width: amount_bar_lenght as u16,
                            height: 3,
                        },
                        if game_ui.is_selecting_amount {
                            Color::from_888(100, 150, 255)
                        } else {
                            Color::from_888(50, 100, 255)
                        },
                    );
                    push_rect_uniform(
                        Rect {
                            x: x + amount_bar_lenght as u16,
                            y: y + 27,
                            width: (30 - amount_bar_lenght) as u16,
                            height: 3,
                        },
                        Color::from_888(100, 100, 100),
                    );
                }
            }
            GameUIElements::Button { text, is_pressed } => todo!(),
            GameUIElements::Label { text } => {
                eadk::display::draw_string(
                    text,
                    Point { x, y },
                    false,
                    Color::from_888(0, 0, 0),
                    Color::from_888(255, 255, 255),
                );
            }
            GameUIElements::Arrow { filling } => {
                eadk::display::push_rect_uniform(
                    Rect {
                        x: element.pos.x + 2,
                        y: element.pos.y + 12,
                        width: 16,
                        height: 6,
                    },
                    Color::from_888(150, 150, 150),
                );

                for i in 0..=10 {
                    eadk::display::push_rect_uniform(
                        Rect {
                            x: element.pos.x + 18 + i,
                            y: element.pos.y - (10 - i) + 2 + 12,
                            width: 1,
                            height: (10 - i) * 2 + 2,
                        },
                        Color::from_888(150, 150, 150),
                    );
                }
            }
            GameUIElements::OneWayItemSlot { item_stack, .. } => {
                // Background
                if item_stack.get_item_type() == ItemType::Air {
                    push_rect_uniform(
                        Rect {
                            x: x + 3,
                            y: y + 3,
                            width: 24,
                            height: 24,
                        },
                        GAMEUI_SLOT_COLOR,
                    );
                }

                let color = if game_ui.selected_id.is_some_and(|id| id == element.id) {
                    Color::from_888(255, 242, 0)
                } else if game_ui.cursor_id == element.id {
                    Color::from_888(255, 0, 0)
                } else {
                    GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR
                };
                // Outline
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y,
                        width: 3,
                        height: 30,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y,
                        width: 30,
                        height: 3,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x,
                        y: y + 27,
                        width: 30,
                        height: 3,
                    },
                    color,
                );
                push_rect_uniform(
                    Rect {
                        x: x + 27,
                        y: y,
                        width: 3,
                        height: 30,
                    },
                    color,
                );

                // Item texture
                let texture_id = item_stack.get_item_type().get_texture_id();

                if texture_id != 0 {
                    self.draw_scalled_tile_on_screen(texture_id, Vector2::new(3 + x, 3 + y), 3);

                    if !item_stack.creative_slot
                        || (game_ui.selected_amount.is_some()
                            && game_ui.selected_id.is_some_and(|id| id == element_id))
                    {
                        // Item amount
                        let amount_text = format!("{}", item_stack.get_amount());

                        draw_string(
                            amount_text.as_str(),
                            Point {
                                x: (30 - 7 * amount_text.len() + x as usize) as u16,
                                y: y,
                            },
                            false,
                            Color::from_888(255, 255, 255),
                            GAMEUI_SLOT_COLOR,
                        );
                    }
                }
            }
        }

        #[cfg(feature = "debug_ui")]
        {
            eadk::display::draw_string(
                format!("{}", element_id).as_str(),
                Point { x: x + 2, y: y + 2 },
                false,
                Color::from_888(0, 0, 0),
                Color::from_888(255, 255, 255),
            );
        }
    }

    pub fn draw_game_ui(&mut self, game_ui: &mut GameUI) {
        if game_ui.blur_background && game_ui.need_complete_redraw {
            self.blur_screen();
        }

        let elements = game_ui.get_elements();

        for i in 0..elements.len() {
            self.draw_game_ui_container(game_ui, i);
        }
        game_ui.need_complete_redraw = false;
        game_ui.need_redraw = false;
    }
}
