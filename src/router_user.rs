use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::{Cookie, Cookies};
use chrono::{Local};

use super::ADMINISTRATOR;
use super::PRACOWNIK;
use super::PROWADZACY;
use super::STUDENT;
use super::UZYTKOWNIK;

use crate::models_user::{Uzytkownik, UzytkownikID, NowyUzytkownik, NoweHaslo};
use crate::models_user::{DaneOsobowe, DaneOsoboweId};
use crate::models_auth::{UzytkownikUprawnieniaNowe, UzytkownikIdUprawnienia};
use crate::models_auth::{AuthLogin, Auth, AuthNowy};

#[post("/uzytkownicy", format = "application/json")]
pub fn uzytkownicy_index(conn: DbConn, cookies: Cookies) -> Json<Value> {
    
    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());

    if &token != "False" {
    
        let now_timestamp = Local::now().timestamp();
        let auth : Auth = AuthLogin::check_token(&token, &conn);

        if (auth.token != "False" && now_timestamp < auth.data) && (
            auth.id_uprawnienie == ADMINISTRATOR ||
            auth.id_uprawnienie == PRACOWNIK ||
            auth.id_uprawnienie == PROWADZACY ||
            auth.id_uprawnienie == STUDENT ||
            auth.id_uprawnienie == UZYTKOWNIK
        ) {
            let uzytkownicy = Uzytkownik::all(&conn);

            return Json(json!({
                "status" : 200,
                "result" : uzytkownicy,
            }))
        } else {
            if now_timestamp < auth.data || auth.token == "False" { // token przeterminowany, trzeba się zalogować ponownie
                return Json(json!({
                    "status" : 401,
                    "result" : "Unauthorized",
                }));
            } else { // po prostu brak uprawnień do zasobu
                return Json(json!({
                    "status" : 403,
                    "result" : "Forbidden",
                }))
            }
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
}

#[post("/uzytkownik/nowy", format = "application/json", data = "<nowy_uzytkownik>")]
pub fn uzytkownicy_nowy(conn: DbConn, nowy_uzytkownik: Json<NowyUzytkownik>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    
    let result = Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn);
    if result == true {
        return Json(json!({
            "status" : 200,
            "result" : Uzytkownik::all(&conn).first(),
        }));
    } else {
        return Json(json!({
            "status" : 400,
            "result" : result,
        }));
    }
}

#[post("/uzytkownik/usun", format = "application/json", data = "<uzytkownik>")]
pub fn uzytkownik_usun(conn: DbConn, uzytkownik: Json<UzytkownikID>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    
    let mut status = 400;
    if Uzytkownik::delete(uzytkownik.id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uzytkownik/aktualizuj", format = "application/json", data = "<uzytkownik>")]
pub fn uzytkownik_aktualizuj(conn: DbConn, uzytkownik: Json<Uzytkownik>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    
    let mut status = 400;
    if Uzytkownik::update(uzytkownik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uzytkownik", format = "application/json", data = "<id>")]
pub fn uzytkownik(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());

    if &token != "False" {
    
        let now_timestamp = Local::now().timestamp();
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        
        if (auth.token != "False" && now_timestamp < auth.data) && (
            auth.id_uprawnienie > 4 ||
            auth.id_uzytkownik == id.id
        ) {
            
            let result = Uzytkownik::get(id.id, &conn);
            let status = if result.is_empty() { 404 } else { 200 };

            // rozbudować pobieranie dodatkowych informacji o użytkowniku
            
            if status == 200 {
                return Json(json!({
                    "status" : 200,
                    "result" : result.get(0),
                }))
            } else {
                return Json(json!({
                    "status" : 404,
                    "result" : "Not Found",
                }))
            }
        } else if auth.token == "False" {
            return Json(json!({
                "status" : 401,
                "result" : "Unauthorized",
            }))
        } else {
            return Json(json!({
                "status" : 403,
                "result" : "Forbidden",
            }))
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
}

#[post("/uzytkownik/uprawnienie/nowe", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_nowe(conn: DbConn, uprawnienie: Json<UzytkownikUprawnieniaNowe>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::add_privilege(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/usun", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_usun(conn: DbConn, uprawnienie: Json<UzytkownikUprawnieniaNowe>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::delete_privilege(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/usun/wszystkie", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_usun_wszystkie(conn: DbConn, uprawnienie: Json<UzytkownikIdUprawnienia>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::delete_privilege_all(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/najwyzsze", format = "application/json", data = "<id>")]
pub fn uzytkownik_uprawnienie_najwyzsze(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_id(id.id, &conn),
    }))
}

#[post("/uzytkownik/uprawnienia", format = "application/json", data = "<id>")]
pub fn uzytkownik_uprawnienia(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_id_all(id.id, &conn),
    }))
}

#[post("/uprawnienie", format = "application/json", data = "<id>")]
pub fn uprawnienie(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_name(id.id, &conn),
    }))
}

