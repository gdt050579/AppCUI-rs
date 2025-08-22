use super::GraphNode;

pub(super) struct Graph<T> where T: GraphNode {
    pub(super) nodes: Vec<Node<T>>,
    pub(super) edges: Vec<Edge>,
}
impl<T> Graph<T> where  T: GraphNode {
    
}