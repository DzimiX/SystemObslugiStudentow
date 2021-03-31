#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("sos")]
pub struct DbConn(diesel::MysqlConnection);

pub mod router;

fn main() {
    rocket();
}

fn rocket() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            router::index,
            router::testy,
            router::test_sql,
        ])
        .launch();
}