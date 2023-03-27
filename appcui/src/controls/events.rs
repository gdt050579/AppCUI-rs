mod common_traits;
mod event;

pub use common_traits::OnPaint;
pub use common_traits::OnKeyPressed;
pub use common_traits::OnMouseEvent;
pub use common_traits::OnDefaultAction;
pub use common_traits::OnResize;
pub use common_traits::Control;
pub use common_traits::EventProcessStatus;
pub use event::Event;