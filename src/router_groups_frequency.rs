use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_groups_frequency::{Termin, TerminNowy, TerminId, TerminIdGrupa};

#[post("/grupa/terminy", format = "application/json", data = "<termin>")]
pub fn grupa_terminy(conn: DbConn, termin : Json<TerminIdGrupa>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",termin.id_grupa).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Termin::get_grupa(id, &conn),
    }))
}

#[post("/grupa/terminy/nowy", format = "application/json", data = "<termin>")]
pub fn grupa_termin_nowy(conn: DbConn, termin : Json<TerminNowy>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Termin::add(termin.into_inner(), &conn),
    }))
}

#[post("/grupa/terminy/usun", format = "application/json", data = "<termin>")]
pub fn grupa_termin_usun(conn: DbConn, termin : Json<TerminId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",termin.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Termin::delete(id, &conn),
    }))
}

#[post("/grupa/terminy/aktualizuj", format = "application/json", data = "<termin>")]
pub fn grupa_termin_aktualizuj(conn: DbConn, termin : Json<Termin>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Termin::update(termin.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}