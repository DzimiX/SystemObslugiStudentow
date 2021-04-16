#![allow(unused_variables)]
#![allow(dead_code)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;



use mysql;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

use rocket_contrib::json::Json;
use serde::Deserialize;

//#[database("sosbeta")]
//pub struct DbConn(diesel::MysqlConnection);

pub mod router;
pub mod routes;

fn main() {
    let pool = mysql::Pool::new("mysql://root@localhost/sosbeta").unwrap();    

    /* 
    
    Z jakiegoś czarodziejskiego powodu nie widzi metod prepare, prep_exec ani niczego co może uruchomić sqla na bazie

    pool.prepare(r"CREATE TEMPORARY TABLE tmp.payment (
       customer_id int not null,
        amount int not null,
        account_name text
    )", ()).unwrap();
    */

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