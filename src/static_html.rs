use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[catch(404)]
pub fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("html/error/404.html")
}

#[get("/", rank = 1)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("html/index.html")
}

#[get("/<file..>?<id>", rank = 4)]
pub fn all_id(id: i32, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}

#[get("/<file..>", rank = 5)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}