use crate::ui::appbar::Side;
use crate::system::Handle;
use super::AppBarItem;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits: 8)]
enum Flags {
    Enabled = 0x01,
    AcceptInput = 0x02,
    OnLeft = 0x04,
}

// must be copy for fast adding in a Vec<...>
#[derive(Copy,Clone)]
pub(super) struct ItemBase {
    flags: Flags,
    x: i32,
    width: u8,
    order: u8,
    handle: Handle<AppBarItem>
}
impl ItemBase {
    pub(super) fn new(width: u8, order: u8, position: Side, accept_input: bool) -> Self {
        let f = Flags::Enabled
            | if position == Side::Left { Flags::OnLeft } else { Flags::None }
            | if accept_input { Flags::AcceptInput } else { Flags::None };

        Self {
            flags: f,
            x: 0,
            width: width.max(1),
            order,
            handle: Handle::None,
        }
    }
    #[inline(always)]
    pub(super) fn update_handle(&mut self, handle: Handle<AppBarItem>) {
        self.handle = handle;
    }
    #[inline(always)]
    pub(super) fn handle(&self) -> Handle<AppBarItem> {
        self.handle
    }
    #[inline(always)]
    pub(super) fn width(&self) -> u8 {
        self.width
    }
    #[inline(always)]
    pub(super) fn order(&self) -> u8 {
        self.order
    }    
    #[inline(always)]
    pub(super) fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    #[inline(always)]
    pub(super) fn x(&self) -> i32 {
        self.x
    }
    #[inline(always)]   
    pub(super) fn is_enabled(&self) -> bool {
        self.flags.contains_one(Flags::Enabled)
    }   
    #[inline(always)]   
    pub(super) fn is_on_left(&self) -> bool {
        self.flags.contains_one(Flags::OnLeft)
    }     
}
