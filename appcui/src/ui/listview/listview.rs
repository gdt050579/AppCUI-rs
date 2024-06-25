use listview::initialization_flags::ListItem;
use AppCUIProcMacro::*;
use super::Flags;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct ListView<T>
where
    T: ListItem
{
    flags: Flags,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OnPaint for ListView<T>
where
    T: ListItem
{
}

impl<T> OnKeyPressed for ListView<T>
where
    T: ListItem
{
}

impl<T> OnMouseEvent for ListView<T>
where
    T: ListItem
{
}