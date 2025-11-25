use crate::{
    eadk::display::{
        Color565, SCREEN_RECT, ScreenPoint, ScreenRect, draw_string, pull_rect, push_rect,
        push_rect_uniform,
    },
    renderer::*,
};

pub struct UnBoundedScreenRect {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
}

#[allow(dead_code)]
impl Renderer {
    pub fn draw_string(&mut self, text: &str, pos: &Vector2<usize>) {
        let mut text_cursor: usize = 0;
        for char in text.chars() {
            let font_index = FONT_ORDER.chars().position(|c| c == char).unwrap();

            let font_pixel_index = font_index * FONT_CHAR_WIDTH;

            for x in 0..FONT_CHAR_WIDTH {
                for y in 0..FONT_HEIGHT {
                    let pixel_value = FONT_DATA[(font_pixel_index + x) + y * FONT_WIDTH];

                    let rgb565 = Color565::from_rgb888(
                        pixel_value as u16,
                        pixel_value as u16,
                        pixel_value as u16,
                    );

                    let pix_x = pos.x + x + text_cursor;

                    if pix_x >= SCREEN_TILE_WIDTH {
                        continue;
                    }

                    self.tile_frame_buffer[pix_x + (pos.y + y) * SCREEN_TILE_WIDTH] = rgb565;
                }
            }
            text_cursor += FONT_CHAR_WIDTH;
        }
    }

    fn draw_string_no_bg_on_screen(&mut self, text: &str, pos: Vector2<usize>) {
        let mut text_cursor: usize = 0;

        let rect_width = FONT_CHAR_WIDTH * text.len();
        let rect = ScreenRect {
            x: pos.x as u16,
            y: pos.y as u16,
            width: rect_width as u16,
            height: FONT_HEIGHT as u16,
        };

        let mut bg_pixels = pull_rect(rect);

        for char in text.chars() {
            let font_index = FONT_ORDER.chars().position(|c| c == char).unwrap();

            let font_pixel_index = font_index * FONT_CHAR_WIDTH;

            for x in 0..FONT_CHAR_WIDTH {
                for y in 0..FONT_HEIGHT {
                    let pixel_value = FONT_DATA[(font_pixel_index + x) + y * FONT_WIDTH];

                    let pix_x = x + text_cursor;

                    if pix_x >= rect_width {
                        continue;
                    }

                    let rgb565 = bg_pixels[pix_x + y * rect_width].apply_light(255 - pixel_value);

                    bg_pixels[pix_x + y * rect_width] = rgb565;
                }
            }
            text_cursor += FONT_CHAR_WIDTH;
        }

        push_rect(rect, &bg_pixels);
    }

    pub fn push_rect_uniform_on_frame_buffer(&mut self, rect: ScreenRect, color: Color565) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                self.tile_frame_buffer[x as usize + y as usize * SCREEN_TILE_WIDTH] = color;
            }
        }
    }

    pub fn push_unbounded_rect_uniform_on_frame_buffer(
        &mut self,
        rect: UnBoundedScreenRect,
        color: Color565,
    ) {
        if rect.x + rect.width <= 0 || rect.y + rect.height <= 0 {
            return;
        }
        for x in rect.x.max(0)..(rect.x + rect.width).min(SCREEN_TILE_WIDTH as isize) {
            for y in rect.y.max(0)..(rect.y + rect.height).min(SCREEN_TILE_HEIGHT as isize) {
                self.tile_frame_buffer[x as usize + y as usize * SCREEN_TILE_WIDTH] = color;
            }
        }
    }

    pub fn show_msg(message: &[&str], background_color: Color565) {
        push_rect_uniform(SCREEN_RECT, background_color);

        let mut y = (SCREEN_HEIGHT - message.len() * 20) / 2;

        for line in message {
            draw_string(
                line,
                ScreenPoint {
                    x: ((320 - line.len() * 10) / 2) as u16,
                    y: y as u16,
                },
                true,
                Color565::from_rgb888(0, 0, 0),
                background_color,
            );
            y += 20
        }
    }
}

impl Color565 {
    pub fn apply_light(&self, light_level: u8) -> Self {
        let light_level = light_level as u16;
        let rgb = self.get_components();
        Color565::new(
            rgb.0 * light_level / 255,
            rgb.1 * light_level / 255,
            rgb.2 * light_level / 255,
        )
    }
}

#[inline(always)]
pub fn inflate(n: u32) -> u32 {
    let n = (n | (n << 16)) & 0xff0000ff;
    let n = (n | (n << 8)) & 0x0f00f00f;
    let n = (n | (n << 4)) & 0xc30c30c3;
    let n = (n | (n << 2)) & 0x49249249;
    n
}

#[inline(always)]
pub fn interleave(x: u32, y: u32, z: u32) -> u32 {
    let x = inflate(x);
    let y = inflate(y) << 1;
    let z = inflate(z) << 2;
    x | y | z
}

#[inline(always)]
pub fn flatten_zorder(p: Vector3<u32>) -> u32 {
    interleave(p.x, p.y, p.z)
}
