use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;

use crate::models_groups::{Grupa, GrupaNowa, GrupaId, GrupaKursId, GrupaZapisyKursId};
use crate::models_groups::{Uczestnik, UczestnikNowy, UczestnikId, UczestnikGrupaId, UczestnikGrupaUczestnikId};
use crate::models_scores::{Ocena, OcenaGrupaUczestnikId};

#[post("/grupa", format = "application/json", data = "<grupa>")]
pub fn grupa(conn: DbConn, grupa : Json<GrupaId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",grupa.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Grupa::get(id, &conn),
    }))
}

#[post("/grupy", format = "application/json", data = "<grupa>")]
pub fn grupy(conn: DbConn, grupa: Json<GrupaKursId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Grupa::get_kurs(grupa.into_inner(),&conn),
    }))
}

#[post("/grupy_zapisy", format = "application/json", data = "<grupa>")]
pub fn grupy_zapisy(conn: DbConn, grupa : Json<GrupaZapisyKursId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Grupa::get_kurs_zapisy(grupa.into_inner(),&conn),
    }))
}

#[post("/grupa/nowe", format = "application/json", data = "<grupa>")]
pub fn grupy_nowe(conn: DbConn, grupa: Json<GrupaNowa>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Grupa::add(grupa.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/grupa/aktualizuj", format = "application/json", data = "<grupa>")]
pub fn grupy_aktualizuj(conn: DbConn, grupa: Json<Grupa>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Grupa::update(grupa.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/grupa/usun", format = "application/json", data = "<id_grupa>")]
pub fn grupy_usun(conn: DbConn, id_grupa: Json<GrupaId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_grupa.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Grupa::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uczestnicy", format = "application/json", data = "<uczestnik>")]
pub fn uczestnicy_grupa(conn: DbConn, uczestnik: Json<UczestnikGrupaId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Uczestnik::get_grupa_uczestnicy(uczestnik.into_inner(),&conn),
    }))
}

#[post("/uczestnik", format = "application/json", data = "<uczestnik>" )]
pub fn uczestnik_grupy(conn: DbConn, uczestnik: Json<UczestnikId>, cookies : Cookies) -> Json<Value> {

    Json(json!({
        "status" : 200,
        "result" : Uczestnik::get_uczestnik(uczestnik.into_inner(),&conn),
    }))
}

#[post("/uczestnicy/nowe", format = "application/json", data = "<uczestnik>")]
pub fn uczestnicy_nowe(conn: DbConn, uczestnik: Json<UczestnikNowy>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Uczestnik::add(uczestnik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uczestnicy/aktualizuj", format = "application/json", data = "<uczestnik>")]
pub fn uczestnicy_aktualizuj(conn: DbConn, uczestnik: Json<Uczestnik>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Uczestnik::update(uczestnik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uczestnik/usun", format = "application/json", data = "<id_uczestnik>")]
pub fn uczestnik_usun(conn: DbConn, id_uczestnik: Json<UczestnikId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_uczestnik.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Uczestnik::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uczestnik_usun", format = "application/json", data = "<uczestnik>")]
pub fn uczestnik_grupa_usun(conn: DbConn, uczestnik: Json<UczestnikGrupaUczestnikId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie
    
    // najprostsze obejście braku implementacji klonowania
    let kopia_wejscia = UczestnikGrupaUczestnikId { 
        id_grupa: uczestnik.id_grupa,
        id_uczestnik: uczestnik.id_uczestnik
    };

    let id = Uczestnik::get_id_from_grupa_uczestnik(kopia_wejscia, &conn);
    
    if id != -1 {
        let usun = OcenaGrupaUczestnikId { 
            id_grupa: uczestnik.id_grupa,
            id_uczestnik: id
        };

        Ocena::delete_grupa_uczestnik(usun, &conn);
    }
    
    let mut status = 400;
    if Uczestnik::delete_grupa_uczestnik(uczestnik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}