use super::Edge;
use super::EdgeBuilder;
use super::EdgeRouting;
use super::GraphNode;
use super::Node;
use super::NodeBuilder;
use super::RenderingOptions;
use crate::prelude::*;
use crate::utils::GlyphParser;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ControlState {
    Disabled,
    Normal,
    Focused,
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}
impl Direction {
    fn compare_point(&self, rect: &Rect) -> Point {
        match self {
            Direction::Left => Point::new(rect.left(), rect.center_y()),
            Direction::Right => Point::new(rect.right(), rect.center_y()),
            Direction::Top => Point::new(rect.center_x(), rect.top()),
            Direction::Bottom => Point::new(rect.center_x(), rect.bottom()),
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
            ControlState::Disabled => theme.text.inactive,
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

fn closest_points(r1: &Rect, r2: &Rect) -> (Point, Point, OrthogonalDirection, Option<Direction>) {
    let h = if r1.right() + 2 <= r2.left() {
        2
    } else if r1.left() >= r2.right() + 2 {
        1
    } else {
        0
    };
    let v = if r1.bottom() + 2 <= r2.top() {
        2
    } else if r1.top() >= r2.bottom() + 2 {
        1
    } else {
        0
    };
    let value: u8 = (h << 2) | v;
    // format : 0b_hh_vv
    // h = 0 => center_x
    // h = 1 => on left of r1 (r1_left to r2_right)
    // h = 2 => on right of r1 (r1_right to r2_left)
    // v = 0 => center_y
    // v = 1 => on top of r1 (r1_top to r2_bottom)
    // v = 2 => on bottom of r1 (r1_bottom to r2_top)
    match value {
        0 => {
            // intersect
            (r1.center(), r2.center(), OrthogonalDirection::Auto, None)
        }
        0b_00_01 => {
            // h = 0, v = 1
            (
                Point::new(r1.center_x(), r1.top()),
                Point::new(r2.center_x(), r2.bottom()),
                OrthogonalDirection::VerticalUntilMiddle,
                Some(Direction::Bottom),
            )
        }
        0b_00_10 => {
            // h = 0, v = 2
            (
                Point::new(r1.center_x(), r1.bottom()),
                Point::new(r2.center_x(), r2.top()),
                OrthogonalDirection::VerticalUntilMiddle,
                Some(Direction::Top),
            )
        }
        0b_01_00 => {
            // h = 1, v = 0
            (
                Point::new(r1.left(), r1.center_y()),
                Point::new(r2.right(), r2.center_y()),
                OrthogonalDirection::HorizontalUntilMiddle,
                Some(Direction::Right),
            )
        }
        0b_01_01 => {
            // h = 1, v = 1
            if (r1.left() - r2.right()).abs() < (r1.top() - r2.bottom()).abs() {
                // more to the left
                (
                    Point::new(r1.left(), r1.center_y()),
                    Point::new(r2.right(), r2.center_y()),
                    OrthogonalDirection::HorizontalUntilMiddle,
                    Some(Direction::Right),
                )
            } else {
                // more to the top
                (
                    Point::new(r1.center_x(), r1.top()),
                    Point::new(r2.center_x(), r2.bottom()),
                    OrthogonalDirection::VerticalUntilMiddle,
                    Some(Direction::Bottom),
                )
            }
        }
        0b_01_10 => {
            // h = 1, v = 2
            if (r1.left() - r2.right()).abs() < (r1.bottom() - r2.top()).abs() {
                // more to the left
                (
                    Point::new(r1.left(), r1.center_y()),
                    Point::new(r2.right(), r2.center_y()),
                    OrthogonalDirection::HorizontalUntilMiddle,
                    Some(Direction::Right),
                )
            } else {
                // more to the bottom
                (
                    Point::new(r1.center_x(), r1.bottom()),
                    Point::new(r2.center_x(), r2.top()),
                    OrthogonalDirection::VerticalUntilMiddle,
                    Some(Direction::Top),
                )
            }
        }
        0b_10_00 => {
            // h = 2, v = 0
            (
                Point::new(r1.right(), r1.center_y()),
                Point::new(r2.left(), r2.center_y()),
                OrthogonalDirection::HorizontalUntilMiddle,
                Some(Direction::Left),
            )
        }
        0b_10_01 => {
            // h = 2, v = 1
            if (r1.right() - r2.left()).abs() < (r1.top() - r2.bottom()).abs() {
                // more to the right
                (
                    Point::new(r1.right(), r1.center_y()),
                    Point::new(r2.left(), r2.center_y()),
                    OrthogonalDirection::HorizontalUntilMiddle,
                    Some(Direction::Left),
                )
            } else {
                // more to the top
                (
                    Point::new(r1.center_x(), r1.top()),
                    Point::new(r2.center_x(), r2.bottom()),
                    OrthogonalDirection::VerticalUntilMiddle,
                    Some(Direction::Bottom),
                )
            }
        }
        0b_10_10 => {
            // h = 2, v = 2
            if (r1.right() - r2.left()).abs() < (r1.bottom() - r2.top()).abs() {
                // more to the right
                (
                    Point::new(r1.right(), r1.center_y()),
                    Point::new(r2.left(), r2.center_y()),
                    OrthogonalDirection::HorizontalUntilMiddle,
                    Some(Direction::Left),
                )
            } else {
                // more to the bottom
                (
                    Point::new(r1.center_x(), r1.bottom()),
                    Point::new(r2.center_x(), r2.top()),
                    OrthogonalDirection::VerticalUntilMiddle,
                    Some(Direction::Top),
                )
            }
        }
        _ => {
            unreachable!("The combination {value} [h={h}, v={v}] is not possible !");
        }
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
    rendering_options: RenderingOptions,
}
impl<T> Graph<T>
where
    T: GraphNode,
{
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge>) -> Self {
        let mut g = Self {
            nodes,
            edges,
            surface_size: Size::new(1, 1),
            surface: Surface::new(200, 200),
            current_node: 0,
            hovered_node: None,
            repr_buffer: String::with_capacity(128),
            rendering_options: RenderingOptions::new(),
        };
        // remove edges that have invalid node index value
        let nodes_count = g.nodes.len() as u32;
        g.edges.retain(|e| (e.from_node_id < nodes_count) && (e.to_node_id < nodes_count));
        // build edges_in / edges_out for each node
        for (idx, e) in g.edges.iter().enumerate() {
            if e.directed {
                g.nodes[e.from_node_id as usize].edges_out.push(idx as u32);
                g.nodes[e.to_node_id as usize].edges_in.push(idx as u32);
            } else {
                g.nodes[e.from_node_id as usize].edges_out.push(idx as u32);
                g.nodes[e.from_node_id as usize].edges_in.push(idx as u32);
                g.nodes[e.to_node_id as usize].edges_out.push(idx as u32);
                g.nodes[e.to_node_id as usize].edges_in.push(idx as u32);
            }
        }
        g
    }
    pub fn with_slices(nodes: &[T], edges: &[(u32, u32)], directed: bool) -> Self
    where
        T: GraphNode + Clone,
    {
        let v: Vec<Node<T>> = nodes.iter().map(|n| NodeBuilder::new(n.clone()).build()).collect();
        let e: Vec<Edge> = edges
            .iter()
            .map(|link| EdgeBuilder::new(link.0, link.1).directed(directed).build())
            .collect();
        Self::new(v, e)
    }
    pub fn with_slices_and_border(nodes: &[T], edges: &[(u32, u32)], border: LineType, directed: bool) -> Self
    where
        T: GraphNode + Clone,
    {
        let v: Vec<Node<T>> = nodes.iter().map(|n| NodeBuilder::new(n.clone()).border(border).build()).collect();
        let e: Vec<Edge> = edges
            .iter()
            .map(|link| EdgeBuilder::new(link.0, link.1).directed(directed).build())
            .collect();
        Self::new(v, e)
    }

    /// Returns the object (of type T) associated with the current node
    /// This method returns None only if th graph is empty (no nodes) otherwise it always returns Some(&T)
    pub fn current_node(&self) -> Option<&Node<T>> {
        if self.current_node < self.nodes.len() {
            Some(&self.nodes[self.current_node])
        } else {
            None
        }
    }

    /// Returns the number of nodes in the graph
    pub fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a object associated with a node in the graph by its index (if the index is valid)
    pub fn node(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    fn update_surface_size(&mut self, pack: bool) {
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
        if pack {
            for n in &mut self.nodes {
                n.rect += (dx, dy);
            }
            // 4 extra ccharacters on left / right (two on left, tow on right)
            // 2 extra on top-bottom (1 on top, 1 on bottom)
            self.surface_size = Size::new(((br.x - tl.x + 1 + 4) as u32).max(1), ((br.y - tl.y + 1 + 2) as u32).max(1));
        } else {
            let dx = dx.max(0);
            let dy = dy.max(0);
            if dx > 0 || dy > 0 {
                for n in &mut self.nodes {
                    n.rect += (dx, dy);
                }
            }
            self.surface_size = Size::new(((br.x + (dx - 2) + 1 + 4) as u32).max(1), ((br.y + (dy - 1) + 1 + 2) as u32).max(1));
        }
        //log!("G","New size = {:?}",self.surface_size);
    }

    pub(super) fn resize_graph(&mut self, pack: bool) {
        self.update_surface_size(pack);
        self.surface.resize(self.surface_size);
    }

    pub(super) fn clear_filter(&mut self, control: &ControlBase) {
        let mut need_repaint = false;
        for n in &mut self.nodes {
            if n.filtered {
                n.filtered = false;
                need_repaint = true;
            }
        }
        if need_repaint {
            self.repaint(control);
        }
    }
    pub(super) fn filter(&mut self, text: &str, control: &ControlBase) -> usize {
        if text.is_empty() {
            self.clear_filter(control);
            return self.nodes.len();
        }
        let mut need_repaint = false;
        let mut count = 0;
        for n in &mut self.nodes {
            self.repr_buffer.clear();
            if n.obj.write_label(&mut self.repr_buffer, Size::new(u32::MAX, u32::MAX)).is_ok() {
                if self.repr_buffer.index_ignoring_case(text).is_some() {
                    if n.filtered {
                        n.filtered = false;
                        need_repaint = true;
                    }
                    count += 1;
                } else if !n.filtered {
                    n.filtered = true;
                    need_repaint = true;
                }
            } else if !n.filtered {
                n.filtered = true;
                need_repaint = true;
            }
        }
        // check to see if the current node is filtered
        if (count > 0) && (self.current_node < self.nodes.len()) && self.nodes[self.current_node].filtered {
            let len = self.nodes.len();
            for i in 0..self.nodes.len() {
                let idx = (self.current_node + i) % len;
                if !self.nodes[idx].filtered {
                    self.current_node = idx;
                    break;
                    // no need to repaint here as we wil do it later
                }
            }
            need_repaint = true;
        }
        if need_repaint {
            self.repaint(control);
        }
        count
    }
    pub(super) fn goto_next_match(&mut self, control: &ControlBase) {
        if self.nodes.is_empty() {
            return;
        }
        let len = self.nodes.len();
        for i in 1..=len {
            let idx = (self.current_node + i) % len;
            if !self.nodes[idx].filtered {
                self.set_current_node(idx, control);
                break;
            }
        }
    }
    pub(super) fn goto_previous_match(&mut self, control: &ControlBase) {
        if self.nodes.is_empty() {
            return;
        }
        let len = self.nodes.len();
        for i in 1..=len {
            let idx = (self.current_node + len - i) % len;
            if !self.nodes[idx].filtered {
                self.set_current_node(idx, control);
                break;
            }
        }
    }

    #[inline(always)]
    pub(super) fn size(&self) -> Size {
        self.surface_size
    }
    pub(super) fn update_rendering_options(&mut self, new_options: &RenderingOptions, control: &ControlBase) {
        if self.rendering_options != *new_options {
            self.rendering_options = *new_options;
            self.repaint(control);
        }
    }
    pub(super) fn mouse_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        for (idx, n) in self.nodes.iter().enumerate() {
            if n.contains(x, y) {
                return Some(idx);
            }
        }
        None
    }
    pub(super) fn current_node_id(&self) -> Option<usize> {
        if self.current_node < self.nodes.len() {
            Some(self.current_node)
        } else {
            None
        }
    }
    pub(super) fn hovered_node_id(&self) -> Option<usize> {
        self.hovered_node
    }
    fn draw_edge(&mut self, index: u32, attr: CharAttribute) {
        let e = &self.edges[index as usize];

        // let p1 = self.nodes[e.from_node_id as usize].rect.center();
        // let p2 = self.nodes[e.to_node_id as usize].rect.center();
        let (p1, p2, orto_dir, entry_dir) = closest_points(&self.nodes[e.from_node_id as usize].rect, &self.nodes[e.to_node_id as usize].rect);
        let line_type = e.line_type.unwrap_or(self.rendering_options.edge_line_type);
        match self.rendering_options.edge_routing {
            EdgeRouting::Direct => self.surface.draw_line(p1.x, p1.y, p2.x, p2.y, line_type, attr),
            EdgeRouting::Orthogonal => self.surface.draw_orthogonal_line(p1.x, p1.y, p2.x, p2.y, line_type, orto_dir, attr),
        }
        if e.directed && self.rendering_options.show_arrow_heads {
            match entry_dir {
                Some(Direction::Left) => self.surface.write_char(p2.x - 1, p2.y, Character::with_char(SpecialChar::TriangleRight)),
                Some(Direction::Right) => self.surface.write_char(p2.x + 1, p2.y, Character::with_char(SpecialChar::TriangleLeft)),
                Some(Direction::Top) => self.surface.write_char(p2.x, p2.y - 1, Character::with_char(SpecialChar::TriangleDown)),
                Some(Direction::Bottom) => self.surface.write_char(p2.x, p2.y + 1, Character::with_char(SpecialChar::TriangleUp)),
                None => (),
            }
        }
    }
    fn draw_edges_from_current(&mut self, attr: CharAttribute) {
        if self.current_node >= self.nodes.len() {
            return;
        }
        let len = self.nodes[self.current_node].edges_out.len();
        for i in 0..len {
            let index = self.nodes[self.current_node].edges_out[i];
            self.draw_edge(index, attr);
        }
    }
    fn draw_edges_to_current(&mut self, attr: CharAttribute) {
        if self.current_node >= self.nodes.len() {
            return;
        }
        let len = self.nodes[self.current_node].edges_in.len();
        for i in 0..len {
            let index = self.nodes[self.current_node].edges_in[i];
            self.draw_edge(index, attr);
        }
    }
    pub(super) fn repaint(&mut self, control: &ControlBase) {
        // clear the entire surface
        let ch = Character::new(0 as char, Color::Transparent, Color::Transparent, CharFlags::None);
        for c in &mut self.surface.chars {
            *c = ch;
        }
        let state = ControlState::new(control);
        let theme = control.theme();
        let text_attr = state.node_attr(theme);
        let edge_attr = state.edge_attr(theme);
        // draw all edges
        let len = self.edges.len() as u32;
        if state == ControlState::Focused {
            for index in 0..len {
                self.draw_edge(index, self.edges[index as usize].attribute.unwrap_or(edge_attr));
            }
        } else {
            for index in 0..len {
                self.draw_edge(index, edge_attr);
            }
        }
        if (state == ControlState::Focused) && (self.current_node < self.nodes.len()) {
            let attr = theme.lines.hovered;
            if self.rendering_options.highlight_edges_out {
                self.draw_edges_from_current(attr);
            }
            if self.rendering_options.highlight_edges_in {
                self.draw_edges_to_current(attr);
            }
        }
        // draw nodes
        for node in &self.nodes {
            self.repr_buffer.clear();
            if state == ControlState::Focused {
                let attr = if node.filtered {
                    ControlState::Disabled.node_attr(theme)
                } else {
                    node.text_attr.unwrap_or(text_attr)
                };
                node.paint(&mut self.surface, attr, &mut self.repr_buffer);
            } else {
                node.paint(&mut self.surface, text_attr, &mut self.repr_buffer);
            };
        }
        // draw nodes related to current_node
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
        let node = &self.nodes[index];
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
                    } else if node.filtered {
                        ControlState::Disabled.node_attr(theme)
                    } else {
                        node.text_attr.unwrap_or(state.node_attr(theme))
                    }
                }
            }
        };
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
    pub(super) fn process_mouse_over(&mut self, control: &ControlBase, point: Point) -> bool {
        let new_idx = self.mouse_pos_to_index(point.x, point.y);
        if new_idx == self.hovered_node {
            return false;
        }
        // first clear the existing one
        self.reset_hover(control);
        self.hovered_node = new_idx;
        if let Some(idx) = new_idx {
            self.paint_node(control, idx);
        }
        true
    }
    pub(super) fn surface(&self) -> &Surface {
        &self.surface
    }
    
