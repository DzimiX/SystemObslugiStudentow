use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::{Uzytkownik, NowyUzytkownik};
use crate::models::{AuthLogin, Auth};

#[get("/uzytkownicy", format = "application/json")]
pub fn uzytkownicy_index(conn: DbConn) -> Json<Value> {
    let uzytkownicy = Uzytkownik::all(&conn);

    Json(json!({
        "status" : 200,
        "result" : uzytkownicy,
    }))
}

#[post("/uzytkownicy", format = "application/json", data = "<nowy_uzytkownik>")]
pub fn uzytkownicy_nowy(conn: DbConn, nowy_uzytkownik: Json<NowyUzytkownik>) -> Json<Value> {
    Json(json!({
        "status" : Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn),
        "result" : Uzytkownik::all(&conn).first(),
    }))
}

#[get("/uzytkownicy/<id>", format = "application/json")]
pub fn uzytkownicy_id(conn: DbConn, id:i32) -> Json<Value> {
    
    let result = Uzytkownik::get(id, &conn);
    let status = if result.is_empty() { 404 } else { 200};

    Json(json!({
        "status" : status,
        "result" : result.get(0),
    }))
}

/*
Jakieś 2h zabawy czemu poniższy endpoint nie działa i rzuca:

the trait bound `rocket_contrib::json::Json<models::AuthLogin>: rocket::data::FromData<'_>` is not satisfied

tldr: 
(1) dodać w modelu do makra derive Deserialize
(2) zapamiętać że czasami szybkość działania kodu nie zrekompensuje spędzonego nad nim czasu i python wcale nie jest zły
(3) używać oryginalnych dodatków w ide które mają pełnego helpa a nie polecanych przez hindusa z bazaru
*/

#[post("/login", format = "application/json", data = "<login_dane>")] 
pub fn logowanie(conn: DbConn, login_dane: Json<AuthLogin>) -> Json<Value> { // 2h zabawy czemu 

    // kiedyś się to zrobi

    Json(json!({
        "status" : "A",
        "result" : "B",
    }))
}
