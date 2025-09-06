use crate::renderer::*;

impl Renderer {
    pub fn draw_ui(&mut self, fps_count: f32, tile_x: usize, tile_y: usize) {
        if tile_x == 0 && tile_y == 0 {
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
}