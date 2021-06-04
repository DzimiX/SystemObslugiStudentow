use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_announcements::{Ogloszenie, OgloszenieNowe, OgloszenieId};

#[post("/ogloszenia", format = "application/json")]
pub fn ogloszenia(conn: DbConn, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Ogloszenie::all(&conn),
    }))
}

#[post("/ogloszenia/nowe", format = "application/json", data = "<ogloszenie>")]
pub fn ogloszenia_nowe(conn: DbConn, ogloszenie: Json<OgloszenieNowe>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Ogloszenie::add(ogloszenie.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ogloszenia/aktualizuj", format = "application/json", data = "<ogloszenie>")]
pub fn ogloszenia_aktualizuj(conn: DbConn, ogloszenie: Json<Ogloszenie>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Ogloszenie::update(ogloszenie.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ogloszenia/usun", format = "application/json", data = "<id_ogloszenie>")]
pub fn ogloszenia_usun(conn: DbConn, id_ogloszenie: Json<OgloszenieId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_ogloszenie.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Ogloszenie::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}