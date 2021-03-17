use std::env;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("must set DATABASE_URL");
    pub static ref FILES_DIR: String = env::var("FILES_DIR").expect("must set FILES_DIR");
    pub static ref DATABASE_PATH: String =
        env::var("DATABASE_PATH").expect("must set DATABASE_PATH");
}

pub fn init() {
    init_dir(files_dir());
    init_db_dir_and_file();
}

pub fn file_path(name: &str) -> PathBuf {
    let mut buf: PathBuf = files_dir().into();
    buf.push(name);
    buf
}

pub fn get_database_url() -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(&DATABASE_URL.as_str());
    buf.to_owned()
}

fn init_dir(path: PathBuf) {
    fs::create_dir_all(&path).unwrap();
}

fn init_db_dir_and_file() {
    let path = database_path();
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();

    use std::fs::OpenOptions;

    let _ = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path);
}

pub fn files_dir() -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(&FILES_DIR.as_str());
    buf
}

fn database_path() -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(&DATABASE_PATH.as_str());
    buf
}