    // returns TRUE if the size was resized to adjust the scrolling bars
    pub(super) fn move_node_to(&mut self, id: usize, x: i32, y: i32, control: &ControlBase) -> bool {
        if id >= self.nodes.len() {
            return false;
        }
        let node = &mut self.nodes[id];
        let tl = node.rect.top_left();
        if (tl.x == x) && (tl.y == y) {
            return false;
        }
        node.rect.set_left(x, true);
        node.rect.set_top(y, true);
        let mut resized = false;
        if node.rect.right() >= (self.surface_size.width as i32) || node.rect.bottom() >= (self.surface_size.height as i32) || x < 0 || y < 0 {
            // need to resize the surface
            self.resize_graph(false);
            resized = true;
        }
        self.repaint(control);
        resized
    }
    fn move_node_with(&mut self, id: usize, dx: i32, dy: i32, control: &ControlBase) {
        if id >= self.nodes.len() {
            return ;
        }
        let tl = self.nodes[id].rect.top_left();
        self.move_node_to(id, tl.x + dx, tl.y + dy, control);
    }

    pub(super) fn set_current_node(&mut self, index: usize, control: &ControlBase) {
        if index != self.current_node {
            if self.rendering_options.highlight_edges_in || self.rendering_options.highlight_edges_out {
                self.current_node = index;
                self.repaint(control);
            } else {
                let old_index = self.current_node;
                self.current_node = index;
                self.paint_node(control, old_index);
                self.paint_node(control, index);
            }
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
                Direction::Left => {
                    if r.right() > n.rect.right() {
                        Some(Direction::Right.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if r.left() < n.rect.left() {
                        Some(Direction::Left.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::Top => {
                    if r.bottom() > n.rect.bottom() {
                        Some(Direction::Bottom.compare_point(&n.rect))
                    } else {
                        None
                    }
                }
                Direction::Bottom => {
                    if r.top() < n.rect.top() {
                        Some(Direction::Top.compare_point(&n.rect))
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
    fn move_to_node_with_direction(&mut self, dir: Direction, control: &ControlBase) {
        if let Some(next_index) = self.next_node(dir) {
            self.set_current_node(next_index, control);
        } 
    }

    pub(super) fn process_key_events(&mut self, key: Key, control: &ControlBase) -> bool {
        match key.value() {
            key!("Left") => self.move_to_node_with_direction(Direction::Left, control),
            key!("Right") => self.move_to_node_with_direction(Direction::Right, control),
            key!("Up") => self.move_to_node_with_direction(Direction::Top, control),
            key!("Down") => self.move_to_node_with_direction(Direction::Bottom, control),
            key!("Ctrl+Left") => self.move_node_with(self.current_node, -1, 0, control),
            key!("Ctrl+Right") => self.move_node_with(self.current_node, 1, 0, control),
            key!("Ctrl+Up") => self.move_node_with(self.current_node, 0, -1, control),
            key!("Ctrl+Down") => self.move_node_with(self.current_node, 0, 1, control),
            key!("Ctrl+Tab") => {
                if !self.nodes.is_empty() {
                    self.set_current_node((self.current_node + 1) % self.nodes.len(), control);
                }
            }
            key!("Ctrl+Shift+Tab") => {
                if !self.nodes.is_empty() {
                    self.set_current_node((self.current_node + self.nodes.len() - 1) % self.nodes.len(), control);
                }
            }
            _ => return false,
        }
        true// key was processed
    }
    pub(super) fn node_description(&mut self, id: usize) -> Option<&str> {
        if id >= self.nodes.len() {
            return None;
        }
        self.repr_buffer.clear();
        if self.nodes[id].obj.write_description(&mut self.repr_buffer).is_err() {
            return None;
        }
        if self.repr_buffer.is_empty() {
            None
        } else {
            Some(&self.repr_buffer)
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
            rendering_options: RenderingOptions::new(),
        }
    }
}
