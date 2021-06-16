use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Cookies;
use chrono::Local;

use super::OCENY;

use crate::models_scores::{Ocena, OcenaNowa, OcenaId, OcenaGrupaUczestnikId};
use crate::models_scores::{OcenaKoncowa, OcenaKoncowaNowa, OcenaKoncowaId, OcenaKoncowaUczestnikId, OcenaKoncowaGrupaUczestnikId};
use crate::models_scores::{Ankieta, AnkietaNowa, AnkietaIdGrupa};

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

#[post("/ocena/koncowa/id", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_id(conn: DbConn, ocena : Json<OcenaKoncowaId>, cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::get_by_id(ocena.into_inner(),&conn),
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

// /api/ocena/koncowa/feedback

#[post("/ocena/koncowa/feedback", format = "application/json", data = "<ankieta>")]
pub fn ocena_koncowa_feedback(conn: DbConn, ankieta: Json<AnkietaNowa>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie
    let temp_id = OcenaKoncowaId {
        id : ankieta.id_ocena_koncowa
    };
    let verify = OcenaKoncowa::get_by_id(temp_id, &conn);

    if verify.id == ankieta.id_ocena_koncowa && verify.id_grupa == ankieta.id_grupa {
        Json(json!({
            "status" : 200,
            "result" : Ankieta::new_feedback(ankieta.into_inner(), &conn),
        }))
    } else {
        Json(json!({
            "status" : 400,
            "result" : "OK",
        }))
    }
}

#[post("/ocena/koncowa/feedback/grupa", format = "application/json", data = "<grupa>")]
pub fn ocena_koncowa_feedback_grupa(conn: DbConn, grupa: Json<AnkietaIdGrupa>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    Json(json!({
        "status" : 200,
        "result" : Ankieta::group_feedback(grupa.into_inner(), &conn),
    }))
}