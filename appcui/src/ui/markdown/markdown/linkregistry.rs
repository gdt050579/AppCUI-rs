use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct LinkID(pub String);

#[derive(PartialEq, Debug)]
struct IsHovered(pub bool);

#[derive(PartialEq, Debug)]
struct LinkArea {
    pub x_pos: i32,
    pub y_pos: i32,
    pub len: i32,
}

#[derive(PartialEq, Debug)]
pub enum LinkDestination {
    HeaderPosition(i32),
    ExternalLink(String),
}

pub(crate) struct LinkRegistry {
    links_map: HashMap<LinkID, (LinkArea, LinkDestination, IsHovered)>,
    current_hovered: Option<LinkID>,
}

impl LinkRegistry {
    pub fn new() -> Self {
        LinkRegistry {
            links_map: HashMap::new(),
            current_hovered: None,
        }
    }

    pub fn register_header_position(&mut self, header: &str, position: i32) {
        let id = LinkID(Self::get_id_from_header(header));
        if let Some((_, link_destination, _)) = self.links_map.get_mut(&id) {
            if let LinkDestination::HeaderPosition(_) = link_destination {
                *link_destination = LinkDestination::HeaderPosition(position);
            }
        } else {
            let area = LinkArea {
                x_pos: 0, 
                y_pos: 0,
                len: 0,
            };
            let header_position = LinkDestination::HeaderPosition(position);
            self.links_map.insert(
                id,
                (area, header_position, IsHovered(false)),
            );
        }
    }

    pub fn register_link_position(&mut self, link: &str, x_pos: i32, y_pos: i32, len: i32, external: bool) {
        let id = LinkID(link.to_string());
        if let Some((link_area, _, _)) = self.links_map.get_mut(&id) {
            *link_area = LinkArea { x_pos, y_pos, len };
        } else {
            let area = LinkArea { x_pos, y_pos, len };
            let link_destination = if !external { LinkDestination::HeaderPosition(0) } else { LinkDestination::ExternalLink(link.to_string()) };
            self.links_map.insert(id, (area, link_destination, IsHovered(false)));
        }
    }
    
    pub fn get_header_position(&self, id: &str) -> Option<i32> {
        let key = LinkID(id.to_string());
        self.links_map.get(&key).and_then(|(_, destination, _)| {
            if let LinkDestination::HeaderPosition(pos) = destination {
                Some(*pos)
            } else {
                None
            }
        })
    }

    pub fn check_for_link_at_position(&self, x: i32, y: i32) -> Option<String> {
        for (id, (area, _, _)) in &self.links_map {
            if Self::is_within_link_area(area, x, y) {
                return Some(id.0.clone());
            }
        }
        None
    }

    fn is_within_link_area(area: &LinkArea, x: i32, y: i32) -> bool {
        x >= area.x_pos && x <= area.x_pos + area.len && y == area.y_pos
    }

    pub fn get_id_from_header(header: &str) -> String {
        header
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .map(|c| if c == ' ' { '-' } else { c.to_ascii_lowercase() })
            .collect()
    }

    pub fn set_link_hovered(&mut self, id: &str) {
        let new_key = LinkID(id.to_string());
        if let Some(ref current) = self.current_hovered {
            if *current == new_key {
                return;
            } else if let Some((_, _, ref mut is_hovered)) = self.links_map.get_mut(current) {
                *is_hovered = IsHovered(false);
            }
        }
        if let Some((_, _, ref mut is_hovered)) = self.links_map.get_mut(&new_key) {
            *is_hovered = IsHovered(true);
            self.current_hovered = Some(new_key);
        }
    }

    pub fn is_hovered(&self, id: &str) -> bool {
        let key = LinkID(id.to_string());
        if let Some((_, _, is_hovered)) = self.links_map.get(&key) {
            is_hovered.0
        } else {
            false
        }
    }

    pub fn clear_hovered(&mut self) {
        if let Some(ref current) = self.current_hovered {
            if let Some((_, _, ref mut is_hovered)) = self.links_map.get_mut(current) {
                *is_hovered = IsHovered(false);
            }
        }
        self.current_hovered = None;
    }

    pub fn is_link_external(&self, id: &str) -> Option<bool> {
        let key = LinkID(id.to_string());
        self.links_map.get(&key).map(|(_, destination, _)| {
            match destination {
                LinkDestination::HeaderPosition(_) => false,
                LinkDestination::ExternalLink(_) => true,
            }
        })
    }
}
