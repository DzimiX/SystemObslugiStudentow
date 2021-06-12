use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_enrollment::{Zapisy, ZapisyNowe, ZapisyId};

#[post("/zapisy", format = "application/json")]
pub fn zapisy(conn: DbConn, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Zapisy::all(&conn),
    }))
}

#[post("/zapisy/id", format = "application/json", data = "<id_zapisy>")]
pub fn zapisy_id(conn: DbConn, id_zapisy: Json<ZapisyId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",id_zapisy.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Zapisy::get(id, &conn),
    }))
}

#[post("/zapisy/nowe", format = "application/json", data = "<zapisy>")]
pub fn zapisy_nowe(conn: DbConn, zapisy: Json<ZapisyNowe>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Zapisy::add(zapisy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/zapisy/aktualizuj", format = "application/json", data = "<zapisy>")]
pub fn zapisy_aktualizuj(conn: DbConn, zapisy: Json<Zapisy>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Zapisy::update(zapisy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/zapisy/usun", format = "application/json", data = "<id_zapisy>")]
pub fn zapisy_usun(conn: DbConn, id_zapisy: Json<ZapisyId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_zapisy.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Zapisy::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}