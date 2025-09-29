use crate::{
    constants::{ItemType, color_palette::GAMEUI_SLOT_COLOR},
    eadk::Rect,
    hud::Hud,
    renderer::*,
};

impl Renderer {
    pub fn draw_hud(&mut self, hud: &Hud, fps_count: f32, tile_x: usize, tile_y: usize) {
        if tile_x == 0 && tile_y == 0 {
            if hud.show_debug {
                self.draw_string(
                    format!("FPS:{fps_count:.2}").as_str(),
                    &Vector2::new(10, 10),
                );

                self.draw_string(
                    format!("Tris:{}", self.triangles_to_render.len()).as_str(),
                    &Vector2::new(10, 30),
                );

                self.draw_string(
                    format!(
                        "{:.1},{:.1},{:.1}",
                        self.camera.get_pos().x,
                        self.camera.get_pos().y,
                        self.camera.get_pos().z
                    )
                    .as_str(),
                    &Vector2::new(10, 50),
                );
            }
        }
        
        let mut draw_cross = |x, y| {
            self.draw_image_negate(
                CROSS_DATA,
                Vector2::new(CROSS_WIDTH as isize, CROSS_HEIGHT as isize),
                Vector2::new(x, y),
            );
        };

        if tile_x == 0 && tile_y == 0 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 1 && tile_y == 0 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 1 && tile_y == 1 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                -((CROSS_HEIGHT / 2) as isize),
            );
        }
        if tile_x == 0 && tile_y == 1 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                -((CROSS_HEIGHT / 2) as isize),
            );

            self.draw_slot_frame_buffer(Vector2::new(60, 85), hud, 0);
            self.draw_slot_frame_buffer(Vector2::new(94, 85), hud, 1);
            self.draw_slot_frame_buffer(Vector2::new(128, 85), hud, 2);
        }
        if tile_x == 1 && tile_y == 1 {
            self.draw_slot_frame_buffer(Vector2::new(2, 85), hud, 3);
            self.draw_slot_frame_buffer(Vector2::new(36, 85), hud, 4);
            self.draw_slot_frame_buffer(Vector2::new(70, 85), hud, 5);
        }

        self.draw_breaking_indicator(tile_x, tile_y, hud);
    }

    pub fn draw_breaking_indicator(&mut self, tile_x: usize, tile_y: usize, hud: &Hud) {
        if let Some(progress) = hud.breaking_progress {
            let bar_len = (40. * progress) as u16;
            if tile_x == 0 && tile_y == 1 {
                self.push_rect_uniform_on_frame_buffer(
                    Rect {
                        x: 138,
                        y: 18,
                        width: 22,
                        height: 9,
                    },
                    Color::from_888(100, 100, 100),
                );
                self.push_rect_uniform_on_frame_buffer(
                    Rect {
                        x: 140,
                        y: 20,
                        width: bar_len.min(20),
                        height: 5,
                    },
                    Color::from_888(200, 200, 200),
                );
            }
            if tile_x == 1 && tile_y == 1 {
                self.push_rect_uniform_on_frame_buffer(
                    Rect {
                        x: 0,
                        y: 18,
                        width: 22,
                        height: 9,
                    },
                    Color::from_888(100, 100, 100),
                );
                if bar_len > 20 {
                    self.push_rect_uniform_on_frame_buffer(
                        Rect {
                            x: 0,
                            y: 20,
                            width: (bar_len - 20),
                            height: 5,
                        },
                        Color::from_888(200, 200, 200),
                    );
                }
            }
        }
    }

    pub fn draw_image_negate(
        &mut self,
        image: &[u8],
        image_size: Vector2<isize>,
        pos: Vector2<isize>,
    ) {
        for y in 0..image_size.y {
            if pos.y + y < 0 || pos.y + y >= SCREEN_TILE_HEIGHT as isize {
                continue;
            }
            for x in 0..image_size.x {
                let dest = Vector2::new(x, y) + pos;

                if dest.x < 0 || dest.x >= SCREEN_TILE_WIDTH as isize {
                    continue;
                }

                let pixel = image[(x + image_size.x * y) as usize];

                if pixel == 0 {
                    let frame_buff_index = (dest.x + dest.y * SCREEN_TILE_WIDTH as isize) as usize;
                    let components = self.tile_frame_buffer[frame_buff_index].get_components();

                    let inverted_color = Color::from_components(
                        0b11111 - components.0,
                        0b111111 - components.1,
                        0b11111 - components.2,
                    );
                    self.tile_frame_buffer[frame_buff_index] = inverted_color;
                }
            }
        }
    }

    fn draw_scalled_tile_on_frame_buffer(
        &mut self,
        texture_id: u8,
        pos: Vector2<u16>,
        scale: usize,
    ) {
        let tileset_x = (texture_id % 16) as usize * 8;
        let tileset_y = (texture_id / 16) as usize * 8;

        for x in 0..8 {
            for y in 0..8 {
                let texture_pixel_index = ((tileset_x + x) + (tileset_y + y) * 128) * 2;
                let pixel = u16::from_be_bytes([
                    TILESET_DATA[texture_pixel_index],
                    TILESET_DATA[texture_pixel_index + 1],
                ]);

                self.push_rect_uniform_on_frame_buffer(
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

    fn draw_slot_frame_buffer(&mut self, pos: Vector2<u16>, hud: &Hud, slot_index: usize) {
        self.push_rect_uniform_on_frame_buffer(
            Rect {
                x: pos.x,
                y: pos.y,
                width: 30,
                height: 30,
            },
            if hud.selected_slot == slot_index {
                Color::from_888(200, 50, 50)
            } else {
                GAMEUI_SLOT_COLOR
            },
        );
        let item_stack = hud.get_slots()[slot_index];
        let texture_id = item_stack.get_item_type().get_texture_id();

        if texture_id != 0 {
            self.draw_scalled_tile_on_frame_buffer(texture_id, pos + Vector2::new(3, 3), 3);
        }

        let item_type = item_stack.get_item_type();
        let max_item_count = item_type.get_max_stack_amount();

        if item_type != ItemType::Air && item_type.get_max_stack_amount() > 1 {
            let item_bar_lenght = 24 * item_stack.get_amount() as u16 / max_item_count as u16;
            self.push_rect_uniform_on_frame_buffer(
                Rect {
                    x: pos.x + 3,
                    y: pos.y + 24,
                    width: item_bar_lenght,
                    height: 3,
                },
                Color::from_888(100, 150, 255),
            );
            self.push_rect_uniform_on_frame_buffer(
                Rect {
                    x: pos.x + 3 + item_bar_lenght,
                    y: pos.y + 24,
                    width: 24 - item_bar_lenght,
                    height: 3,
                },
                Color::from_888(200, 200, 200),
            );
        }
    }
}
