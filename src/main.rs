#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

mod router;
mod models;
mod schema;
mod static_html;

mod db;

fn main() {

    let mut tokens = 0;

    rocket();
}

fn rocket() {

    let database_url = "mysql://root@127.0.0.1/sosbeta";
    let pool = db::init_pool(database_url.to_string());

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![
            static_html::index,
            static_html::all,
            router::uzytkownicy_index,
            router::uzytkownicy_nowy,
            router::uzytkownicy_id,
            
            //router::logowanie,
        ])
        .launch();
}