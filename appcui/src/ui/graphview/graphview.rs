use super::edge::Edge;
use super::graph::Graph;
use super::initialization_flags::Flags;
use super::node::Node;
use crate::{prelude::*, ui::graphview::GraphNode};

use self::components::ScrollBars;

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
    T: GraphNode,
{
    graph: Graph<T>,
    origin_point: Point,
    background: Option<Character>,
    flags: Flags,
    drag: Drag,
    scrollbars: ScrollBars,
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
                    | if flags == Flags::ScrollBars {
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
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
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
        super::layout::grid::rearange(&mut self.graph);
        self.graph.resize_to_fit();
        self.graph.repaint(&self.base);
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
        self.scrollbars.set_indexes((-self.origin_point.x) as u64, (-self.origin_point.y) as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }
    fn update_scroll_bars(&mut self) {
        let paint_sz = self.graph.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64, &self.base);
        self.move_scroll_to(self.origin_point.x, self.origin_point.y);        
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
        if (self.has_focus()) && (self.flags.contains_one(Flags::ScrollBars)) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }
        surface.draw_surface(self.origin_point.x, self.origin_point.y, self.graph.surface());
    }
}
impl<T> OnKeyPressed for GraphView<T>
where
    T: GraphNode,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
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
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.graph.reset_hover(&self.base);
                return EventProcessStatus::Processed;
            }
            MouseEvent::Over(point) => {
                return self.graph.process_mouse_over(&self.base, *point);
            }
            MouseEvent::Pressed(data) => {
                if let Some(id) = self.graph.mouse_pos_to_index(data.x, data.y) {
                    // click on a node
                    self.graph.set_current_node(id, &self.base);
                    let tl = self.graph.nodes[id].rect.top_left();
                    self.drag = Drag::Node(NodeInfo {
                        id,
                        top_left: tl,
                        origin: Point::new(data.x, data.y),
                    });
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
            MouseEvent::Released(data) => match &self.drag {
                Drag::None => {
                    return EventProcessStatus::Ignored;
                }
                Drag::View(p) => {
                    self.move_scroll_to(self.origin_point.x + data.x - p.x, self.origin_point.y + data.y - p.y);
                    self.drag = Drag::None;
                    return EventProcessStatus::Processed;
                }
                Drag::Node(node_info) => {
                    if self.graph.move_node_to(
                        node_info.id,
                        node_info.top_left.x + data.x - node_info.origin.x,
                        node_info.top_left.y + data.y - node_info.origin.y,
                        &self.base,
                    ) {
                        self.update_scroll_bars();
                    }
                    self.drag = Drag::None;
                    return EventProcessStatus::Processed;
                }
            },
            MouseEvent::DoubleClick(mouse_event_data) => todo!(),
            MouseEvent::Drag(data) => match &self.drag {
                Drag::None => {
                    return EventProcessStatus::Ignored;
                }
                Drag::View(p) => {
                    self.move_scroll_to(self.origin_point.x + data.x - p.x, self.origin_point.y + data.y - p.y);
                    self.drag = Drag::View(Point::new(data.x, data.y));
                    return EventProcessStatus::Processed;
                }
                Drag::Node(node_info) => {
                    if self.graph.move_node_to(
                        node_info.id,
                        node_info.top_left.x + data.x - node_info.origin.x,
                        node_info.top_left.y + data.y - node_info.origin.y,
                        &self.base,
                    ) {
                        self.update_scroll_bars();
                    }
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
        EventProcessStatus::Ignored
    }
}
