use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_courses::{Kurs, KursNowy, KursId};
use crate::models_groups::{Grupa, ZapisyId};

#[post("/kurs", format = "application/json", data = "<kurs>")]
pub fn kurs(conn: DbConn, kurs : Json<KursId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",kurs.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Kurs::get(id, &conn),
    }))
}

#[post("/kursy/zapisy", format = "application/json", data = "<zapisy>")]
pub fn kursy_zapisy(conn: DbConn, zapisy : Json<ZapisyId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let kursy = Grupa::get_all_kurs_zapisy(zapisy.into_inner(), &conn);
    // usunąć zbędne rzeczy (opcjonalnie), wybrać tylko id_kursu (grupowane)

    Json(json!({
        "status" : 200,
        "result" : kursy,
    }))
}


#[post("/kursy", format = "application/json")]
pub fn kursy(conn: DbConn, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Kurs::all(&conn),
    }))
}

#[post("/kursy/nowe", format = "application/json", data = "<kurs_nowy>")]
pub fn kursy_nowe(conn: DbConn, kurs_nowy: Json<KursNowy>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Kurs::add(kurs_nowy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/kursy/aktualizuj", format = "application/json", data = "<kurs>")]
pub fn kursy_aktualizuj(conn: DbConn, kurs: Json<Kurs>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Kurs::update(kurs.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/kursy/usun", format = "application/json", data = "<id_kurs>")]
pub fn kursy_usun(conn: DbConn, id_kurs: Json<KursId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_kurs.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Kurs::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}