//! User interface components and controls for AppCUI.
//!
//! This module provides a comprehensive set of UI controls and components for building
//! terminal-based user interfaces in the AppCUI framework. The UI module includes:
//!
//! # Core Components
//!
//! - [`Desktop`]: The main container for the application UI
//! - [`Window`]: Standard application windows
//! - [`Panel`]: Container for grouping related controls
//! - [`Layout`]: Management of control positioning and sizing
//!
//! # Input Controls
//!
//! - [`Button`]: Clickable button control
//! - [`TextField`]: Single-line text input
//! - [`TextArea`]: Multi-line text input and editing
//! - [`Password`]: Masked text input for secure data
//! - [`CheckBox`]: Boolean selection control
//! - [`RadioBox`]: Single selection from multiple options
//! - [`ThreeStateBox`]: Three-state checkbox (checked, unchecked, indeterminate)
//! - [`ComboBox`]: Editable dropdown list
//! - [`DropDownList`]: Non-editable dropdown list
//! - [`NumericSelector`]: Numeric value input and adjustment
//! - [`DatePicker`]: Date selection control
//! - [`KeySelector`]: Keyboard shortcut selector
//! - [`ColorPicker`]: Color selection control
//!
//! # Display Controls
//!
//! - [`Label`]: Static text display
//! - [`ProgressBar`]: Visual representation of progress
//! - [`ImageViewer`]: Display and manipulation of images
//! - [`Canvas`]: Custom drawing surface
//! - [`HLine`]/[`VLine`]: Horizontal and vertical separators
//!
//! # Navigation and Organization
//!
//! - [`Tab`]: Tabbed interface for organizing content
//! - [`Accordion`]: Collapsible sections of controls
//! - [`VSplitter`]/[`HSplitter`]: Resizable split views (vertical/horizontal)
//! - [`TreeView`]: Hierarchical data presentation
//! - [`ListBox`]: Simple list of selectable items
//! - [`ListView`]: Multi-column list with headers
//! - [`PathFinder`]: File system navigation control
//!
//! # Menus and Commands
//!
//! - [`Menu`]: Application menus
//! - [`CommandBar`]: Shortcut command interface
//!
//! # Example
//!
//! Creating a simple hello world window:
//!
//! ```rust, no_run
//! use appcui::prelude::*;
//!
//! fn main() -> Result<(), appcui::system::Error> {
//!     // Initialize the application
//!     let mut app = App::new().build()?;
//!     
//!     // Create a window with centered layout and specific size
//!     let mut win = Window::new(
//!         "First Window",
//!         Layout::new("dock:center,width:30,height:9"),
//!         window::Flags::Sizeable
//!     );
//!     
//!     // Add a label to the window
//!     win.add(Label::new("Hello World !", Layout::new("dock:center,width:13,height:1")));
//!     
//!     // Add the window to the application and run
//!     app.add_window(win);
//!     app.run();
//!     
//!     Ok(())
//! }

pub mod common;
pub mod layout;

// controls
pub mod accordion;
pub mod button;
pub mod canvas;
pub mod checkbox;
pub mod colorpicker;
pub mod combobox;
pub mod command_bar;
pub mod components;
pub mod datepicker;
pub mod desktop;
pub mod dropdownlist;
pub mod hline;
pub mod hsplitter;
pub mod imageviewer;
pub mod keyselector;
pub mod label;
pub mod listbox;
pub mod listview;
pub mod markdown;
pub mod menu;
pub mod numericselector;
pub mod panel;
pub mod password;
pub mod pathfinder;
pub mod progressbar;
pub mod radiobox;
pub mod selector;
pub mod tab;
pub mod textarea;
pub mod textfield;
pub mod threestatebox;
pub mod togglebutton;
pub mod treeview;
pub mod vline;
pub mod vsplitter;
pub mod window;

// re-export
pub use accordion::Accordion;
pub use button::Button;
pub use canvas::Canvas;
pub use checkbox::CheckBox;
pub use colorpicker::ColorPicker;
pub use combobox::ComboBox;
pub use command_bar::CommandBar;
pub use common::ControlBase;
pub use datepicker::DatePicker;
pub use desktop::Desktop;
pub use dropdownlist::DropDownList;
pub use hline::HLine;
pub use hsplitter::HSplitter;
pub use imageviewer::ImageViewer;
pub use keyselector::KeySelector;
pub use label::Label;
pub use layout::Layout;
pub use listbox::ListBox;
pub use listview::ListView;
pub use markdown::Markdown;
pub use menu::Menu;
pub use numericselector::NumericSelector;
pub use panel::Panel;
pub use password::Password;
pub use pathfinder::PathFinder;
pub use progressbar::ProgressBar;
pub use radiobox::RadioBox;
pub use selector::Selector;
pub use tab::Tab;
pub use textarea::TextArea;
pub use textfield::TextField;
pub use threestatebox::ThreeStateBox;
pub use togglebutton::ToggleButton;
pub use treeview::TreeView;
pub use vline::VLine;
pub use vsplitter::VSplitter;
pub use window::ModalWindow;
pub use window::Window;
