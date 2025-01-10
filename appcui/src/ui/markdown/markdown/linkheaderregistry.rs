use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct LinkHeaderID(pub String);

pub struct HeaderPosition(pub i32);

pub struct LinkArea {
    pub x_pos: i32,
    pub y_pos: i32,
    pub len: i32,
}

pub struct LinkHeaderRegistry {
    pub link_header_positions: HashMap<LinkHeaderID, (LinkArea, HeaderPosition)>,
}

impl LinkHeaderRegistry {
    pub fn new() -> Self {
        LinkHeaderRegistry {
            link_header_positions: HashMap::new(),
        }
    }

    pub fn register_header_position(&mut self, header: &str, position: i32) {
        let id = LinkHeaderID(Self::generate_link_id(header));
        
        if let Some((_, header_position)) = self.link_header_positions.get_mut(&id) {
            *header_position = HeaderPosition(position);
        } else {
            let area = LinkArea {
                x_pos: 0, 
                y_pos: 0,
                len: 0,
            };
            let header_position = HeaderPosition(position);
            self.link_header_positions.insert(id, (area, header_position));
        }
    }

    pub fn register_link_position(&mut self, link: &str, x_pos: i32, y_pos: i32, len: i32) {
        let id = LinkHeaderID(Self::generate_link_id(link));
        
        if let Some((link_area, _)) = self.link_header_positions.get_mut(&id) {
            *link_area = LinkArea { x_pos, y_pos, len };
        } else {
            let area = LinkArea { x_pos, y_pos, len };
            let header_position = HeaderPosition(0); 
            self.link_header_positions.insert(id, (area, header_position));
        }
    }

    pub fn get_entry(&self, id: &str) -> Option<&(LinkArea, HeaderPosition)> {
        let id = LinkHeaderID(Self::generate_link_id(id));
        self.link_header_positions.get(&id)
    }

    pub fn get_link_area(&self, header: &str) -> Option<&LinkArea> {
        let id = LinkHeaderID(Self::generate_link_id(header));
        self.link_header_positions.get(&id).map(|(link_area, _)| link_area)
    }

    pub fn get_header_position(&self, header: &str) -> Option<&HeaderPosition> {
        let id = LinkHeaderID(Self::generate_link_id(header));
        self.link_header_positions.get(&id).map(|(_, header_position)| header_position)
    }

    pub fn check_for_link_at_position(&self, x: i32, y: i32) -> Option<String> {
        for (id, (area, _)) in &self.link_header_positions {
            if self.is_within_link_area(area, x, y) {
                return Some(id.0.clone());
            }
        }
        None
    }

    fn is_within_link_area(&self, area: &LinkArea, x: i32, y: i32) -> bool {
        x >= area.x_pos && x <= area.x_pos + area.len && y >= area.y_pos && y <= area.y_pos + 20
    }

    fn generate_link_id(header: &str) -> String {
        header
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .map(|c| if c == ' ' { '-' } else { c.to_ascii_lowercase() })
            .collect()
    }
}
