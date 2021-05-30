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

#[get("/<file..>?<id_grupa>&<id_uczestnik>", rank = 4)]
pub fn all_id(id_grupa: i32, id_uczestnik: i32, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}

#[get("/<file..>", rank = 5)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}