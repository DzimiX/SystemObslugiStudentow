use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;
use chrono::Local;

use super::OCENY;

use crate::models_scores::{Ocena, OcenaNowa, OcenaId, OcenaGrupaUczestnikId};
use crate::models_scores::{OcenaKoncowa, OcenaKoncowaNowa, OcenaKoncowaId, OcenaKoncowaUczestnikId, OcenaKoncowaGrupaUczestnikId};

#[post("/ocena", format = "application/json", data = "<uczestnik>")]
pub fn ocena_grupa_uczestnik(conn: DbConn, uczestnik : Json<OcenaGrupaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Ocena::get_grupa_student(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/nowa", format = "application/json", data = "<ocena>")]
pub fn ocena_nowa(conn: DbConn, ocena: Json<OcenaNowa>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    if OCENY.contains(&ocena.ocena) {
        let ocena_temp = OcenaNowa {
            id_grupa : ocena.id_grupa,
            id_uczestnik : ocena.id_uczestnik,
            ocena : ocena.ocena,
            waga : ocena.waga,
            komentarz : String::from(&ocena.komentarz),
            data : Local::now().timestamp()
        };
    
        let mut status = 400;
        if Ocena::add(ocena_temp, &conn) == true {
            status = 200;
        }

        Json(json!({
            "status" : status,
            "result" : "OK",
        }))
    }else {
        Json(json!({
            "status" : 400,
            "result" : "Zła ocena",
        }))
    }
}

#[post("/ocena/aktualizuj", format = "application/json", data = "<ocena>")]
pub fn ocena_aktualizuj(conn: DbConn, ocena: Json<Ocena>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Ocena::update(ocena.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ocena/usun", format = "application/json", data = "<ocena>")]
pub fn ocena_usun(conn: DbConn, ocena: Json<OcenaId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",ocena.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Ocena::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ocena/uczestnik/usun", format = "application/json", data = "<ocena>")]
pub fn ocena_uczestnik_usun(conn: DbConn, ocena : Json<OcenaGrupaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Ocena::delete_grupa_uczestnik(ocena.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ocena/uczestnik/srednia", format = "application/json", data = "<ocena>")]
pub fn ocena_uczestnik_srednia(conn: DbConn, ocena : Json<OcenaGrupaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    let srednia = Ocena::average(ocena.into_inner(), &conn);

    if srednia != -1.0 {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : srednia,
    }))
}

#[post("/ocena/koncowa", format = "application/json", data = "<uczestnik>")]
pub fn ocena_koncowa_grupa_uczestnik(conn: DbConn, uczestnik : Json<OcenaKoncowaGrupaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::get_grupa_student(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/akceptuj", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_akceptuj(conn: DbConn, ocena : Json<OcenaKoncowaId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::accept(ocena.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/wszystkie", format = "application/json", data = "<uczestnik>")]
pub fn ocena_koncowa_uczestnik(conn: DbConn, uczestnik : Json<OcenaKoncowaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::get_all(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/nowa", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_nowa(conn: DbConn, ocena: Json<OcenaKoncowaNowa>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if OcenaKoncowa::add(ocena.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ocena/koncowa/aktualizuj", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_aktualizuj(conn: DbConn, ocena: Json<OcenaKoncowa>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if OcenaKoncowa::update(ocena.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ocena/koncowa/usun", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_usun(conn: DbConn, ocena: Json<OcenaKoncowaId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",ocena.id).parse::<i32>().unwrap();

    let mut status = 400;
    if OcenaKoncowa::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}