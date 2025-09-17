use crate::{
    constants::color_palette::*,
    eadk::{
        Point, Rect,
        display::{draw_string, push_rect_uniform, wait_for_vblank},
    },
    menu::{Menu, MenuElement, TextAnchor},
    renderer::*,
};

impl Renderer {
    pub fn draw_menu(&self, menu: &mut Menu) {
        if !menu.need_redraw {
            return;
        }

        menu.need_redraw = false;

        let mut element_y = menu.pos.y;

        let elements = menu.get_elements();
        for i in 0..elements.len() {
            let element = &elements[i];

            let default_rect = if matches!(&elements[i], MenuElement::Button { .. }) {
                if i < elements.len() - 1
                    && let MenuElement::ButtonOption { text, .. } = &elements[i + 1]
                {
                    let option_button_width = 20 + text.len() * 10;
                    Rect {
                        x: menu.pos.x as u16,
                        y: element_y as u16,
                        width: (menu.width - option_button_width - 10) as u16,
                        height: 30,
                    }
                } else {
                    Rect {
                        x: menu.pos.x as u16,
                        y: element_y as u16,
                        width: menu.width as u16,
                        height: 30,
                    }
                }
            } else if let MenuElement::ButtonOption { text, .. } = &elements[i] {
                let option_button_width = 20 + text.len() * 10;
                Rect {
                    x: (menu.pos.x + menu.width - option_button_width) as u16,
                    y: element_y as u16,
                    width: option_button_width as u16,
                    height: 30,
                }
            } else {
                Rect {
                    x: menu.pos.x as u16,
                    y: element_y as u16,
                    width: menu.width as u16,
                    height: 30,
                }
            };

            let draw_outline = || {
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y - 1,
                        width: default_rect.width + 2,
                        height: 1,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y + default_rect.height,
                        width: default_rect.width + 2,
                        height: 1,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x - 1,
                        y: default_rect.y,
                        width: 1,
                        height: default_rect.height,
                    },
                    MENU_OUTLINE_COLOR,
                );
                push_rect_uniform(
                    Rect {
                        x: default_rect.x + default_rect.width,
                        y: default_rect.y,
                        width: 1,
                        height: default_rect.height,
                    },
                    MENU_OUTLINE_COLOR,
                );
            };

            let element_bg_color = if i == menu.selected_index {
                MENU_ELEMENT_BACKGROUND_COLOR_HOVER
            } else {
                MENU_ELEMENT_BACKGROUND_COLOR
            };

            match element {
                MenuElement::Button { text, .. } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    draw_outline();
                    let text_x = menu.pos.x + (default_rect.width as usize - 10 * text.len()) / 2;
                    draw_string(
                        text,
                        Point {
                            x: text_x as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        element_bg_color,
                    );
                }
                MenuElement::Slider { text_fn, value, .. } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    let text = text_fn(*value);
                    let cursor_width = 20;
                    let x_pos =
                        default_rect.x + (value * (menu.width - cursor_width - 4) as f32) as u16;
                    let text_x = menu.pos.x + (default_rect.width as usize - 10 * text.len()) / 2;
                    draw_string(
                        text.as_str(),
                        Point {
                            x: text_x as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        element_bg_color,
                    );
                    push_rect_uniform(
                        Rect {
                            x: x_pos + 2,
                            y: default_rect.y + 2,
                            width: 20,
                            height: default_rect.height - 4,
                        },
                        Color::from_888(255, 255, 255),
                    );
                    draw_outline();
                }
                MenuElement::Label {
                    text, text_anchor, ..
                } => {
                    let text_y = match text_anchor {
                        TextAnchor::Left => menu.pos.x + 10,
                        TextAnchor::Center => menu.pos.x + (menu.width - 10 * text.len()) / 2,
                        TextAnchor::Right => menu.pos.x + menu.width - 10 * text.len() - 10,
                    };
                    draw_string(
                        text,
                        Point {
                            x: text_y as u16,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        MENU_BACKGROUND_COLOR,
                    );
                }
                MenuElement::ButtonOption { text, .. } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    draw_outline();

                    draw_string(
                        text.as_str(),
                        Point {
                            x: default_rect.x + 10,
                            y: (element_y + 6) as u16,
                        },
                        true,
                        MENU_TEXT_COLOR,
                        element_bg_color,
                    );
                }
                MenuElement::Void { .. } => {}
                MenuElement::Entry {
                    placeholder_text,
                    value,
                    ..
                } => {
                    push_rect_uniform(default_rect, element_bg_color);
                    draw_outline();
                    let text_x = menu.pos.x + 10;
                    if value.is_empty() {
                        draw_string(
                            &placeholder_text,
                            Point {
                                x: text_x as u16,
                                y: (element_y + 6) as u16,
                            },
                            true,
                            MENU_TEXT_COLOR,
                            element_bg_color,
                        );
                    } else {
                        draw_string(
                            &value,
                            Point {
                                x: text_x as u16,
                                y: (element_y + 6) as u16,
                            },
                            true,
                            MENU_TEXT_COLOR,
                            element_bg_color,
                        );
                    }
                    push_rect_uniform(
                        Rect {
                            x: (text_x + value.len() * 10) as u16,
                            y: (element_y + 6) as u16,
                            width: 2,
                            height: 18,
                        },
                        MENU_TEXT_COLOR,
                    );
                }
            }

            element_y += if i < elements.len() - 1
                && matches!(&elements[i + 1], MenuElement::ButtonOption { .. })
            // keep the same y if we have a button option next to a button
            {
                0
            } else if matches!(
                // If the element needs margin, add and additional margin
                &element,
                MenuElement::Label {
                    allow_margin: true,
                    ..
                } | MenuElement::Button {
                    allow_margin: true,
                    ..
                } | MenuElement::Slider {
                    allow_margin: true,
                    ..
                } | MenuElement::Void {
                    allow_margin: true,
                    ..
                } | MenuElement::Entry {
                    allow_margin: true,
                    ..
                }
            ) {
                40
            } else if i > 0 // If the element is a button option and that the previous element is a button requesting for margin, add margin
                && matches!(
                    &elements[i - 1],
                    MenuElement::Button {
                        allow_margin: true,
                        ..
                    }
                )
            {
                40
            } else {
                30
            };
        }

        wait_for_vblank();
    }
}
