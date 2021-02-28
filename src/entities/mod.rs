pub struct User {}

pub struct File {
    path: String,
}

pub struct Photo {
    pub uri: String,
}

impl Photo {
    pub fn uri<'a>(&'a self) -> &'a str {
        &self.uri.as_str()
    }

    pub fn public_url(&self) -> String {
        format!("http://localhost:8080/public/{}", &self.uri)
    }
}
