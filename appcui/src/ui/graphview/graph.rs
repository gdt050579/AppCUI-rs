use std::u32;
use std::usize;

use super::Edge;
use super::GraphNode;
use super::Node;
use super::NodeBuilder;
use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ControlState {
    Disabled,
    Normal,
    Focused,
}

impl ControlState {
    fn new(control: &ControlBase) -> Self {
        if !control.is_enabled() {
            ControlState::Disabled
        } else if control.has_focus() {
            ControlState::Focused
        } else {
            ControlState::Normal
        }
    }
    #[inline(always)]
    fn node_attr(&self, theme: &Theme) -> (CharAttribute, CharAttribute) {
        match self {
            ControlState::Disabled => (theme.button.text.inactive, theme.button.hotkey.inactive),
            ControlState::Normal | ControlState::Focused => (theme.button.text.normal, theme.button.hotkey.normal),
        }
    }
    #[inline(always)]
    fn hovered_node_attr(&self, theme: &Theme) -> (CharAttribute, CharAttribute) {
        (theme.button.text.hovered, theme.button.hotkey.hovered)
    }
    #[inline(always)]
    fn current_node_attr(&self, theme: &Theme) -> (CharAttribute, CharAttribute) {
        (theme.button.text.pressed_or_selectd, theme.button.hotkey.pressed_or_selectd)
    }
}

