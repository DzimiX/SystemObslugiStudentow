use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;

use rocket::http::{Cookie, Cookies};

use super::ADMINISTRATOR;
use super::PRACOWNIK;
use super::PROWADZACY;
use super::STUDENT;
use super::UZYTKOWNIK;

use crate::models::{Uzytkownik, UzytkownikID, NowyUzytkownik, NoweHaslo};
use crate::models::{AuthLogin, Auth, AuthNowy};

#[get("/uzytkownicy", format = "application/json")]
pub fn uzytkownicy_index(conn: DbConn, mut cookies: Cookies) -> Json<Value> {
    
    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get_private("token").unwrap_or(cookie_temp).value());

    if &token != "False" {
    
        let auth : Auth = AuthLogin::check_token(&token, &conn);

        if auth.token != "False" && (
            auth.id_uprawnienie == ADMINISTRATOR ||
            auth.id_uprawnienie == PRACOWNIK
        ) {
            let uzytkownicy = Uzytkownik::all(&conn);

            return Json(json!({
                "status" : 200,
                "result" : uzytkownicy,
            }))
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
}

#[post("/uzytkownicy", format = "application/json", data = "<nowy_uzytkownik>")]
pub fn uzytkownicy_nowy(conn: DbConn, nowy_uzytkownik: Json<NowyUzytkownik>) -> Json<Value> {
    //println!("{}",String::from(token));
    
    Json(json!({
        "status" : Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn),
        "result" : Uzytkownik::all(&conn).first(),
    }))
}

#[post("/uzytkownik", format = "application/json", data = "<id>")]
pub fn uzytkownik(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get_private("token").unwrap_or(cookie_temp).value());

    if &token != "False" {
    
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        
        if auth.token != "False" && (auth.id_uprawnienie > 4 || auth.id_uzytkownik == id.id) {
            
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
        
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
}

#[post("/uzytkownik/nowehaslo", format = "application/json", data = "<nowe_haslo>")]
pub fn uzytkownik_nowe_haslo(conn: DbConn, nowe_haslo: Json<NoweHaslo>, mut cookies: Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get_private("token").unwrap_or(cookie_temp).value());

    if &token != "False" {
    
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        
        if auth.token != "False" && (auth.id_uprawnienie > 4 || auth.id_uzytkownik == nowe_haslo.id_uzytkownik) {
            
            let wynik = Uzytkownik::set_password(nowe_haslo.into_inner(), &conn);
            
            if wynik {
                return Json(json!({
                    "status" : 200,
                    "result" : "OK",
                }))
            } else {
                return Json(json!({
                    "status" : 400,
                    "result" : "Error",
                }))
            }
        }
    }

    return Json(json!({
        "status" : 401,
        "result" : "Unauthorized",
    }))
    
}

#[post("/login", format = "application/json", data = "<login_dane>")] 
pub fn logowanie(conn: DbConn, login_dane: Json<AuthLogin>, mut cookies : Cookies) -> Json<Value> { // 2h zabawy czemu 

    let login : String = format!("{}",login_dane.login);
    let id_uzytkownik : i32 = AuthLogin::get_id(&login, &conn);

    if id_uzytkownik != -1 {

        let haslo = format!("{}",login_dane.haslo);
        
        let zgadza : bool = AuthLogin::check_hash(id_uzytkownik, haslo, &conn);

        if zgadza {

            let id_uprawnienie : i32 = AuthLogin::get_privilege_id(id_uzytkownik, &conn);
                
            /*
            Ogarnąć jakoś datetime do db kiedyś

            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();
            let time = datetime.format("%d/%m/%Y %T");
            */

            let mut token : String = AuthLogin::generate_fresh_token(&conn);
            
            while &token == "False"{
                token = AuthLogin::generate_fresh_token(&conn);
            }

            AuthLogin::delete_user_token(id_uzytkownik, &conn);

            let nowy_auth = AuthNowy {
                id_uzytkownik : id_uzytkownik,
                id_uprawnienie : id_uprawnienie,
                token : String::from(&token),
            };

            AuthLogin::add_user_token(nowy_auth, &conn);

            if &token != "False" {

                cookies.add_private(Cookie::new("token", String::from(&token)));
                cookies.add_private(Cookie::new("id", id_uzytkownik.to_string()));
                cookies.add_private(Cookie::new("id_uprawnienie", id_uprawnienie.to_string()));

                return Json(json!({
                    "status" : 200,
                    "result" : "Authorized"
                    //{
                        //"id_uzytkownik": id_uzytkownik,
                        //"id_uprawnienie": id_uprawnienie,
                        //"token": token
                    //},
                }))
            }
        }
    }

    return Json(json!({
        "status" : 400,
        "result" : "Bad Request",
    }))

}

#[get("/auth", format = "application/json")] 
pub fn autoryzacja(conn: DbConn, mut cookies : Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get_private("token").unwrap_or(cookie_temp).value());

    let zwrotka : Auth = AuthLogin::check_token(&token, &conn);

    if zwrotka.token != "False" {
        return Json(json!({
            "status" : 200,
            "result" : "Authorized",
        }))
    } else {
        return Json(json!({
            "status" : 401,
            "result" : "Unauthorized",
        }))
    }
}