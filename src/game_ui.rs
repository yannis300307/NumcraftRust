 pub enum GameUI {
    /// A simple button
    Button {
        text: String,
        is_pressed: bool,
        id: usize,
        up_id: Option<usize>,
        down_id: Option<usize>,
        left_id: Option<usize>,
        right_id: Option<usize>,
    },
    Label {
        text: String,
    },
    ItemSlot {
        inventory: todo(),
        iventory_slot_index: usize,
        id: usize,
        up_id: Option<usize>,
        down_id: Option<usize>,
        left_id: Option<usize>,
        right_id: Option<usize>,
    }
}

struct GameUI {
    elements: Vec<MenuElement>,
    pub selected_index: usize,
    pub need_redraw: bool,
    pub blur_background: bool,
}