#[post("/uzytkownik/publiczne", format = "application/json", data = "<id>")]
pub fn uzytkownik_publiczne(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value> {
    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());
    if &token != "False" {
        let now_timestamp = Local::now().timestamp();
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        if auth.token != "False" && now_timestamp < auth.data {
            
            let result = Uzytkownik::get(id.id, &conn);
            let status = if result.is_empty() { 404 } else { 200 };
            
            if status == 200 {
                return Json(json!({
                    "status" : 200,
                    "result" : result.get(0),
                }))
            } else {
                return Json(json!({
                    "status" : 404,
                    "result" : "Not Found",
                }))
            }
        } else if auth.token == "False" {
            return Json(json!({
                "status" : 401,
                "result" : "Unauthorized",
            }))
        } else {
            return Json(json!({
                "status" : 403,
                "result" : "Forbidden",
            }))
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
}

#[post("/uzytkownik/nowehaslo", format = "application/json", data = "<nowe_haslo>")]
pub fn uzytkownik_nowe_haslo(conn: DbConn, nowe_haslo: Json<NoweHaslo>, cookies: Cookies) -> Json<Value> {
    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());
    if &token != "False" {
        let now_timestamp = Local::now().timestamp();
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        if (auth.token != "False" && now_timestamp < auth.data) && (
            auth.id_uprawnienie == ADMINISTRATOR ||
            auth.id_uzytkownik == nowe_haslo.id_uzytkownik
        ){
            
            let wynik = Uzytkownik::set_password(nowe_haslo.into_inner(), &conn);
            
            if wynik {
                return Json(json!({ // udało się
                    "status" : 200,
                    "result" : "OK",
                }))
            } else {
                return Json(json!({ // nie udało się
                    "status" : 400,
                    "result" : "Error",
                }))
            }
        } else {
            return Json(json!({ // zabronione (brak uprawnień)
                "status" : 403,
                "result" : "Forbidden",
            }))
        }
    } else {
        return Json(json!({ // zabronione (brak autoryzacji)
            "status" : 401,
            "result" : "Unauthorized",
        }))
    }
}

#[post("/uzytkownik/usunhaslo", format = "application/json", data = "<id>")]
pub fn uzytkownik_usun_haslo(conn: DbConn, id: Json<UzytkownikID>, cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    AuthLogin::delete_user_token(id.id, &conn);
    Uzytkownik::delete_password(id.id, &conn);

    return Json(json!({
        "status" : 200,
        "result" : "OK",
    }))
}

#[post("/dane_osobowe/pokaz", format = "application/json", data = "<id_uzytkownik>")]
pub fn dane_osobowe_pokaz(conn: DbConn, id_uzytkownik: Json<DaneOsoboweId>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne
    
    let id : i32 = format!("{}",id_uzytkownik.id_uzytkownik).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : DaneOsobowe::get(id, &conn).first(),
    }))
}

#[post("/dane_osobowe/nowe", format = "application/json", data = "<dane_osobowe>")]
pub fn dane_osobowe_nowe(conn: DbConn, dane_osobowe: Json<DaneOsobowe>, cookies: Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if DaneOsobowe::add(dane_osobowe.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/dane_osobowe/aktualizuj", format = "application/json", data = "<dane_osobowe>")]
pub fn dane_osobowe_aktualizuj(conn: DbConn, dane_osobowe: Json<DaneOsobowe>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if DaneOsobowe::update(dane_osobowe.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/dane_osobowe/usun", format = "application/json", data = "<id_dane_osobowe>")]
pub fn dane_osobowe_usun(conn: DbConn, id_dane_osobowe: Json<DaneOsoboweId>, cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_dane_osobowe.id_uzytkownik).parse::<i32>().unwrap();

    let mut status = 400;
    if DaneOsobowe::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/uzytkownik/nowy/rejestracja", format = "application/json", data = "<nowy_uzytkownik>")]
pub fn uzytkownicy_nowy_rejestracja(conn: DbConn, nowy_uzytkownik: Json<NowyUzytkownik>) -> Json<Value> { 
    // teoretycznie niebezpieczne 
    
    let result = Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn);
    if result == true {
        return Json(json!({
            "status" : 200,
            "result" : Uzytkownik::all(&conn).first(),
        }));
    } else {
        return Json(json!({
            "status" : 400,
            "result" : result,
        }));
    }
}
#[post("/dane_osobowe/nowe/rejestracja", format = "application/json", data = "<dane_osobowe>")]
pub fn dane_osobowe_nowe_rejestracja(conn: DbConn, dane_osobowe: Json<DaneOsobowe>) -> Json<Value> { 
    // teoretycznie niebezpieczne 

    let mut status = 400;
    if DaneOsobowe::add(dane_osobowe.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}
#[post("/uzytkownik/nowehaslo/rejestracja", format = "application/json", data = "<nowe_haslo>")]
pub fn uzytkownik_nowe_haslo_rejestracja(conn: DbConn, nowe_haslo: Json<NoweHaslo>) -> Json<Value> {
    // teoretycznie niebezpieczne 
    Uzytkownik::set_password(nowe_haslo.into_inner(), &conn);
            
    return Json(json!({
        "status" : 200,
        "result" : "OK",
    }))
}