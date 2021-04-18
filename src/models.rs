use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::uzytkownicy;
use crate::schema::uzytkownicy::dsl::uzytkownicy as wszyscy_uzytkownicy;

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
        wszyscy_uzytkownicy
            .order(uzytkownicy::id.desc())
            .load::<Uzytkownik>(conn)
            .expect("error loading the books")
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Uzytkownik> {
        wszyscy_uzytkownicy
            .find(id)
            .load::<Uzytkownik>(conn)
            .expect("Problem z wczytaniem użytkownika.")
    }
}