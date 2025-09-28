use libm::tanf;

use crate::{
    constants::EntityType,
    entity::item::ItemEntityCustomData,
    renderer::{frustum::Frustum, *},
    world::World,
};

impl Renderer {
    pub fn draw_flat_model_entities(
        &mut self,
        world: &World,
        mat_view: &Matrix4<f32>,
        tile_x: usize,
        tile_y: usize,
        frustum: &Frustum,
    ) {
        for entity in world.get_all_entities() {
            if !entity
                .get_bbox()
                .is_some_and(|bbox| frustum.is_bbox_in_frustum(&bbox))
                || entity.pos.metric_distance(&self.camera.get_pos()) > MAX_ENTITY_RENDER_DISTANCE
            {
                continue;
            }

            if let EntityType::Item { .. } = entity.get_type() {
                // Extract the custom data of the entity
                let item_data = ItemEntityCustomData::get_item_data(&entity).expect("Item Entity must have ItemData as custom data.");

                let texture_id = item_data.item_stack.get_item_type().get_texture_id();

                // Transform and project the point
                let pos = entity.pos;
                let transformed = (mat_view * Vector4::new(pos.x, pos.y, pos.z, 1.0)).xyz();
                let projected = (self.project_point(transformed) + Vector2::new(1., 1.))
                    .component_mul(&HALF_SCREEN);

                let tile_offset = Vector2::new(
                    -((SCREEN_TILE_WIDTH * tile_x) as isize),
                    -((SCREEN_TILE_HEIGHT * tile_y) as isize),
                );

                let default_size = ITEM_ENTITY_SPRITE_SIZE;
                let sprite_size: isize = ((default_size
                    / self.camera.get_pos().metric_distance(&pos))
                    * (SCREEN_HEIGHTF / tanf(2.0 * (FOV / 2.0))))
                    as isize;

                let point = projected.map(|v| v as isize) + tile_offset;

                if true
                    || (point.x > -sprite_size / 2
                        && point.y > -sprite_size / 2
                        && point.x < SCREEN_TILE_WIDTH as isize + sprite_size / 2
                        && point.y < SCREEN_TILE_HEIGHT as isize + sprite_size / 2)
                {
                    self.draw_set_size_tile_on_frame_buffer(
                        texture_id,
                        point.map(|v| v - sprite_size / 2),
                        Vector2::repeat(sprite_size),
                    );
                }
            }
        }
    }

    fn draw_set_size_tile_on_frame_buffer(
        &mut self,
        texture_id: u8,
        pos: Vector2<isize>,
        size: Vector2<isize>,
    ) {
        let tileset_x = (texture_id % 16) as usize * 8;
        let tileset_y = (texture_id / 16) as usize * 8;

        if pos.x + size.x <= 0 || pos.y + size.y <= 0 {
            return;
        }
        for x in 0..size.x {
            let dest_x = x + pos.x;
            if dest_x < 0 || dest_x >= SCREEN_TILE_WIDTH as isize {
                continue;
            }
            for y in 0..size.y {
                let u = x * 8 / size.x;
                let v = y * 8 / size.y;

                let dest_y = y + pos.y;
                if dest_y < 0 || dest_y >= SCREEN_TILE_HEIGHT as isize {
                    continue;
                }

                let texture_pixel_index =
                    ((tileset_x + u as usize) + (tileset_y + v as usize) * 128) * 2;
                let pixel = u16::from_be_bytes([
                    TILESET_DATA[texture_pixel_index],
                    TILESET_DATA[texture_pixel_index + 1],
                ]);

                self.tile_frame_buffer[dest_x as usize + dest_y as usize * SCREEN_TILE_WIDTH] =
                    Color { rgb565: pixel };
            }
        }
    }
}
