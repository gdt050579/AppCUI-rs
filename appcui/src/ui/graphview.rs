//! A graph view control for displaying and interacting with node-edge graphs.
//!
//! The GraphView control provides a way to display and interact with graphs, which are collections of nodes and edges.
//! It supports various layout algorithms, edge routing, and interactive features.
//! 
//! The GraphView control is designed to be used with the GraphNode trait, which provides the necessary methods
//! for displaying and interacting with nodes and edges.
//! 
//! The GraphView control is also designed to be used with the GraphBuilder trait, which provides the necessary methods
//! for building graphs.

mod graphview;
mod graphnode;
mod initialization_flags;
mod node_layout;
mod node;
mod edge;
mod graph;
mod graph_rendering_options;
pub mod events;
#[cfg(test)]
mod tests;

use self::graph_rendering_options::RenderingOptions;

pub use self::graphview::GraphView;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::EdgeRouting;
pub use self::initialization_flags::ArrangeMethod;
pub use self::graphnode::GraphNode;
pub use self::graph::Graph;
pub use self::node::Node;
pub use self::node::NodeBuilder;
pub use self::edge::Edge;
pub use self::edge::EdgeBuilder;

