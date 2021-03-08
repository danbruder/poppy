use uuid::Uuid;

pub struct User {}

#[derive(Clone, new)]
pub struct File {
    pub path: String,
}

#[derive(Clone)]
pub struct Photo {
    pub id: String,
    pub uri: String,
}

impl Photo {
    pub fn new(path: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            uri: path.to_owned(),
        }
    }
    pub fn public_url(&self) -> String {
        format!("http://localhost:8080/public/{}", &self.uri)
    }
}
