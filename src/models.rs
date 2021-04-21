use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::tokeny;
use crate::schema::uzytkownicy_hasla;
use crate::schema::uzytkownicy_uprawnienia;
use crate::schema::uprawnienia;


use crate::schema::uzytkownicy;
use crate::schema::uzytkownicy::dsl::uzytkownicy as all_uzytkownicy;

#[derive(Queryable, Serialize)]
pub struct Uzytkownik {
    pub id: i32,
    pub login: String,
    pub imie: String,
    pub nazwisko: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "uzytkownicy"]
pub struct NowyUzytkownik {
    pub login: String,
    pub imie: String,
    pub nazwisko: String,
}

impl Uzytkownik {
    pub fn add(uzytkownik: NowyUzytkownik, conn: &MysqlConnection) -> bool {
        diesel::insert_into(uzytkownicy::table)
            .values(&uzytkownik)
            .execute(conn)
            .is_ok()
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Uzytkownik> {
        all_uzytkownicy
            .order(uzytkownicy::id.desc())
            .load::<Uzytkownik>(conn)
            .expect("error loading the books")
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Uzytkownik> {
        all_uzytkownicy
            .find(id)
            .load::<Uzytkownik>(conn)
            .expect("Problem z wczytaniem użytkownika.")
    }
}

#[derive(Queryable, Serialize)]
//#[table_name = "tokeny"]
pub struct Auth {
    pub id: i32,
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

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "uzytkownicy_hasla"]
pub struct UzytkownikHaslo {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub haslo: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "uzytkownicy_uprawnienia"]
pub struct UzytkownikUprawnienia {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uprawnienia"]
pub struct Uprawnienia {
    pub id: i32,
    pub nazwa: String,
}


impl AuthLogin {

    pub fn getId(login: String, conn: &MysqlConnection) -> i32 { // jeżeli pomyślna to zwrot id dla podanego loginu
        let data : Uzytkownik = uzytkownicy::table
            .filter(uzytkownicy::login.eq(login))
            .first(conn)
            .expect("Błędne dane.");
        return data.id;
    }

    pub fn checkHash(id: i32, hash: String, conn: &MysqlConnection) -> bool { // jeżeli pomyślna to zwrot id dla podanego loginu
        let data : UzytkownikHaslo = uzytkownicy_hasla::table
            .filter(uzytkownicy_hasla::id_uzytkownik.eq(id))
            .first(conn)
            .expect("Błędne dane.");

        println!("data.haslo: {}", data.haslo);
        println!("hash: {}", hash);
        if (data.haslo == hash){
            return true;
        }
        return false;
    }

    
}