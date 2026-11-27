pub(crate) struct Params {
    pub(crate) times: u32,
    pub(crate) filter: String,
}
impl Params {
    pub(crate) fn new(args: &[String]) -> Result<Self, String> {
        let mut me = Self { times: 10, filter: String::new() };
        

        Ok(me)
    }
}