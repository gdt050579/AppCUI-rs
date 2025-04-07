//! # appcui
//!
//! `appcui` is a cross-platform TUI (**T**ext **U**ser **I**nterface / **T**erminal **U**ser **I**nterface) / CUI (**C**onsole **U**ser **I**nterface) framework designed to allow quick creation TUI/CUI based applications. 
//! AppCUI has a lot of out-of-the-box controls, and can also provide quick macros to create custom controls. The list of current supported controls are:
//! * Accordion
//! * Button
//! * CheckBox
//! * ComboBox
//! * Canvas
//! * DatePicker
//! * DropDownList
//! * HLine
//! * HSplitter
//! * ImageViewer
//! * Label
//! * ListBox
//! * Menu
//! * Markdown
//! * Panel
//! * ProgressBar
//! * RadioButton
//! * Tab
//! * TextBox
//! * VLine
//! * Window
//!   and many more.



pub mod graphics;
pub mod terminals;
pub mod input;
pub mod ui;
pub mod system;
pub mod dialogs;
pub mod prelude;
mod utils;