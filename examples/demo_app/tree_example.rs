use appcui::prelude::*;

#[derive(ListItem)]
struct TreeItem {
    #[Column(name="Classification", width=100)]
    name: String,
}

impl TreeItem {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[Window()]
pub(crate) struct Win {
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Animal Kingdom Classification',d:c,w:70,h:20,flags:Sizeable"),
        };

        // Create a tree control
        let mut tree = treeview!("TreeItem,d:c,flags:ScrollBars+SearchBar+LargeIcons");
        
        // Add root items (Kingdoms)
        let animalia = tree.add_item(treeview::Item::new(
            TreeItem::new("Animalia"),
            false,
            None,
            ['🐾', ' ']
        ));
        let plantae = tree.add_item(treeview::Item::new(
            TreeItem::new("Plantae"),
            false,
            None,
            ['🌱', ' ']
        ));
        let fungi = tree.add_item(treeview::Item::new(
            TreeItem::new("Fungi"),
            false,
            None,
            ['🍄', ' ']
        ));

        // Add phyla under Animalia
        let chordata = tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Chordata"),
            false,
            None,
            ['🐠', ' ']
        ), animalia);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Arthropoda"),
            false,
            None,
            ['🦗', ' ']
        ), animalia);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Mollusca"),
            false,
            None,
            ['🐚', ' ']
        ), animalia);

        // Add classes under Chordata
        let mammalia = tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Mammalia"),
            false,
            None,
            ['🐘', ' ']
        ), chordata);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Aves"),
            false,
            None,
            ['🦅', ' ']
        ), chordata);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Reptilia"),
            false,
            None,
            ['🐍', ' ']
        ), chordata);

        // Add orders under Mammalia
        let primates = tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Primates"),
            false,
            None,
            ['🐒', ' ']
        ), mammalia);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Carnivora"),
            false,
            None,
            ['🐺', ' ']
        ), mammalia);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Rodentia"),
            false,
            None,
            ['🐭', ' ']
        ), mammalia);

        // Add families under Primates
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Hominidae"),
            false,
            None,
            ['👨', ' ']
        ), primates);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Cercopithecidae"),
            false,
            None,
            ['🐵', ' ']
        ), primates);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Lemuridae"),
            false,
            None,
            ['🦝', ' ']
        ), primates);

        // Add classes under Plantae
        let angiosperms = tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Angiosperms"),
            false,
            None,
            ['🌺', ' ']
        ), plantae);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Gymnosperms"),
            false,
            None,
            ['🌲', ' ']
        ), plantae);

        // Add orders under Angiosperms
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Rosales"),
            false,
            None,
            ['🌹', ' ']
        ), angiosperms);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Fabales"),
            false,
            None,
            ['🌿', ' ']
        ), angiosperms);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Poales"),
            false,
            None,
            ['🌾', ' ']
        ), angiosperms);

        // Add classes under Fungi
        let ascomycota = tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Ascomycota"),
            false,
            None,
            ['🍄', ' ']
        ), fungi);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Basidiomycota"),
            false,
            None,
            ['🍄', ' ']
        ), fungi);

        // Add orders under Ascomycota
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Pezizales"),
            false,
            None,
            ['🍄', ' ']
        ), ascomycota);
        tree.add_item_to_parent(treeview::Item::new(
            TreeItem::new("Saccharomycetales"),
            false,
            None,
            ['🍄', ' ']
        ), ascomycota);

        me.add(tree);
        me
    }
} 