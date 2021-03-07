pub struct User {}

#[derive(Clone, new)]
pub struct File {
    pub path: String,
}

#[derive(Clone, new)]
pub struct Photo {
    pub id: String,
    pub uri: String,
}

impl Photo {
    pub fn public_url(&self) -> String {
        format!("http://localhost:8080/public/{}", &self.uri)
    }
}
