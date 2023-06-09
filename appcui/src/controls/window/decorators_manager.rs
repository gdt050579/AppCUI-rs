use super::{Decorator, DecoratorPaintData, DecoratorType};
use crate::{
    graphics::{Size, Surface},
    system::Theme,
    utils::VectorIndex,
};

struct PositionHelper {
    x: i32,
    y: i32,
    index: VectorIndex,
    decoraror_type: DecoratorType,
}
impl PositionHelper {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            index: VectorIndex::default(),
            decoraror_type: DecoratorType::None,
        }
    }
}

const MAX_TAG_CHARS: usize = 8;
pub(super) struct DecoratorsManager {
    items: Vec<Decorator>,
    current: VectorIndex,
    pressed: bool,
}

impl DecoratorsManager {
    pub(super) fn new() -> Self {
        Self {
            items: Vec::with_capacity(4),
            current: VectorIndex::Invalid,
            pressed: false,
        }
    }
    pub(super) fn add(&mut self, decorator: Decorator) {
        self.items.push(decorator);
    }
    #[inline(always)]
    pub(super) fn get(&self, index: usize) -> Option<&Decorator> {
        self.items.get(index)
    }
    #[inline(always)]
    pub(super) fn get_mut(&mut self, index: usize) -> Option<&mut Decorator> {
        self.items.get_mut(index)
    }
    pub(super) fn position_to_decorator(&self, x: i32, y: i32) -> Option<usize> {
        for (index, item) in self.items.iter().enumerate() {
            if item.contains(x, y) {
                return Some(index);
            }
        }
        None
    }
    pub(super) fn get_from_position(&self, x: i32, y: i32) -> Option<(usize, &Decorator)> {
        for (index, item) in self.items.iter().enumerate() {
            if item.contains(x, y) {
                return Some((index, item));
            }
        }
        None
    }
    pub(super) fn get_index_from_position(&self, x: i32, y: i32) -> Option<usize> {
        for (index, item) in self.items.iter().enumerate() {
            if item.contains(x, y) {
                return Some(index);
            }
        }
        None
    }
    pub(super) fn paint(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        focused: bool,
        maximized: bool,
    ) {
        let mut paint_data = DecoratorPaintData {
            focused,
            current: false,
            maximized,
            is_current_item_pressed: self.pressed,
            sep_attr: if focused {
                theme.lines.normal
            } else {
                theme.lines.inactive
            },
        };
        let current_bar_index = self.current.index();
        // paint bar items
        for (index, item) in self.items.iter().enumerate() {
            paint_data.current = index == current_bar_index;
            item.paint(surface, theme, &paint_data);
        }
    }
    fn update_position_from_left(&mut self, index: usize, pos: &mut PositionHelper, right: i32) {
        let d = &mut self.items[index];
        let (next, add_flags) = d.update_position_from_left(pos.x, pos.y, pos.decoraror_type);
        let last_index = pos.index;
        if next < right {
            d.set_visible();
            pos.index = VectorIndex::with_value(index);
            pos.x = next;
            pos.decoraror_type = d.get_type();
        }
        if add_flags && last_index.is_valid() {
            self.items[last_index.index()].set_right_marker();
        }
    }
    fn update_position_from_right(&mut self, index: usize, pos: &mut PositionHelper, left: i32) {
        let d = &mut self.items[index];
        let (next, add_flags) = d.update_position_from_right(pos.x, pos.y, pos.decoraror_type);
        let last_index = pos.index;
        if next > left {
            d.set_visible();
            pos.index = VectorIndex::with_value(index);
            pos.x = next;
            pos.decoraror_type = d.get_type();
        }
        if add_flags && last_index.is_valid() {
            self.items[last_index.index()].set_left_marker();
        }
    }
    pub(super) fn update_positions(&mut self, size: Size) -> (i32, i32) {
        // clear all flags (visible & left|right marker)
        for d in &mut self.items {
            d.clear();
        }
        let mut top_left = PositionHelper::new(1, 0);
        let mut top_right = PositionHelper::new((size.width as i32) - 2, 0);
        let mut bottom_left = PositionHelper::new(1, (size.height as i32) - 1);
        let mut bottom_right =
            PositionHelper::new((size.width as i32) - 1, (size.height as i32) - 1);
        let count = self.items.len();

        for index in 0..count {
            let d = &self.items[index];
            if d.is_hidden() {
                continue;
            }
            let layout = d.get_layout();
            match layout {
                super::decorator::DecoratorLayout::TopLeft => {
                    self.update_position_from_left(index, &mut top_left, top_right.x);
                }
                super::decorator::DecoratorLayout::BottomLeft => {
                    self.update_position_from_left(index, &mut bottom_left, bottom_right.x)
                }
                super::decorator::DecoratorLayout::TopRight => {
                    self.update_position_from_right(index, &mut top_right, top_left.x);
                }
                super::decorator::DecoratorLayout::BottomRight => {
                    self.update_position_from_right(index, &mut bottom_right, bottom_left.x);
                }
            }
        }

        // last elements
        if top_left.index.is_valid() {
            self.items[top_left.index.index()].set_right_marker();
        }
        if bottom_left.index.is_valid() {
            self.items[bottom_left.index.index()].set_right_marker();
        }
        if top_right.index.is_valid() {
            self.items[top_right.index.index()].set_left_marker();
        }
        if bottom_right.index.is_valid() {
            self.items[bottom_right.index.index()].set_left_marker();
        }
        //let title_x_pos = top_left.x + 1;
        //let title_space = (top_right.x - title_x_pos).max(0);
        //(title_x_pos, title_space as u16)
        (top_left.x+1,top_right.x)
    }
    pub(super) fn set_tag(&mut self, name: &str) {
        for d in &mut self.items {
            if d.get_type() == DecoratorType::Tag {
                if name.len() == 0 {
                    d.hide();
                    break;
                }
                if name.len() > MAX_TAG_CHARS {
                    d.set_text(&name[..MAX_TAG_CHARS], false);
                } else {
                    d.set_text(name, false);
                }
                d.set_tooltip(name);
                d.unhide();
                break;
            }
        }
    }
    pub(super) fn get_tag(&self) -> Option<&str> {
        for d in &self.items {
            if d.get_type() == DecoratorType::Tag {
                if d.is_hidden() {
                    return None;
                }
                let tag_name = d.get_text();
                if tag_name.len() == 0 {
                    return None;
                }
                return Some(tag_name);
            }
        }
        None
    }
    pub(super) fn set_current(&mut self, idx: VectorIndex) {
        self.current = idx;
    }
    pub(super) fn get_current(&self) -> VectorIndex {
        self.current
    }
    pub(super) fn is_current_item_pressed(&self) -> bool {
        self.pressed
    }
    pub(super) fn set_current_item_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }
    pub(super) fn check_singlechoice(&mut self, idx: usize) {
        if idx >= self.items.len() {
            return;
        }
        if self.items[idx].get_type() != DecoratorType::SingleChoice {
            return;
        }
        let count = self.items.len();
        let mut end_index = idx;
        while (end_index < count)
            && (self.items[end_index].get_type() == DecoratorType::SingleChoice)
        {
            end_index += 1;
        }
        let mut start_index = idx;
        while (start_index > 0)
            && (self.items[start_index].get_type() == DecoratorType::SingleChoice)
        {
            start_index -= 1;
        }
        if start_index > 0 {
            start_index += 1;
        } else {
            if self.items[start_index].get_type() != DecoratorType::SingleChoice {
                start_index += 1;
            }
        }
        for i in start_index..end_index {
            self.items[i].set_checked(false);
        }
        self.items[idx].set_checked(true);
    }
}
