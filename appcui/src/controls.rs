pub mod layout;
pub mod events;
pub mod control_base;
pub mod control_handle;
pub mod control_char_attributes_state;
pub (crate) mod control_manager;

pub use layout::Layout;
pub use control_base::ControlBase;
pub use control_handle::ControlHandle;
pub use control_char_attributes_state::ControlCharAttributesState;
pub (crate) use control_manager::ControlManager;

use control_base::StatusFlags;


// controls
pub mod desktop;
pub mod checkbox;
pub mod label;
pub mod menu;
pub mod window;
pub mod button;
// re-export
pub use desktop::Desktop;
pub use checkbox::CheckBox;
pub use label::Label;
pub use window::Window;
pub use window::WindowFlags;
pub use button::Button;
pub use button::ButtonFlags;