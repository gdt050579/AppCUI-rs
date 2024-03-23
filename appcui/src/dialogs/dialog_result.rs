#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    Retry
}