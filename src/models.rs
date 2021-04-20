use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::tokeny;

use crate::schema::uzytkownicy;
use crate::schema::uzytkownicy::dsl::uzytkownicy as all_uzytkownicy;

#[derive(Queryable, Serialize)]
pub struct Auth {
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

impl AuthLogin {
    pub fn authorize(login_data: AuthLogin, conn: &MysqlConnection) -> i32 { // jeżeli pomyślna to zwrot id dla podanego loginu
        let id = uzytkownicy::table
            .filter(uzytkownicy::login.eq(login_data.login))
            .select(uzytkownicy::id)
            .execute(conn)
            .expect("Wystąpił problem");

        return 5
    }
}