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

use crate::models::{Uzytkownik, UzytkownikID, NowyUzytkownik, NoweHaslo};
use crate::models::{AuthLogin, Auth, AuthNowy};
use crate::models::{Wiadomosc, WiadomoscId, NowaWiadomosc, NowaWiadomoscBezDaty, NowaWiadomoscUczestnik};
use crate::models::{Ogloszenie, OgloszenieNowe, OgloszenieId};
use crate::models::{Zapisy, ZapisyNowe, ZapisyId};
use crate::models::{DaneOsobowe, DaneOsoboweId};
use crate::models::{Kurs, KursNowy, KursId};

#[post("/uzytkownicy", format = "application/json")]
pub fn uzytkownicy_index(conn: DbConn, mut cookies: Cookies) -> Json<Value> {
    
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
pub fn uzytkownicy_nowy(conn: DbConn, nowy_uzytkownik: Json<NowyUzytkownik>) -> Json<Value> { 
    Json(json!({
        "status" : Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn),
        "result" : Uzytkownik::all(&conn).first(),
    }))
}

#[post("/uzytkownik", format = "application/json", data = "<id>")]
pub fn uzytkownik(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value> {

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

#[post("/uzytkownik/publiczne", format = "application/json", data = "<id>")]
pub fn uzytkownik_publiczne(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value> {

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
pub fn uzytkownik_nowe_haslo(conn: DbConn, nowe_haslo: Json<NoweHaslo>, mut cookies: Cookies) -> Json<Value> {

    let cookie_temp = Cookie::new("token", "False");
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());

    if &token != "False" {
    
        let now_timestamp = Local::now().timestamp();
        let auth : Auth = AuthLogin::check_token(&token, &conn);

        if (auth.token != "False" && now_timestamp < auth.data) && (
            auth.id_uprawnienie == ADMINISTRATOR ||
            auth.id_uzytkownik == nowe_haslo.id_uzytkownik
        ) {
            
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
            let delayed_timestamp = now_timestamp + 36000; // + 36000s = + 10h

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
    let token = String::from(cookies.get("token").unwrap_or(&cookie_temp).value());
    println!("{}", &token);

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

#[post("/wiadomosci/nowa", format = "application/json", data = "<nowa_wiadomosc>")]
pub fn wiadomosci_nowa(conn: DbConn, nowa_wiadomosc: Json<NowaWiadomoscBezDaty>) -> Json<Value> { 
    // niebezpieczne  
    let wiadomosc = NowaWiadomosc {
        id_uzytkownik : nowa_wiadomosc.id_uzytkownik,
        temat : String::from(&nowa_wiadomosc.temat),
        data : Local::now().timestamp(),
        dane : String::from(&nowa_wiadomosc.dane),
    };

    let mut status = 400;
    if Wiadomosc::add(wiadomosc, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : Wiadomosc::get_last_id(&conn),
    }))
}

#[post("/wiadomosci/dodajodbiorce", format = "application/json", data = "<nowy_wiadomosc_uczestnik>")]
pub fn wiadomosci_dodajodbiorce(conn: DbConn, nowy_wiadomosc_uczestnik: Json<NowaWiadomoscUczestnik>) -> Json<Value> { 
    // niebezpieczne
    let mut status = 400;
    if Wiadomosc::add_recipient(nowy_wiadomosc_uczestnik.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/wiadomosci/pokaz", format = "application/json", data = "<id_wiadomosc>")]
pub fn wiadomosci_pokaz(conn: DbConn, id_wiadomosc: Json<WiadomoscId>) -> Json<Value> { 
    // niebezpieczne
    let id : i32 = format!("{}",id_wiadomosc.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Wiadomosc::get(id, &conn).first(),
    }))
}

#[post("/wiadomosci/domnie", format = "application/json", data = "<id>")]
pub fn wiadomosci_domnie(conn: DbConn, id: Json<WiadomoscId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne
    let id_uczestnik : i32 = format!("{}", id.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Wiadomosc::get_messages(id_uczestnik, &conn),
    }))
}

#[post("/ogloszenia", format = "application/json")]
pub fn ogloszenia(conn: DbConn, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Ogloszenie::all(&conn),
    }))
}

#[post("/ogloszenia/nowe", format = "application/json", data = "<ogloszenie>")]
pub fn ogloszenia_nowe(conn: DbConn, ogloszenie: Json<OgloszenieNowe>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Ogloszenie::add(ogloszenie.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ogloszenia/aktualizuj", format = "application/json", data = "<ogloszenie>")]
pub fn ogloszenia_aktualizuj(conn: DbConn, ogloszenie: Json<Ogloszenie>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Ogloszenie::update(ogloszenie.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/ogloszenia/usun", format = "application/json", data = "<id_ogloszenie>")]
pub fn ogloszenia_usun(conn: DbConn, id_ogloszenie: Json<OgloszenieId>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_ogloszenie.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Ogloszenie::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

/// ZAPISY

#[post("/zapisy", format = "application/json")]
pub fn zapisy(conn: DbConn, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Zapisy::all(&conn),
    }))
}

#[post("/zapisy/nowe", format = "application/json", data = "<zapisy>")]
pub fn zapisy_nowe(conn: DbConn, zapisy: Json<ZapisyNowe>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Zapisy::add(zapisy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/zapisy/aktualizuj", format = "application/json", data = "<zapisy>")]
pub fn zapisy_aktualizuj(conn: DbConn, zapisy: Json<Zapisy>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Zapisy::update(zapisy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/zapisy/usun", format = "application/json", data = "<id_zapisy>")]
pub fn zapisy_usun(conn: DbConn, id_zapisy: Json<ZapisyId>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_zapisy.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Zapisy::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

/// DANE OSOBOWE

#[post("/dane_osobowe/pokaz", format = "application/json", data = "<id_uzytkownik>")]
pub fn dane_osobowe_pokaz(conn: DbConn, id_uzytkownik: Json<DaneOsoboweId>) -> Json<Value> { 
    // niebezpieczne
    
    let id : i32 = format!("{}",id_uzytkownik.id_uzytkownik).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : DaneOsobowe::get(id, &conn).first(),
    }))
}

#[post("/dane_osobowe/nowe", format = "application/json", data = "<dane_osobowe>")]
pub fn dane_osobowe_nowe(conn: DbConn, dane_osobowe: Json<DaneOsobowe>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn dane_osobowe_aktualizuj(conn: DbConn, dane_osobowe: Json<DaneOsobowe>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn dane_osobowe_usun(conn: DbConn, id_dane_osobowe: Json<DaneOsoboweId>, mut cookies : Cookies) -> Json<Value> { 
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

/// KURSY

#[post("/kursy", format = "application/json")]
pub fn kursy(conn: DbConn, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Kurs::all(&conn),
    }))
}

#[post("/kursy/nowe", format = "application/json", data = "<kurs_nowy>")]
pub fn kursy_nowe(conn: DbConn, kurs_nowy: Json<KursNowy>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Kurs::add(kurs_nowy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/kursy/aktualizuj", format = "application/json", data = "<kurs>")]
pub fn kursy_aktualizuj(conn: DbConn, kurs: Json<Kurs>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Kurs::update(kurs.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/kursy/usun", format = "application/json", data = "<id_kurs>")]
pub fn kursy_usun(conn: DbConn, id_kurs: Json<KursId>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_kurs.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Kurs::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

// Grupy (do zapisów i realizacji zajęć) dla kursów

