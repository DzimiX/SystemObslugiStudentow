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
use super::OCENY;

use crate::models_user::{Uzytkownik, UzytkownikID, NowyUzytkownik, NoweHaslo};
use crate::models_user::{DaneOsobowe, DaneOsoboweId};
use crate::models_auth::{UprawnienieId, UzytkownikUprawnieniaNowe, UzytkownikUprawnienia, UzytkownikIdUprawnienia};
use crate::models_auth::{AuthLogin, Auth, AuthNowy};
use crate::models_messages::{Wiadomosc, WiadomoscId, NowaWiadomosc, NowaWiadomoscBezDaty, NowaWiadomoscUczestnik};
use crate::models_announcements::{Ogloszenie, OgloszenieNowe, OgloszenieId};
use crate::models_enrollment::{Zapisy, ZapisyNowe, ZapisyId};
use crate::models_courses::{Kurs, KursNowy, KursId};
use crate::models_groups::{Grupa, GrupaNowa, GrupaId, GrupaKursId, GrupaZapisyKursId};
use crate::models_groups::{Uczestnik, UczestnikNowy, UczestnikId, UczestnikGrupaId, UczestnikGrupaUczestnikId};
use crate::models_scores::{Ocena, OcenaNowa, OcenaId, OcenaUczestnikId, OcenaGrupaUczestnikId};
use crate::models_scores::{OcenaKoncowa, OcenaKoncowaNowa, OcenaKoncowaId, OcenaKoncowaUczestnikId, OcenaKoncowaGrupaUczestnikId};
use crate::models_applications::{Sprawy, SprawyId, SprawyNowe};

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
    // niebezpieczne
    
    let mut result = Uzytkownik::add(nowy_uzytkownik.into_inner(), &conn);
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
pub fn uzytkownik_usun(conn: DbConn, uzytkownik: Json<UzytkownikID>) -> Json<Value> { 
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
pub fn uzytkownik_aktualizuj(conn: DbConn, uzytkownik: Json<Uzytkownik>) -> Json<Value> { 
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

#[post("/uzytkownik/uprawnienie/nowe", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_nowe(conn: DbConn, uprawnienie: Json<UzytkownikUprawnieniaNowe>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::add_privilege(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/usun", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_usun(conn: DbConn, uprawnienie: Json<UzytkownikUprawnieniaNowe>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::delete_privilege(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/usun/wszystkie", format = "application/json", data = "<uprawnienie>")]
pub fn uzytkownik_uprawnienie_usun_wszystkie(conn: DbConn, uprawnienie: Json<UzytkownikIdUprawnienia>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::delete_privilege_all(uprawnienie.into_inner(), &conn),
    }))
}

#[post("/uzytkownik/uprawnienie/najwyzsze", format = "application/json", data = "<id>")]
pub fn uzytkownik_uprawnienie_najwyzsze(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_id(id.id, &conn),
    }))
}

#[post("/uzytkownik/uprawnienia", format = "application/json", data = "<id>")]
pub fn uzytkownik_uprawnienia(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_id_all(id.id, &conn),
    }))
}

#[post("/uprawnienie", format = "application/json", data = "<id>")]
pub fn uprawnienie(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    return Json(json!({
        "status" : 200,
        "result" : AuthLogin::get_privilege_name(id.id, &conn),
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

#[post("/uzytkownik/usunhaslo", format = "application/json", data = "<id>")]
pub fn uzytkownik_usun_haslo(conn: DbConn, id: Json<UzytkownikID>, mut cookies: Cookies) -> Json<Value>{
    // niebezpieczne

    AuthLogin::delete_user_token(id.id, &conn);
    Uzytkownik::delete_password(id.id, &conn);

    return Json(json!({
        "status" : 200,
        "result" : "OK",
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
pub fn autoryzacja(conn: DbConn, mut cookies : Cookies) -> Json<Value> {

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

#[post("/kurs", format = "application/json", data = "<kurs>")]
pub fn kurs(conn: DbConn, kurs : Json<KursId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",kurs.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Kurs::get(id, &conn),
    }))
}

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

#[post("/grupa", format = "application/json", data = "<grupa>")]
pub fn grupa(conn: DbConn, grupa : Json<GrupaId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let id : i32 = format!("{}",grupa.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Grupa::get(id, &conn),
    }))
}

#[post("/grupy", format = "application/json", data = "<grupa>")]
pub fn grupy(conn: DbConn, grupa: Json<GrupaKursId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Grupa::get_kurs(grupa.into_inner(),&conn),
    }))
}

#[post("/grupy_zapisy", format = "application/json", data = "<grupa>")]
pub fn grupy_zapisy(conn: DbConn, grupa : Json<GrupaZapisyKursId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Grupa::get_kurs_zapisy(grupa.into_inner(),&conn),
    }))
}

#[post("/grupa/nowe", format = "application/json", data = "<grupa>")]
pub fn grupy_nowe(conn: DbConn, grupa: Json<GrupaNowa>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn grupy_aktualizuj(conn: DbConn, grupa: Json<Grupa>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn grupy_usun(conn: DbConn, id_grupa: Json<GrupaId>, mut cookies : Cookies) -> Json<Value> { 
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

// Uczestnicy grup

#[post("/uczestnicy", format = "application/json", data = "<uczestnik>")]
pub fn uczestnicy_grupa(conn: DbConn, uczestnik: Json<UczestnikGrupaId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Uczestnik::get_grupa_uczestnicy(uczestnik.into_inner(),&conn),
    }))
}

