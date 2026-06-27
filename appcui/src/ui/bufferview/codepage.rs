use flat_string::FlatString;

#[derive(Clone, Copy)]
pub struct Codepage {
    pub(super) map: [char;256],
    pub(super) name: FlatString<22>,
}
impl Codepage {
    const fn from_parts(map: &'static [char;256], name: &'static str) -> Self {
        Self {
            map: *map,
            name: FlatString::from_str(name),
        }
    }
    pub fn new(name: &str) -> Self {
        let mut me = Self {
            map: ['\0';256],
            name: FlatString::from_str(name),
        };
        for i in 0..256 {
            me.map[i] = (i as u8) as char;
        }
        me
    }
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    #[inline(always)]
    pub fn map(&self) -> &[char;256] {
        &self.map
    }
    #[inline(always)]
    pub fn set_map(&mut self, map: [char;256]) {
        self.map = map;
    }
    #[inline(always)]
    pub fn set_name(&mut self, name: &str) {
        self.name = FlatString::from_str(name);
    }
    #[inline(always)]
    pub fn set(&mut self, index: u8, ch: char) {
        self.map[index as usize] = ch;
    }
    #[inline(always)]
    pub fn get(&self, index: u8) -> char {
        self.map[index as usize]
    }
    pub fn clear(&mut self, ch: char) {
        for i in 0..256 {
            self.map[i] = ch;
        }
    }
}