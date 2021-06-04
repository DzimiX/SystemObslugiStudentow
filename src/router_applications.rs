use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_applications::{Sprawy, SprawyId, SprawyNowe};

#[post("/sprawy", format = "application/json")]
pub fn sprawy(conn: DbConn, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Sprawy::all(&conn),
    }))
}

#[post("/sprawy/nowe", format = "application/json", data = "<sprawy>")]
pub fn sprawy_nowe(conn: DbConn, sprawy: Json<SprawyNowe>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Sprawy::add(sprawy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/sprawy/aktualizuj", format = "application/json", data = "<sprawy>")]
pub fn sprawy_aktualizuj(conn: DbConn, sprawy: Json<Sprawy>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Sprawy::update(sprawy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/sprawy/pokaz", format = "application/json", data = "<id_sprawy>")]
pub fn sprawy_pokaz(conn: DbConn, id_sprawy: Json<SprawyId>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    let id : i32 = format!("{}",id_sprawy.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Sprawy::get(id, &conn).first(),
    }))
}

#[post("/sprawy/usun", format = "application/json", data = "<id_sprawy>")]
pub fn sprawy_usun(conn: DbConn, id_sprawy: Json<SprawyId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_sprawy.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Sprawy::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}