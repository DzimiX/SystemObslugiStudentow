use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use rand::Rng;

use bcrypt;

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

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy"]
pub struct UzytkownikID {
    pub id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "uzytkownicy"]
pub struct NowyUzytkownik {
    pub login: String,
    pub imie: String,
    pub nazwisko: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_hasla"]
pub struct NoweHaslo {
    pub id_uzytkownik: i32,
    pub haslo: String,
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

    pub fn set_password(mut data: NoweHaslo, conn: &MysqlConnection) -> bool {

        let hash = bcrypt::hash(format!("{}",data.haslo), 8);
        let mut hash_string : String = String::from("False");

        match hash {
            Ok(data) => hash_string = String::from(data),
            Err(_error) => print!("Hash error"),
        };
        data.haslo = hash_string;

        diesel::delete(uzytkownicy_hasla::table
            .filter(uzytkownicy_hasla::id_uzytkownik.eq(data.id_uzytkownik))
        )
        .execute(conn)
        .expect("Błąd.");

        diesel::insert_into(uzytkownicy_hasla::table)
            .values(&data)
            .execute(conn)
            .is_ok()
    }
}

#[derive(Queryable, Serialize, Deserialize)]
//#[table_name = "tokeny"]
pub struct Auth {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
    pub token: String,
    //pub data: Datetime,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "tokeny"]
pub struct AuthNowy {
    pub id_uzytkownik: i32,
    pub id_uprawnienie: i32,
    pub token: String,
    //pub data: Datetime,
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

    pub fn get_privilege_id(id: i32, conn: &MysqlConnection) -> i32 {
        let data : UzytkownikUprawnienia = uzytkownicy_uprawnienia::table
            .filter(uzytkownicy_uprawnienia::id_uzytkownik.eq(id))
            .order(uzytkownicy_uprawnienia::id_uprawnienie.desc())
            .first(conn)
            .expect("Błędne dane.");
        
        return data.id_uprawnienie;
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
            Ok(data) => return data,
            Err(_error) => return Auth{
                id : -1,
                id_uzytkownik: -1,
                id_uprawnienie: -1,
                token: String::from("False"),
            },
        };
    }
}