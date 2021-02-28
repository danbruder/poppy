pub struct User {}
pub struct Org {}

pub struct Photo {
    pub uri: String,
}

impl Photo {
    pub fn uri<'a>(&'a self) -> &'a str {
        &self.uri.as_str()
    }
}
