use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;
use chrono::{Local};

use crate::models_messages::{Wiadomosc, WiadomoscId, NowaWiadomosc, NowaWiadomoscBezDaty, NowaWiadomoscUczestnik};

#[post("/wiadomosci/nowa", format = "application/json", data = "<nowa_wiadomosc>")]
pub fn wiadomosci_nowa(conn: DbConn, nowa_wiadomosc: Json<NowaWiadomoscBezDaty>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne  
    let wiadomosc = NowaWiadomosc {
        id_uzytkownik : nowa_wiadomosc.id_uzytkownik,
        temat : String::from(&nowa_wiadomosc.temat),
        data : Local::now().timestamp(),
        dane : String::from(&nowa_wiadomosc.dane),
    };

    let mut status = 400;
    if Wiadomosc::add(wiadomosc, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : Wiadomosc::get_last_id(&conn),
    }))
}

#[post("/wiadomosci/dodajodbiorce", format = "application/json", data = "<nowy_wiadomosc_uczestnik>")]
pub fn wiadomosci_dodajodbiorce(conn: DbConn, nowy_wiadomosc_uczestnik: Json<NowaWiadomoscUczestnik>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    let mut status = 400;
    if Wiadomosc::add_recipient(nowy_wiadomosc_uczestnik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/wiadomosci/pokaz", format = "application/json", data = "<id_wiadomosc>")]
pub fn wiadomosci_pokaz(conn: DbConn, id_wiadomosc: Json<WiadomoscId>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    let id : i32 = format!("{}",id_wiadomosc.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Wiadomosc::get(id, &conn).first(),
    }))
}

#[post("/wiadomosci/domnie", format = "application/json", data = "<id>")]
pub fn wiadomosci_domnie(conn: DbConn, id: Json<WiadomoscId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne
    let id_uczestnik : i32 = format!("{}", id.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Wiadomosc::get_messages(id_uczestnik, &conn),
    }))
}