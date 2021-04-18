use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::{Uzytkownik, NowyUzytkownik};

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
        "status" : 200,
        "result" : result.get(0),
    }))
}


#[get("/testy")]
pub fn testy() -> &'static str {
    println!("x");
    // baza danych odpytanie
    "testy"
}

#[get("/testsql")]
pub fn test_sql() -> &'static str {
    "tu będzie wspaniała odpowiedź z bazy danych"
}

#[get("/login/<login>/<password>")]
pub fn login(login: String, password: String) -> String {
    return format!("Przesłany login: {} przesłane hasło: {}", login, password);
}

