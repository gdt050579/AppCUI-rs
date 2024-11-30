pub trait Navigator<E, R>
where
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    fn entries(&self, path: &str) -> Vec<E>;
    fn roots(&self) -> Vec<R>;
}
pub trait NavigatorEntry {
    fn name(&self) -> &str;
    fn is_container(&self) -> bool;
}
pub trait NavigatorRoot {
    fn name(&self) -> &str;
}
