pub trait Navigator<E, R, P>
where
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    fn entries(&self, path: &P) -> Vec<E>;
    fn roots(&self) -> Vec<R>;
    fn join(&self, path: &P, entry: &E) -> Option<P>;
}
pub trait NavigatorEntry {
    fn name(&self) -> &str;
    fn is_container(&self) -> bool;
}
pub trait NavigatorRoot {
    fn name(&self) -> &str;
}
