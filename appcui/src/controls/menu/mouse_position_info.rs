pub (super) MousePositionInfo {  
    item_index: u32,
    is_on_menu: bool,
    is_on_up_button: bool,
    is_on_bottom_button: bool
}
impl MousePositionInfo {
    fn new(x: i32, y: i32, menu: &Menu)->Self {
        let mut mpi = MousePositionInfo {item_index:0, is_on_menu: false, is_on_bottom_button: false, is_on_bottom_button: false};
        if (x >= 1) && (y >= 1) && (x <= (menu.width as i32)) && (y <= (menu.visible_items_count as i32))
        {
            mpi.ItemIndex = (y - 1) + FirstVisibleItem;
            if ((mpi.ItemIndex < ItemsCount) && (Items[mpi.ItemIndex]->Enabled) &&
                (Items[mpi.ItemIndex]->Type != MenuItemType::Line))
            {
                // all good - current item is valid
            }
            else
            {
                mpi.ItemIndex = NO_MENUITEM_SELECTED;
            }
        }
        else
        {
            mpi.ItemIndex = NO_MENUITEM_SELECTED;
        }
    
/*
    if ((x >= 1) && (y >= 1) && (x <= (int) Width) && (y <= (int) VisibleItemsCount))
    {
        mpi.ItemIndex = (y - 1) + FirstVisibleItem;
        if ((mpi.ItemIndex < ItemsCount) && (Items[mpi.ItemIndex]->Enabled) &&
            (Items[mpi.ItemIndex]->Type != MenuItemType::Line))
        {
            // all good - current item is valid
        }
        else
        {
            mpi.ItemIndex = NO_MENUITEM_SELECTED;
        }
    }
    else
    {
        mpi.ItemIndex = NO_MENUITEM_SELECTED;
    }
    mpi.IsOnMenu       = (x >= 0) && (y >= 0) && (x < (int) this->Width + 2) && (y < (int) this->VisibleItemsCount + 2);
    const auto middle   = this->Width >> 1;
    mpi.IsOnUpButton   = (y == 0) && (static_cast<uint32>(x) >= middle) && (static_cast<uint32>(x) <= middle + 2);
    mpi.IsOnDownButton  = (y == ScreenClip.ClipRect.Height - 1) && (static_cast<uint32>(x) >= middle) &&
                         (static_cast<uint32>(x) <= middle + 2);



*/
    }
}