pub struct Graph<T>
where
    T: GraphNode,
{
    pub(super) nodes: Vec<Node<T>>,
    pub(super) edges: Vec<Edge>,
    surface_size: Size,
    surface: Surface,
    current_node: usize,
    hovered_node: Option<usize>,
    repr_buffer: String,
}
impl<T> Graph<T>
where
    T: GraphNode,
{
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge>) -> Self {
        Self {
            nodes,
            edges,
            surface_size: Size::new(1, 1),
            surface: Surface::new(200, 200),
            current_node: 0,
            hovered_node: None,
            repr_buffer: String::with_capacity(128),
        }
    }
    pub fn with_slices(nodes: &[T], edges: &[(u32, u32)]) -> Self
    where
        T: GraphNode + Clone,
    {
        let v: Vec<Node<T>> = nodes.iter().map(|n| NodeBuilder::new(n.clone()).build()).collect();
        let e: Vec<Edge> = edges
            .iter()
            .map(|link| Edge {
                from_node_id: link.0,
                to_node_id: link.1,
            })
            .collect();
        Self::new(v, e)
    }
    pub fn with_slices_and_border(nodes: &[T], edges: &[(u32, u32)], border: LineType) -> Self
    where
        T: GraphNode + Clone,
    {
        let v: Vec<Node<T>> = nodes.iter().map(|n| NodeBuilder::new(n.clone()).border(border).build()).collect();
        let e: Vec<Edge> = edges
            .iter()
            .map(|link| Edge {
                from_node_id: link.0,
                to_node_id: link.1,
            })
            .collect();
        Self::new(v, e)
    }
    fn update_surface_size(&mut self) {
        if self.nodes.is_empty() {
            self.surface_size = Size::new(1, 1);
            return;
        }
        let r = self.nodes[0].rect;
        let mut tl = r.top_left();
        let mut br = r.bottom_right();
        for n in &self.nodes {
            tl.x = tl.x.min(n.rect.left());
            tl.y = tl.y.min(n.rect.top());
            br.x = br.x.max(n.rect.right());
            br.y = br.y.max(n.rect.bottom());
            //log!("G","  R = {:?}",n.rect);
        }
        let dx = 2 - tl.x; // one character on X-axes
        let dy = 1 - tl.y; // two character on Y-axes
                           //log!("G","dx={}, dy={}, TL = {:?}, BR = {:?}",dx,dy, tl, br);
        for n in &mut self.nodes {
            n.rect += (dx, dy);
            //log!("G","  R = {:?}",n.rect);
        }
        // 4 extra ccharacters on left / right (two on left, tow on right)
        // 2 extra on top-bottom (1 on top, 1 on bottom)
        self.surface_size = Size::new(((br.x - tl.x + 1 + 4) as u32).max(1), ((br.y - tl.y + 1 + 2) as u32).max(1));
        //log!("G","New size = {:?}",self.surface_size);
    }

    pub(super) fn resize_to_fit(&mut self) {
        self.update_surface_size();
        self.surface.resize(self.surface_size);
    }

    #[inline(always)]
    pub(super) fn size(&self) -> Size {
        self.surface_size
    }
    pub(super) fn mouse_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        for (idx, n) in self.nodes.iter().enumerate() {
            if n.contains(x, y) {
                return Some(idx);
            }
        }
        None
    }
    pub(super) fn repaint(&mut self, control: &ControlBase) {
        // clear the entire surface
        let ch = Character::new(' ', Color::Transparent, Color::Transparent, CharFlags::None);
        for c in &mut self.surface.chars {
            *c = ch;
        }
        let state = ControlState::new(control);
        let theme = control.theme();
        let (text_attr, border_attr) = state.node_attr(theme);
        // draw nodes
        for node in &self.nodes {
            self.repr_buffer.clear();
            if state == ControlState::Focused {
                node.paint(
                    &mut self.surface,
                    node.text_attr.unwrap_or(text_attr),
                    node.border_attr.unwrap_or(border_attr),
                    &mut self.repr_buffer,
                );
            } else {
                node.paint(&mut self.surface, text_attr, border_attr, &mut self.repr_buffer);
            };
        }
        // draw hover node (if case)
        let len = self.nodes.len();
        let hover_node_id = self.hovered_node.unwrap_or(usize::MAX);
        if (state != ControlState::Disabled) && (hover_node_id < len) {
            let node = &self.nodes[hover_node_id];
            let (t, b) = state.hovered_node_attr(theme);
            node.paint(&mut self.surface, t, b, &mut self.repr_buffer);
        }
        if (state == ControlState::Focused) && (self.current_node < len) {
            let node = &self.nodes[self.current_node];
            let (t, b) = state.current_node_attr(theme);
            node.paint(&mut self.surface, t, b, &mut self.repr_buffer);
        }
    }
    pub(super) fn paint_node(&mut self, control: &ControlBase, index: usize) {
        let len = self.nodes.len();
        if index >= len {
            return;
        }
        let state = ControlState::new(control);
        let theme = control.theme();
        let (t, b) = match state {
            ControlState::Disabled => state.node_attr(theme),
            ControlState::Normal => {
                let hover_node_id = self.hovered_node.unwrap_or(usize::MAX);
                if hover_node_id == index {
                    state.hovered_node_attr(theme)
                } else {
                    state.node_attr(theme)
                }
            }
            ControlState::Focused => {
                if index == self.current_node {
                    state.current_node_attr(theme)
                } else {
                    let hover_node_id = self.hovered_node.unwrap_or(usize::MAX);
                    if hover_node_id == index {
                        state.hovered_node_attr(theme)
                    } else {
                        state.node_attr(theme)
                    }
                }
            }
        };
        let node = &self.nodes[index];
        node.paint(&mut self.surface, t, b, &mut self.repr_buffer);
    }
    pub(super) fn reset_hover(&mut self, control: &ControlBase) {
        let index = self.hovered_node.unwrap_or(usize::MAX);
        if self.hovered_node.is_some() {
            self.hovered_node = None;
            self.paint_node(control, index);
        }
    }
    pub(super) fn process_mouse_over(&mut self, control: &ControlBase, point: Point) -> EventProcessStatus {
        let new_idx = self.mouse_pos_to_index(point.x, point.y);
        if new_idx == self.hovered_node {
            return EventProcessStatus::Ignored;
        }
        // first clear the existing one
        self.reset_hover(control);
        self.hovered_node = new_idx;
        if let Some(idx) = new_idx {
            self.paint_node(control, idx);
        }
        EventProcessStatus::Processed
    }
    pub(super) fn surface(&self) -> &Surface {
        &self.surface
    }
}

impl<T> Default for Graph<T>
where
    T: GraphNode,
{
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
            surface_size: Default::default(),
            surface: Surface::new(1, 1),
            current_node: 0,
            hovered_node: None,
            repr_buffer: String::new(),
        }
    }
}
