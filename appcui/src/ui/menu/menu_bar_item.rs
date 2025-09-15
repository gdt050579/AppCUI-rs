// use crate::{system::Handle, utils::Caption, utils::ExtractHotKeyMethod};

// use super::Menu;


// /*
// * Menu(Menu)
// * Separator(|)
// * Label(text)
// * Button
// // x, enabled, hidden
// // fac un manager cu toate (problema e ca daca inchide o fereastra ar trebui sa le sterg !!! (de vazut cum))
// // cand se schimba focusul le adaug pe toate in vector

// menubar().add(Menu, order, direction::Left|Right)
// menubar().add(menubar::Label::new(...), order, drection::Left|Right)
// menubar().add(menubar::Separator::new(), order, direction::Left|Right)
// menubar().add(menubar::Menu(menu::Menu), order, direction::Left|Right)

// enum MenuBarItem {
//     Menu()
// }

// */

// pub(super) struct MenuBarItem_old {
//     pub(super) caption: Caption,
//     pub(super) x: i32,
//     pub(super) handle: Handle<Menu>,
//     pub(super) receiver_control_handle: Handle<()>,
//     pub(super) order: u8,
// }
// impl MenuBarItem_old {
//     pub(super) fn new(handle: Handle<Menu>, control_handle: Handle<()>, caption: &Caption, order: u8) -> Self {
//         Self {
//             x: 0,
//             handle,
//             receiver_control_handle: control_handle,
//             caption: if caption.chars_count() == 0 {
//                 Caption::new("?", ExtractHotKeyMethod::NoHotKey)
//             } else {
//                 caption.clone()
//             },
//             order
//         }
//     }
//     pub(super) fn set(&mut self, handle: Handle<Menu>, control_handle: Handle<()>, caption: &Caption) {
//         self.x = 0;
//         self.handle = handle;
//         self.receiver_control_handle = control_handle;
//         if caption.chars_count() == 0 {
//             self.caption.set_text("?", ExtractHotKeyMethod::NoHotKey);
//         } else {
//             self.caption.copy_from(caption);
//         }
//     }
//     pub(super) fn set_order(&mut self, order: u8) {
//         self.order = order;
//     }
// }
