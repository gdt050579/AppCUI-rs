pub(crate) struct Finder {
    cached_path: String,
    cached_items: Vec<String>,
    only_folders: bool,
}

impl Finder {
    pub fn new(only_folders: bool) -> Self {
        Self {
            cached_path: String::new(),
            cached_items: vec![],
            only_folders: only_folders,
        }
    }
}

