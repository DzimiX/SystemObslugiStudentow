use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::{Cookie, Cookies};
use chrono::{Local};

use crate::models_auth::{AuthLogin, Auth, AuthNowy, AuthIdUzytkownik};

#[post("/login", format = "application/json", data = "<login_dane>")] 
pub fn logowanie(conn: DbConn, login_dane: Json<AuthLogin>, mut cookies : Cookies) -> Json<Value> { // 2h zabawy czemu 

    let login : String = format!("{}",login_dane.login);
    let id_uzytkownik : i32 = AuthLogin::get_id(&login, &conn);

    if id_uzytkownik != -1 {

        let haslo = format!("{}",login_dane.haslo);
        
        let zgadza : bool = AuthLogin::check_hash(id_uzytkownik, haslo, &conn);

        if zgadza {

            let id_uprawnienie : i32 = AuthLogin::get_privilege_id(id_uzytkownik, &conn);
            let now_timestamp = Local::now().timestamp();
            let hours_10_in_seconds : i64 = 36000;
            let delayed_timestamp = now_timestamp + hours_10_in_seconds;
            let mut token : String = AuthLogin::generate_fresh_token(&conn);
            
            while &token == "False"{
                token = AuthLogin::generate_fresh_token(&conn);
            }

            AuthLogin::delete_user_token(id_uzytkownik, &conn);

            let nowy_auth = AuthNowy {
                id_uzytkownik : id_uzytkownik,
                id_uprawnienie : id_uprawnienie,
                token : String::from(&token),
                data : delayed_timestamp,
            };

            AuthLogin::add_user_token(nowy_auth, &conn);

            if &token != "False" {

                // secure(true) tylko dla HTTPS!!!
                cookies.add(Cookie::build("token", String::from(&token)).domain("localhost.").path("/").secure(false).finish());
                cookies.add(Cookie::build("id", id_uzytkownik.to_string()).domain("localhost.").path("/").secure(false).finish());
                cookies.add(Cookie::build("id_uprawnienie", id_uprawnienie.to_string()).domain("localhost.").path("/").secure(false).finish());

                return Json(json!({
                    "status" : 200,
                    "result" : "Authorized"
                }))
            }
        }
    }

    return Json(json!({
        "status" : 400,
        "result" : "Bad Request",
    }))
}

#[post("/auth", format = "application/json")] 
pub fn autoryzacja(conn: DbConn, cookies : Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());

    let auth : Auth = AuthLogin::check_token(&token, &conn);

    let now_timestamp = Local::now().timestamp();

    if auth.token != "False" && now_timestamp < auth.data {
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

#[post("/auth/aktywne", format = "application/json", data = "<uzytkownik>")] 
pub fn czy_aktywne(conn: DbConn, uzytkownik: Json<AuthIdUzytkownik>, cookies : Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());

    let auth : Auth = AuthLogin::check_token(&token, &conn);
    
    let now_timestamp = Local::now().timestamp();

    if auth.token != "False" && now_timestamp < auth.data {
        
        let czy_aktywne = AuthLogin::check_account_enabled(uzytkownik.id_uzytkownik, &conn);
        let mut status = 400;
        if czy_aktywne {
            status = 200;
        }
        return Json(json!({
            "status" : status,
            "result" : czy_aktywne,
        }))
    } else {
        return Json(json!({
            "status" : 401,
            "result" : "Unauthorized",
        }))
    }
}