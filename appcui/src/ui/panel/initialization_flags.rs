#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Type {
    Border,
    Window,
    Page,
    TopBar
}