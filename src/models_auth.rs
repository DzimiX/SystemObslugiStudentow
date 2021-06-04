use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use rand::Rng;
use chrono::{Local};

use crate::schema::uzytkownicy;
use crate::schema::uzytkownicy_uprawnienia;
use crate::schema::uzytkownicy_hasla;
use crate::schema::uprawnienia;
use crate::schema::tokeny;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Auth {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
    pub token: String,
    pub data: i64,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "tokeny"]
pub struct AuthNowy {
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
    pub token: String,
    pub data: i64,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct AuthLogin {
    pub login: String,
    pub haslo: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "uzytkownicy_hasla"]
pub struct UzytkownikHaslo {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub haslo: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_uprawnienia"]
pub struct UzytkownikUprawnienia {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_uprawnienia"]
pub struct UzytkownikUprawnieniaNowe {
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_uprawnienia"]
pub struct UzytkownikIdUprawnienia {
    pub id_uzytkownik: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uprawnienia"]
pub struct Uprawnienia {
    pub id: i32,
    pub nazwa: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uprawnienia"]
pub struct UprawnienieId {
    pub id: i32
}

#[derive(Queryable, Serialize)]
pub struct Uzytkownik {
    pub id: i32,
    pub login: String,
    pub imie: String,
    pub nazwisko: String,
}

impl AuthLogin {

    pub fn get_id(login: &String, conn: &MysqlConnection) -> i32 { // jeżeli pomyślna to zwrot id dla podanego loginu
        let data : Result<Uzytkownik,diesel::result::Error> = uzytkownicy::table
            .filter(uzytkownicy::login.eq(&login))
            .first(conn);
        match data {
            Ok(data) => return data.id,
            Err(_error) => return -1,
        };
    }

    pub fn check_hash(id: i32, haslo: String, conn: &MysqlConnection) -> bool { // jeżeli pomyślna to zwrot id dla podanego loginu
        let data : UzytkownikHaslo = uzytkownicy_hasla::table
            .filter(uzytkownicy_hasla::id_uzytkownik.eq(&id))
            .first(conn)
            .expect("Błędne dane.");
    
            let verify = bcrypt::verify(&haslo, &data.haslo);
            let verify_bool : bool;

            match verify {
                Ok(data) => verify_bool = bool::from(data),
                Err(_error) => return false,
            };

        if verify_bool {
            return true;
        }
        return false;
    }

    pub fn add_privilege(privilege: UzytkownikUprawnieniaNowe, conn: &MysqlConnection) -> bool {
        diesel::insert_into(uzytkownicy_uprawnienia::table)
            .values(&privilege)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_privilege(privilege: UzytkownikUprawnieniaNowe, conn: &MysqlConnection) -> bool {
        diesel::delete(uzytkownicy_uprawnienia::table
            .filter(uzytkownicy_uprawnienia::id_uzytkownik.eq(privilege.id_uzytkownik))
            .filter(uzytkownicy_uprawnienia::id_uprawnienie.eq(privilege.id_uprawnienie))
        )
        .execute(conn)
        .expect("Błąd.");

        return true
    }

    pub fn delete_privilege_all(privilege: UzytkownikIdUprawnienia, conn: &MysqlConnection) -> bool {
        diesel::delete(uzytkownicy_uprawnienia::table
            .filter(uzytkownicy_uprawnienia::id_uzytkownik.eq(privilege.id_uzytkownik))
        )
        .execute(conn)
        .expect("Błąd.");

        return true
    }

    pub fn get_privilege_id(id: i32, conn: &MysqlConnection) -> i32 { // najwyższy poziom uprawnień użytkownika
        let data : Result<UzytkownikUprawnienia,diesel::result::Error> = uzytkownicy_uprawnienia::table
            .filter(uzytkownicy_uprawnienia::id_uzytkownik.eq(id))
            .order(uzytkownicy_uprawnienia::id_uprawnienie.desc())
            .first(conn);
        
        match data {
            Ok(data) => return data.id_uprawnienie,
            Err(_error) => return -1,
        };
    }

    pub fn get_privilege_id_all(id: i32, conn: &MysqlConnection) -> Vec<UzytkownikUprawnienia> { // najwyższy poziom uprawnień użytkownika
        uzytkownicy_uprawnienia::table
            .filter(uzytkownicy_uprawnienia::id_uzytkownik.eq(id))
            .order(uzytkownicy_uprawnienia::id_uprawnienie.desc())
            .load::<UzytkownikUprawnienia>(conn)
            .expect("Problem z wczytaniem uprawnień użytkownika.")
    }

    pub fn get_privilege_name(id: i32, conn: &MysqlConnection) -> String {
        let data : Result<Uprawnienia,diesel::result::Error> = uprawnienia::table
            .filter(uprawnienia::id.eq(id))
            .first(conn);
 
        match data {
            Ok(data) => return data.nazwa,
            Err(_error) => return String::from("null"),
        };
    }

    pub fn generate_fresh_token(conn: &MysqlConnection) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789)(*&^%$#@!~";

        let mut rng = rand::thread_rng();
        let token: String = (0..512)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        let data : Result<Auth,diesel::result::Error> = tokeny::table
            .filter(tokeny::token.eq(&token))
            .first(conn);

        match data {
            Ok(_data) => return String::from("False"),
            Err(_error) => return token,
        };
    }

    pub fn delete_user_token(id: i32, conn: &MysqlConnection) -> bool {
        
        diesel::delete(tokeny::table
            .filter(tokeny::id_uzytkownik.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");

        return true
    }

    pub fn add_user_token(dane : AuthNowy, conn : &MysqlConnection) -> bool {
        diesel::insert_into(tokeny::table)
            .values(&dane)
            .execute(conn)
            .is_ok()
    }

    pub fn check_token(token : &String, conn : &MysqlConnection) -> Auth {
        let data : Result<Auth,diesel::result::Error> = tokeny::table
            .filter(tokeny::token.eq(&token))
            .first(conn);

        match data {
            Ok(data) => {
                AuthLogin::renew_token(&token, &conn);
                return data;
            },
            Err(_error) => return Auth{
                id : -1,
                id_uzytkownik: -1,
                id_uprawnienie: -1,
                token: String::from("False"),
                data: -1,
            },
        };
    }

    fn renew_token(token : &String, conn : &MysqlConnection) {

        let now_timestamp = Local::now().timestamp();
        let delayed_timestamp = now_timestamp + 36000; // + 36000s = + 10h

        let updated_row = diesel::update(tokeny::table.filter(tokeny::token.eq(&token)))
            .set(tokeny::data.eq(delayed_timestamp))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            println!("Problem połączenia z bazą danych, ale to nie krytyczna funkcja...")
        }
    }
}