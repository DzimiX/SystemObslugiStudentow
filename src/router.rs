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

#[post("/login", format = "application/json", data = "<login_dane>")] 
pub fn logowanie(conn: DbConn, login_dane: Json<AuthLogin>) -> Json<Value> { // 2h zabawy czemu 

    let login = format!("{}",login_dane.login);
    let id = AuthLogin::getId(login, &conn);

    if(id != -1){
        // odpytać teraz uzytkownicy_hasla czy hash hasła się zgadza
        let hash = format!("{}",login_dane.haslo);
        // TODO konwersja       ^^^^^^^^^^^^^^^^ jako HASH!!!
        
        //println!("hash: {}", hash);
        
        let zgadza = AuthLogin::checkHash(id, hash, &conn);

        if(zgadza){
            // użytkownik istnieje, hasło się zgadza
            // trzeba sformułować templatke tokenu i go dodać do bazy i zwrócić użytkownikowi
            
            // jeżeli się zgadza to zrobić token i go dać użytkownikowi
            
            
            
            return Json(json!({
                "status" : 200,
                "result" : "OK, token itd...",
            }))
        }
    }

    return Json(json!({
        "status" : 400,
        "result" : "Bad Request",
    }))

    
}
