pub enum DialogButtons {
    Ok,
    YesNo,
    YesNoCancel,
    RetryCancel
}

impl DialogButtons {
    pub (super) fn count(&self)->u32 {
        match self {
            DialogButtons::Ok => 1,
            DialogButtons::YesNo => 2,
            DialogButtons::YesNoCancel => 3,
            DialogButtons::RetryCancel => 2,
        }
    }
}