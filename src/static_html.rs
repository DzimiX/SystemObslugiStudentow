use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/", rank = 1)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("html/index.html")
}

#[get("/<file..>", rank = 5)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}