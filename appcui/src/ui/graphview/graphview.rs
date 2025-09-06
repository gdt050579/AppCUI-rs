use std::any::TypeId;

use super::events::*;
use super::graph::Graph;
use super::initialization_flags::*;
use super::RenderingOptions;
use crate::{prelude::*, ui::graphview::GraphNode};

struct NodeInfo {
    id: usize,
    top_left: Point,
    origin: Point,
}
enum Drag {
    None,
    View(Point),
    Node(NodeInfo),
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnFocus, internal=true)]
pub struct GraphView<T>
where
    T: GraphNode + 'static,
{
    graph: Graph<T>,
    origin_point: Point,
    background: Option<Character>,
    flags: Flags,
    drag: Drag,
    comp: ListScrollBars,
    arrange_method: ArrangeMethod,
    rendering_options: RenderingOptions,
}
impl<T> GraphView<T>
where
    T: GraphNode,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags.contains_one(Flags::ScrollBars | Flags::SearchBar) {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            flags,
            origin_point: Point::ORIGIN,
            background: None,
            drag: Drag::None,
            graph: Graph::default(),
            arrange_method: ArrangeMethod::Grid,
            rendering_options: RenderingOptions::new(),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }

    /// Sets the background of the GraphView to the specified character.
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut GraphView = GraphView::new(Size::new(100, 100), layout!("x:1,y:1,w:30,h:10"), GraphView::Flags::ScrollBars);
    /// GraphView.set_background(Character::new('*', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn set_background(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }

    /// Clears the background character of the GraphView. It esentially resets it to transparent foreground and backgroud colors
    pub fn clear_background(&mut self) {
        self.background = None;
    }

    pub fn set_graph(&mut self, graph: Graph<T>) {
        self.graph = graph;
        self.graph.update_rendering_options(&self.rendering_options, &self.base);
        self.arrange_nodes(self.arrange_method);
    }

    pub fn set_edge_routing(&mut self, routing: EdgeRouting) {
        self.rendering_options.edge_routing = routing;
        self.graph.update_rendering_options(&self.rendering_options, &self.base);
    }

    pub fn set_edge_line_type(&mut self, line_type: LineType) {
        self.rendering_options.edge_line_type = line_type;
        self.graph.update_rendering_options(&self.rendering_options, &self.base);
    }

    pub fn enable_edge_highlighting(&mut self, incoming: bool, outgoing: bool) {
        self.rendering_options.highlight_edges_in = incoming;
        self.rendering_options.highlight_edges_out = outgoing;
        self.graph.update_rendering_options(&self.rendering_options, &self.base);
    }

    pub fn enable_arrow_heads(&mut self, enabled: bool) {
        self.rendering_options.show_arrow_heads = enabled;
        self.graph.update_rendering_options(&self.rendering_options, &self.base);
    }

    pub fn arrange_nodes(&mut self, method: ArrangeMethod) {
        match method {
            ArrangeMethod::Grid => super::node_layout::grid::rearange(&mut self.graph),
            ArrangeMethod::Circular => super::node_layout::circular::rearange(&mut self.graph),
            ArrangeMethod::Hierarchical => super::node_layout::hierarchical::rearange(&mut self.graph, 2),
            ArrangeMethod::HierarchicalPacked => super::node_layout::hierarchical::rearange(&mut self.graph, 1),
            ArrangeMethod::ForceDirected => super::node_layout::force_directed::rearange(&mut self.graph),
        }
        self.arrange_method = method;
        self.graph.resize_graph(true);
        self.graph.repaint(&self.base);
    }

    pub fn graph(&self) -> &Graph<T> {
        &self.graph
    }

    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.size();
        let surface_size = self.graph.size();
        self.origin_point.x = if surface_size.width <= sz.width {
            0
        } else {
            x.max((sz.width as i32) - (surface_size.width as i32))
        };
        self.origin_point.y = if surface_size.height <= sz.height {
            0
        } else {
            y.max((sz.height as i32) - (surface_size.height as i32))
        };
        self.origin_point.x = self.origin_point.x.min(0);
        self.origin_point.y = self.origin_point.y.min(0);
        self.comp.set_indexes((-self.origin_point.x) as u64, (-self.origin_point.y) as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.comp.horizontal_index() as i32);
        let v = -(self.comp.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }
    fn update_scroll_bars(&mut self) {
        let paint_sz = self.graph.size();
        let sz = self.size();
        self.comp.resize(paint_sz.width as u64, paint_sz.height as u64, &self.base, sz);
        self.move_scroll_to(self.origin_point.x, self.origin_point.y);
    }
    fn ensure_node_is_visible(&mut self, node_id: usize) {
        if let Some(node) = self.graph.nodes.get(node_id) {
            let node_rect = node.rect;
            let sz = self.size();
            let view_rect = Rect::with_point_and_size(Point::new(-self.origin_point.x, -self.origin_point.y), sz);

            if !view_rect.contains_rect(node_rect) {
                let mut adx = 0;
                let mut ady = 0;

                if node_rect.right() > view_rect.right() {
                    adx = node_rect.right() - view_rect.right();
                }
                if node_rect.left() < view_rect.left() {
                    adx = node_rect.left() - view_rect.left();
                }
                if node_rect.bottom() > view_rect.bottom() {
                    ady = node_rect.bottom() - view_rect.bottom();
                }
                if node_rect.top() < view_rect.top() {
                    ady = node_rect.top() - view_rect.top();
                }
                self.move_scroll_to(self.origin_point.x - adx, self.origin_point.y - ady);
            }
        }
    }
    fn ensure_current_node_is_visible(&mut self) {
        if let Some(id) = self.graph.current_node_id() {
            self.ensure_node_is_visible(id);
        }
    }
    fn search_text(&mut self) {
        let txt = self.comp.search_text();
        let count = self.graph.filter(txt, &self.base);
        self.ensure_current_node_is_visible();
        if count == self.graph.nodes.len() {
            self.comp.clear_match_count();
        } else {
            self.comp.set_match_count(count);
        }
    }
    fn goto_next_match(&mut self) {
        self.graph.goto_next_match(&self.base);
        self.ensure_current_node_is_visible();
    }
    fn goto_previous_match(&mut self) {
        self.graph.goto_previous_match(&self.base);
        self.ensure_current_node_is_visible();
    }
    fn raise_current_node_changed(&mut self, old_id: Option<usize>) {
        let new_id = self.graph.current_node_id();
        if (old_id != new_id) && new_id.is_some() {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::GraphView(EventData {
                    event_type: GraphViewEventTypes::CurrentNodeChanged,
                    type_id: TypeId::of::<T>(),
                }),
            });
        }
    }
    fn raise_action_on_node(&mut self, id: usize) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::GraphView(EventData {
                event_type: GraphViewEventTypes::NodeAction(id),
                type_id: TypeId::of::<T>(),
            }),
        });
    }
}
impl<T> OnResize for GraphView<T>
where
    T: GraphNode,
{
    fn on_resize(&mut self, _: Size, _: Size) {
        self.update_scroll_bars();
    }
}
impl<T> OnPaint for GraphView<T>
where
    T: GraphNode,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags.contains_one(Flags::ScrollBars | Flags::SearchBar)) {
            self.comp.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }
        surface.draw_surface(self.origin_point.x, self.origin_point.y, self.graph.surface());
        // let sz = format!("Size {:?}", self.size());
        // surface.write_string(0, 0, &sz, charattr!("w,black"), false);
    }
}
impl<T> OnKeyPressed for GraphView<T>
where
    T: GraphNode,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let nid = self.graph.current_node_id();
        if self.comp.process_key_pressed(key, character) {
            self.search_text();
            self.raise_current_node_changed(nid);
            return EventProcessStatus::Processed;
        }
        if self.graph.process_key_events(key, &self.base) {
            self.ensure_current_node_is_visible();
            self.comp.exit_edit_mode();
            self.raise_current_node_changed(nid);
            return EventProcessStatus::Processed;
        }
        let result = match key.value() {
            key!("Alt+Left") => {
                self.move_scroll_to(self.origin_point.x + 1, self.origin_point.y);
                EventProcessStatus::Processed
            }
            key!("Alt+Right") => {
                self.move_scroll_to(self.origin_point.x - 1, self.origin_point.y);
                EventProcessStatus::Processed
            }
            key!("Alt+Up") => {
                self.move_scroll_to(self.origin_point.x, self.origin_point.y + 1);
                EventProcessStatus::Processed
            }
            key!("Alt+Down") => {
                self.move_scroll_to(self.origin_point.x, self.origin_point.y - 1);
                EventProcessStatus::Processed
            }
            key!("PageUp") => {
                self.move_scroll_to(self.origin_point.x, self.origin_point.y + self.size().height as i32);
                EventProcessStatus::Processed
            }
            key!("PageDown") => {
                self.move_scroll_to(self.origin_point.x, self.origin_point.y - self.size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.move_scroll_to(0, 0);
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.move_scroll_to(i32::MIN, i32::MIN);
                EventProcessStatus::Processed
            }
            key!("Enter") => {
                if self.comp.is_in_edit_mode() {
                    self.goto_next_match();
                    self.raise_current_node_changed(nid);
                    // exist directly so that we don't exit the edit mode
                    return EventProcessStatus::Processed;
                } else {
                    if let Some(id) = self.graph.current_node_id() {
                        self.raise_action_on_node(id);
                        return EventProcessStatus::Processed;
                    }
                    EventProcessStatus::Ignored
                }
            }
            key!("Ctrl+Enter") => {
                if self.comp.is_in_edit_mode() {
                    self.goto_previous_match();
                    self.raise_current_node_changed(nid);
                    // exist directly so that we don't exit the edit mode
                    return EventProcessStatus::Processed;
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        };
        if result == EventProcessStatus::Processed {
            self.comp.exit_edit_mode();
        }
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            result
        }
    }
}

