pub struct Edge {
    pub(super) from_node_id: u32,
    pub(super) to_node_id: u32,
}
impl Edge {
    pub fn bidirectional(from_node_id: u32, to_node_id: u32) -> Self {
        Self { from_node_id, to_node_id }
    }
}
