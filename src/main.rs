#![allow(unused_variables)]
#![allow(dead_code)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("sosbeta")]
pub struct DbConn(diesel::MysqlConnection);

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub mod router;
pub mod routes;
mod models;
mod schema;

fn main() {

    let database_url = "mysql://root@127.0.0.1/sosbeta";
    let conn = MysqlConnection::establish(&database_url).unwrap();

    // TESTY dodawanie do bazy

    let uzytkownik = models::NowyUzytkownik {
        login: String::from("anowak"),
        imie: String::from("Adam"),
        nazwisko: String::from("Nowak"),
    };

    if models::Uzytkownik::add(uzytkownik, &conn) {
        println!("Poszłykoniepobetonie");
    } else {
        println!("Zmieniamyormporaz95");
    }

    // https://youtu.be/X4MeByx38-4?t=2

    rocket();
}

fn rocket() {
    rocket::ignite()
        //.attach(DbConn::fairing())
        .mount("/", routes![
            router::index,
            router::testy,
            router::test_sql,
            router::login,
            //router::user,
            routes::user::page,
            routes::courses::page,
        ])
        .launch();
}