#[post("/uczestnik", format = "application/json", data = "<uczestnik>" )]
pub fn uczestnik_grupy(conn: DbConn, uczestnik: Json<UczestnikId>, mut cookies : Cookies) -> Json<Value> {

    Json(json!({
        "status" : 200,
        "result" : Uczestnik::get_uczestnik(uczestnik.into_inner(),&conn),
    }))
}

#[post("/uczestnicy/nowe", format = "application/json", data = "<uczestnik>")]
pub fn uczestnicy_nowe(conn: DbConn, uczestnik: Json<UczestnikNowy>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn uczestnicy_aktualizuj(conn: DbConn, uczestnik: Json<Uczestnik>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn uczestnik_usun(conn: DbConn, id_uczestnik: Json<UczestnikId>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn uczestnik_grupa_usun(conn: DbConn, uczestnik: Json<UczestnikGrupaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie
    
    // najprostsze obejście braku implementacji klonowania
    let kopia_wejscia = UczestnikGrupaUczestnikId { 
        id_grupa: uczestnik.id_grupa,
        id_uczestnik: uczestnik.id_uczestnik
    };

    let id = Uczestnik::get_id_from_grupa_uczestnik(kopia_wejscia, &conn);
    //println!("{}",id);
    
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

// Oceny dla uczestnika w grupie

#[post("/ocena", format = "application/json", data = "<uczestnik>")]
pub fn ocena_grupa_uczestnik(conn: DbConn, uczestnik : Json<OcenaGrupaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Ocena::get_grupa_student(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/nowa", format = "application/json", data = "<ocena>")]
pub fn ocena_nowa(conn: DbConn, ocena: Json<OcenaNowa>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_aktualizuj(conn: DbConn, ocena: Json<Ocena>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_usun(conn: DbConn, ocena: Json<OcenaId>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_uczestnik_usun(conn: DbConn, ocena : Json<OcenaGrupaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_uczestnik_srednia(conn: DbConn, ocena : Json<OcenaGrupaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
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

// Ocena końcowa dla uczestnika w grupie

#[post("/ocena/koncowa", format = "application/json", data = "<uczestnik>")]
pub fn ocena_koncowa_grupa_uczestnik(conn: DbConn, uczestnik : Json<OcenaKoncowaGrupaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::get_grupa_student(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/akceptuj", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_akceptuj(conn: DbConn, ocena : Json<OcenaKoncowaId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::accept(ocena.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/wszystkie", format = "application/json", data = "<uczestnik>")]
pub fn ocena_koncowa_uczestnik(conn: DbConn, uczestnik : Json<OcenaKoncowaUczestnikId>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : OcenaKoncowa::get_all(uczestnik.into_inner(),&conn),
    }))
}

#[post("/ocena/koncowa/nowa", format = "application/json", data = "<ocena>")]
pub fn ocena_koncowa_nowa(conn: DbConn, ocena: Json<OcenaKoncowaNowa>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_koncowa_aktualizuj(conn: DbConn, ocena: Json<OcenaKoncowa>, mut cookies : Cookies) -> Json<Value> { 
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
pub fn ocena_koncowa_usun(conn: DbConn, ocena: Json<OcenaKoncowaId>, mut cookies : Cookies) -> Json<Value> { 
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

// SPRAWY

#[post("/sprawy", format = "application/json")]
pub fn sprawy(conn: DbConn, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    Json(json!({
        "status" : 200,
        "result" : Sprawy::all(&conn),
    }))
}

#[post("/sprawy/nowe", format = "application/json", data = "<sprawy>")]
pub fn sprawy_nowe(conn: DbConn, sprawy: Json<SprawyNowe>, mut cookies : Cookies) -> Json<Value> { 
    // niebezpieczne

    let mut status = 400;
    if Sprawy::add(sprawy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/sprawy/aktualizuj", format = "application/json", data = "<sprawy>")]
pub fn sprawy_aktualizuj(conn: DbConn, sprawy: Json<Sprawy>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let mut status = 400;
    if Sprawy::update(sprawy.into_inner(), &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}

#[post("/sprawy/pokaz", format = "application/json", data = "<id_sprawy>")]
pub fn sprawy_pokaz(conn: DbConn, id_sprawy: Json<SprawyId>) -> Json<Value> { 
    // niebezpieczne
    let id : i32 = format!("{}",id_sprawy.id).parse::<i32>().unwrap();

    Json(json!({
        "status" : 200,
        "result" : Sprawy::get(id, &conn).first(),
    }))
}

#[post("/sprawy/usun", format = "application/json", data = "<id_sprawy>")]
pub fn sprawy_usun(conn: DbConn, id_sprawy: Json<SprawyId>, mut cookies : Cookies) -> Json<Value> { 
    //niebezpiecznie

    let id : i32 = format!("{}",id_sprawy.id).parse::<i32>().unwrap();

    let mut status = 400;
    if Sprawy::delete(id, &conn) == true {
        status = 200;
    }

    Json(json!({
        "status" : status,
        "result" : "OK",
    }))
}