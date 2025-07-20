use crate::prelude::*;

// the purpose of this desktop is to be used internaly for Single window apps
// it has not paint / no keyboard or mouse implementation to add minimal performance impact for Single window apps

#[CustomControl(internal=true, desktop=true)]
pub(crate) struct EmptyDesktop {}

impl EmptyDesktop {
    pub(crate) fn new() -> Self {
        if RuntimeManager::is_instantiated() {
            panic!("A desktop object can only be created once (when the application is started) !");
        }   
        Self {
            base: ControlBase::with_status_flags(
                layout!("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | StatusFlags::DesktopControl,
            ),
        }
    }
}
