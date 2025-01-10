pub trait Navigator<E, R, P>: Clone
where
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    fn entries(&self, path: &P) -> Vec<E>;
    fn roots(&self) -> Vec<R>;
    fn new() -> Self;
    fn join(&self, path: &P, entry: &E) -> Option<P>;
    fn exists(&self, path: &P) -> Option<bool>;
    fn current_dir(&self) -> P;
}
pub trait NavigatorEntry {
    fn name(&self) -> &str;
    fn is_container(&self) -> bool;
}
pub trait NavigatorRoot {
    fn path(&self) -> &str;
}
