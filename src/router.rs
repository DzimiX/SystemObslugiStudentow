use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use std::option::Option;

use rocket::http::{Cookie, Cookies};

use crate::models::{Uzytkownik, NowyUzytkownik};
use crate::models::{AuthLogin, Auth, AuthNowy, AuthToken};

#[get("/uzytkownicy", format = "application/json")]
pub fn uzytkownicy_index(conn: DbConn, mut cookies: Cookies) -> Json<Value> {
    
    
    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());
    println!("{}", token );

    if &token != "False" {
        
        println!("DZIAŁAM");
        let auth : Auth = AuthLogin::check_token(&token, &conn);
        println!("{}", auth.token);
        println!("{}", auth.id_uprawnienie);

        if auth.token != "False" && auth.id_uprawnienie > 4 {
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
pub fn logowanie(conn: DbConn, login_dane: Json<AuthLogin>, mut cookies : Cookies) -> Json<Value> { // 2h zabawy czemu 

    let login : String = format!("{}",login_dane.login);
    let id_uzytkownik : i32 = AuthLogin::get_id(&login, &conn);

    if id_uzytkownik != -1 {
        // odpytać teraz uzytkownicy_hasla czy hash hasła się zgadza
        let hash = format!("{}",login_dane.haslo);
        // TODO konwersja       ^^^^^^^^^^^^^^^^ jako HASH!!!
        
        //println!("hash: {}", hash);
        
        let zgadza : bool = AuthLogin::check_hash(id_uzytkownik, hash, &conn);

        if zgadza {
            // użytkownik istnieje, hasło się zgadza
            // trzeba sformułować templatke tokenu i go dodać do bazy i zwrócić użytkownikowi
            
            // jeżeli się zgadza to zrobić token i go dać użytkownikowi
            let id_uprawnienie : i32 = AuthLogin::get_privilege_id(id_uzytkownik, &conn);
            //println!("ID_UPRAWNIENIE: {}", id_uprawnienie);

                
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
            
            //println!("{}", &token);

            AuthLogin::delete_user_token(id_uzytkownik, &conn);

            let nowy_auth = AuthNowy {
                id_uzytkownik : id_uzytkownik,
                id_uprawnienie : id_uprawnienie,
                token : String::from(&token),
            };

            AuthLogin::add_user_token(nowy_auth, &conn);

            if &token != "False" {

                cookies.add(Cookie::new("token", String::from(&token)));

                return Json(json!({
                    "status" : 200,
                    "result" : {
                        "id_uzytkownik": id_uzytkownik,
                        "id_uprawnienie": id_uprawnienie,
                        "token": token
                    },
                }))
            }
        }
    }

    return Json(json!({
        "status" : 400,
        "result" : "Bad Request",
    }))

}

#[post("/auth", format = "application/json", data = "<token_data>")] 
pub fn autoryzacja(conn: DbConn, token_data: Json<AuthToken>) -> Json<Value> {

    let token_parsed = &token_data.token;

    let zwrotka : Auth = AuthLogin::check_token(&token_parsed, &conn);

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