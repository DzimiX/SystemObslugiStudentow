#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod router;
mod models;
mod schema;
mod static_html;

mod db;

static DATABASE_URL : &str = "mysql://root@127.0.0.1/sosbeta";

static ADMINISTRATOR : i32 = 5;
static PRACOWNIK     : i32 = 4;
static PROWADZACY    : i32 = 3;
static STUDENT       : i32 = 2;
static UZYTKOWNIK    : i32 = 1;

use chrono::{Local};

fn main() {

    let now_timestamp = Local::now().timestamp();
    println!("{}", now_timestamp);

    rocket();
}

fn rocket() {
    let pool = db::init_pool(DATABASE_URL.to_string());

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![
            static_html::index,
            static_html::all,
        ])
        .mount("/api", routes![
            router::uzytkownicy_index,
            router::uzytkownicy_nowy,
            
            router::uzytkownik,
            router::uzytkownik_nowe_haslo,
            
            router::logowanie,
            router::autoryzacja,
        ])
        .launch();
}