use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use rand::Rng;

use bcrypt;

use chrono::{Local};

use crate::schema::tokeny;

use crate::schema::uzytkownicy_hasla;
use crate::schema::uzytkownicy_uprawnienia;
use crate::schema::uprawnienia;

use crate::schema::miasta;

use crate::schema::wiadomosci;
use crate::schema::wiadomosci_uczestnicy;

use crate::schema::uzytkownicy;

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
        uzytkownicy::table
            .order(uzytkownicy::id.desc())
            .load::<Uzytkownik>(conn)
            .expect("Problem z wczytaniem użytkownika.")
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Uzytkownik> {
        uzytkownicy::table
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

#[derive(Queryable, Serialize)]
pub struct Miasto {
    pub id: i32,
    pub nazwa: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "miasta"]
pub struct NoweMiasto {
    pub nazwa: String,
}

impl Miasto {
    pub fn add(miasto: NoweMiasto, conn: &MysqlConnection) -> bool {
        diesel::insert_into(miasta::table)
            .values(&miasto)
            .execute(conn)
            .is_ok()
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Miasto> {
        miasta::table
            .order(miasta::id.desc())
            .load::<Miasto>(conn)
            .expect("Problem z wczytaniem miast.")
    }
}

#[derive(Queryable, Serialize, Deserialize)]
//#[table_name = "tokeny"]
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

#[derive(Queryable, Serialize, Deserialize)]
pub struct Wiadomosc {
    pub id: i32,
    pub id_uzytkownik: i32,
    pub temat: String,
    pub data: i64,
    pub dane: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "wiadomosci"]
pub struct NowaWiadomosc {
    pub id_uzytkownik: i32,
    pub temat: String,
    pub data: i64,
    pub dane: String,
}

#[derive(Queryable, Serialize)]
pub struct WiadomoscUczestnik {
    pub id : i32,
    pub id_wiadomosc : i32,
    pub id_uczestnik : i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "wiadomosci_uczestnicy"]
pub struct NowaWiadomoscUczestnik {
    pub id_wiadomosc : i32,
    pub id_uczestnik : i32,
}


#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "wiadomosci"]
pub struct WiadomoscId {
    pub id: i32,
}

impl Wiadomosc {

    pub fn add(wiadomosc: NowaWiadomosc, conn: &MysqlConnection) -> bool {
        diesel::insert_into(wiadomosci::table)
            .values(&wiadomosc)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Wiadomosc> {
        wiadomosci::table
            .find(id)
            .load::<Wiadomosc>(conn)
            .expect("Problem z wczytaniem wiadomości.")
    }

    pub fn add_recipient(uczestnik: NowaWiadomoscUczestnik, conn: &MysqlConnection) -> bool {
        diesel::insert_into(wiadomosci_uczestnicy::table)
            .values(&uczestnik)
            .execute(conn)
            .is_ok()
    }

    pub fn get_messages(id_uczestnik : i32, conn : &MysqlConnection) -> Vec<WiadomoscUczestnik> {
        let data = wiadomosci_uczestnicy::table
            .filter(wiadomosci_uczestnicy::id_uczestnik.eq(id_uczestnik))
            .load::<WiadomoscUczestnik>(conn);

        match data {
            Ok(data) => {
                return data;
            },
            Err(_error) => {

                let mut temp : Vec<WiadomoscUczestnik> = Vec::new();
                temp.push(
                    WiadomoscUczestnik {
                        id : -1,
                        id_wiadomosc : -1,
                        id_uczestnik : -1,    
                    }
                );
                return temp;
            }
        };
    }
}