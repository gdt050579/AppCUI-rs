use std::u32;
use std::u64;
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

enum Direction {
    OnLeft,
    OnRight,
    OnTop,
    OnBottom,
}
impl Direction {
    fn compare_point(&self, rect: &Rect) -> Point {
        match self {
            Direction::OnLeft => Point::new(rect.left(), rect.center_y()),
            Direction::OnRight => Point::new(rect.right(), rect.center_y()),
            Direction::OnTop => Point::new(rect.center_x(), rect.top()),
            Direction::OnBottom => Point::new(rect.center_x(), rect.bottom()),
        }
    }
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
    fn node_attr(&self, theme: &Theme) -> CharAttribute {
        match self {
            ControlState::Disabled => theme.button.text.inactive,
            ControlState::Normal | ControlState::Focused => theme.button.text.normal,
        }
    }
    #[inline(always)]
    fn edge_attr(&self, theme: &Theme) -> CharAttribute {
        match self {
            ControlState::Disabled => theme.lines.inactive,
            ControlState::Normal => theme.lines.normal,
            ControlState::Focused => theme.lines.focused,
        }
    }
    #[inline(always)]
    fn hovered_node_attr(&self, theme: &Theme) -> CharAttribute {
        theme.button.text.hovered
    }
    #[inline(always)]
    fn current_node_attr(&self, theme: &Theme) -> CharAttribute {
        theme.button.text.focused
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
        let text_attr = state.node_attr(theme);
        let edge_attr = state.edge_attr(theme);
        // draw edges
        for e in &self.edges {
            let p1 = self.nodes[e.from_node_id as usize].rect.center();
            let p2 = self.nodes[e.to_node_id as usize].rect.center();
            self.surface
                .draw_orthogonal_line(p1.x, p1.y, p2.x, p2.y, LineType::Single, OrthogonalDirection::Auto, edge_attr);
        }
        // draw nodes
        for node in &self.nodes {
            self.repr_buffer.clear();
            if state == ControlState::Focused {
                node.paint(&mut self.surface, node.text_attr.unwrap_or(text_attr), &mut self.repr_buffer);
            } else {
                node.paint(&mut self.surface, text_attr, &mut self.repr_buffer);
            };
        }
        // draw hover node (if case)
        let len = self.nodes.len();
        let hover_node_id = self.hovered_node.unwrap_or(usize::MAX);
        if (state != ControlState::Disabled) && (hover_node_id < len) {
            let node = &self.nodes[hover_node_id];
            let attr = state.hovered_node_attr(theme);
            self.repr_buffer.clear();
            node.paint(&mut self.surface, attr, &mut self.repr_buffer);
        }
        if (state == ControlState::Focused) && (self.current_node < len) {
            let node = &self.nodes[self.current_node];
            let attr = state.current_node_attr(theme);
            self.repr_buffer.clear();
            node.paint(&mut self.surface, attr, &mut self.repr_buffer);
        }
    }
    pub(super) fn paint_node(&mut self, control: &ControlBase, index: usize) {
        let len = self.nodes.len();
        if index >= len {
            return;
        }
        let state = ControlState::new(control);
        let theme = control.theme();
        let attr = match state {
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
        self.repr_buffer.clear();
        node.paint(&mut self.surface, attr, &mut self.repr_buffer);
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
    pub(super) fn move_node_to(&mut self, id: usize, x: i32, y: i32, control: &ControlBase) -> bool {
        if id >= self.nodes.len() {
            return false;
        }
        let node = &mut self.nodes[id];
        let tl = node.rect.top_left();
        if (tl.x == x) && (tl.y == y) {
            return false;
        }
        node.rect.set_left(x);
        node.rect.set_top(y);
        let mut resized = false;
        if node.rect.right() >= (self.surface_size.width as i32) || node.rect.bottom() >= (self.surface_size.height as i32) || x < 0 || y < 0 {
            // need to resize the surface
            self.resize_to_fit();
            resized = true;
        }
        self.repaint(control);
        resized
    }
    fn move_node_with(&mut self, id: usize, dx: i32, dy: i32, control: &ControlBase) -> bool {
        if id >= self.nodes.len() {
            return false;
        }
        let tl = self.nodes[id].rect.top_left();
        self.move_node_to(id, tl.x + dx, tl.y + dy, control)
    }

    pub(super) fn set_current_node(&mut self, index: usize, control: &ControlBase) {
        if index != self.current_node {
            let old_index = self.current_node;
            self.current_node = index;
            self.paint_node(control, old_index);
            self.paint_node(control, index);
        }
    }

    fn next_node(&self, dir: Direction) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }
        let r = self.nodes[self.current_node].rect;
        let c = dir.compare_point(&r);
        let mut best = None;
        let mut best_dist = u64::MAX;
        for (index, n) in self.nodes.iter().enumerate() {
            if index == self.current_node {
                continue;
            }
            let dp = match dir {
                Direction::OnLeft => {
                    if r.right() > n.rect.right() {
                        Some(Direction::OnRight.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::OnRight => {
                    if r.left() < n.rect.left() {
                        Some(Direction::OnLeft.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::OnTop => {
                    if r.bottom() > n.rect.bottom() {
                        Some(Direction::OnBottom.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::OnBottom => {
                    if r.top() < n.rect.top() {
                        Some(Direction::OnTop.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
            };
            if let Some(nc) = dp {
                let dist = ((nc.x - c.x) * (nc.x - c.x)) as u64 + ((nc.y - c.y) * (nc.y - c.y)) as u64;
                if dist < best_dist {
                    best = Some(index);
                    best_dist = dist;
                }
            }
        }
        best
    }
    fn move_to_node_with_direction(&mut self, dir: Direction, control: &ControlBase) -> bool {
        if let Some(next_index) = self.next_node(dir) {
            self.set_current_node(next_index, control);
            true
        } else {
            false
        }
    }

    pub(super) fn process_key_events(&mut self, key: Key, control: &ControlBase) -> bool {
        match key.value() {
            key!("Left") => self.move_to_node_with_direction(Direction::OnLeft, control),
            key!("Right") => self.move_to_node_with_direction(Direction::OnRight, control),
            key!("Up") => self.move_to_node_with_direction(Direction::OnTop, control),
            key!("Down") => self.move_to_node_with_direction(Direction::OnBottom, control),
            key!("Ctrl+Left") => self.move_node_with(self.current_node, -1, 0, control),
            key!("Ctrl+Right") => self.move_node_with(self.current_node, 1, 0, control),
            key!("Ctrl+Up") => self.move_node_with(self.current_node, 0, -1, control),
            key!("Ctrl+Down") => self.move_node_with(self.current_node, 0, 1, control),
            key!("Ctrl+Tab") => {
                if self.nodes.len() > 0 {
                    self.set_current_node((self.current_node + 1) % self.nodes.len(), control);
                }
                true
            }
            key!("Ctrl+Shift+Tab") => {
                if self.nodes.len() > 0 {
                    self.set_current_node((self.current_node + self.nodes.len() - 1) % self.nodes.len(), control);
                }
                true
            }
            _ => false,
        }
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