impl<T> OnFocus for GraphView<T>
where
    T: GraphNode,
{
    fn on_focus(&mut self) {
        self.graph.repaint(&self.base);
    }

    fn on_lose_focus(&mut self) {
        self.graph.repaint(&self.base);
    }
}

impl<T> OnMouseEvent for GraphView<T>
where
    T: GraphNode,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.graph.reset_hover(&self.base);
                self.hide_tooltip();
                return EventProcessStatus::Processed;
            }
            MouseEvent::Over(point) => {
                let p = Point::new(point.x - self.origin_point.x, point.y - self.origin_point.y);
                if !self.graph.process_mouse_over(&self.base, p) {
                    return EventProcessStatus::Ignored;
                }
                if let Some(id) = self.graph.hovered_node_id() {
                    let r = self.graph.nodes[id].rect + (self.origin_point.x, self.origin_point.y);
                    if let Some(desc) = self.graph.node_description(id) {
                        self.base.show_tooltip_on_rect(desc, &r);
                    } else {
                        self.hide_tooltip();
                    }
                } else {
                    self.hide_tooltip();
                }
                return EventProcessStatus::Processed;
            }
            MouseEvent::Pressed(mouse_data) => {
                let data = Point::new(mouse_data.x - self.origin_point.x, mouse_data.y - self.origin_point.y);
                if let Some(id) = self.graph.mouse_pos_to_index(data.x, data.y) {
                    // click on a node
                    let nid = self.graph.current_node_id();
                    self.graph.set_current_node(id, &self.base);
                    let tl = self.graph.nodes[id].rect.top_left();
                    self.drag = Drag::Node(NodeInfo {
                        id,
                        top_left: tl,
                        origin: Point::new(data.x, data.y),
                    });
                    self.raise_current_node_changed(nid);
                    return EventProcessStatus::Processed;
                }
                if self.flags.contains_one(Flags::ScrollBars) && (self.has_focus()) {
                    let sz = self.size();
                    if (data.x == sz.width as i32) || (data.y == sz.height as i32) {
                        return EventProcessStatus::Ignored;
                    }
                }
                self.drag = Drag::View(Point::new(data.x, data.y));
                return EventProcessStatus::Processed;
            }
            MouseEvent::Released(mouse_data) => match &self.drag {
                Drag::None => {
                    return EventProcessStatus::Ignored;
                }
                Drag::View(p) => {
                    self.move_scroll_to(self.origin_point.x + mouse_data.x - p.x, self.origin_point.y + mouse_data.y - p.y);
                    self.drag = Drag::None;
                    return EventProcessStatus::Processed;
                }
                Drag::Node(node_info) => {
                    let data = Point::new(mouse_data.x - self.origin_point.x, mouse_data.y - self.origin_point.y);
                    if self.graph.move_node_to(
                        node_info.id,
                        node_info.top_left.x + data.x - node_info.origin.x,
                        node_info.top_left.y + data.y - node_info.origin.y,
                        &self.base,
                    ) {
                        self.update_scroll_bars();
                    }
                    self.drag = Drag::None;
                    self.ensure_current_node_is_visible();
                    return EventProcessStatus::Processed;
                }
            },
            MouseEvent::DoubleClick(mouse_data) => {
                let data = Point::new(mouse_data.x - self.origin_point.x, mouse_data.y - self.origin_point.y);
                if let Some(id) = self.graph.mouse_pos_to_index(data.x, data.y) {
                    self.raise_action_on_node(id);
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            MouseEvent::Drag(mouse_data) => match &self.drag {
                Drag::None => {
                    return EventProcessStatus::Ignored;
                }
                Drag::View(p) => {
                    self.move_scroll_to(self.origin_point.x + mouse_data.x - p.x, self.origin_point.y + mouse_data.y - p.y);
                    self.drag = Drag::View(Point::new(mouse_data.x, mouse_data.y));
                    return EventProcessStatus::Processed;
                }
                Drag::Node(node_info) => {
                    let data = Point::new(mouse_data.x - self.origin_point.x, mouse_data.y - self.origin_point.y);
                    if self.graph.move_node_to(
                        node_info.id,
                        node_info.top_left.x + data.x - node_info.origin.x,
                        node_info.top_left.y + data.y - node_info.origin.y,
                        &self.base,
                    ) {
                        self.update_scroll_bars();
                    }
                    self.ensure_current_node_is_visible();
                    return EventProcessStatus::Processed;
                }
            },
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Left => self.move_scroll_to(self.origin_point.x + 1, self.origin_point.y),
                    MouseWheelDirection::Right => self.move_scroll_to(self.origin_point.x - 1, self.origin_point.y),
                    MouseWheelDirection::Up => self.move_scroll_to(self.origin_point.x, self.origin_point.y + 1),
                    MouseWheelDirection::Down => self.move_scroll_to(self.origin_point.x, self.origin_point.y - 1),
                };
                return EventProcessStatus::Processed;
            }
        }
    }
}
