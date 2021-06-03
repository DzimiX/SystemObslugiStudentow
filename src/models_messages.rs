use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::wiadomosci;
use crate::schema::wiadomosci_uczestnicy;

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

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "wiadomosci"]
pub struct NowaWiadomoscBezDaty {
    pub id_uzytkownik: i32,
    pub temat: String,
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

    pub fn get_last_id(conn: &MysqlConnection) -> i32 {
        let data : Result<Wiadomosc,diesel::result::Error> = wiadomosci::table
            .order(wiadomosci::id.desc())
            .first(conn);
        
        match data {
            Ok(data) => return data.id,
            Err(_error) => return -1,
        };
    }

    /*
        let data : Result<Uzytkownik,diesel::result::Error> = uzytkownicy::table
            .filter(uzytkownicy::login.eq(&login))
            .first(conn);
    */

    pub fn add_recipient(uczestnik: NowaWiadomoscUczestnik, conn: &MysqlConnection) -> bool {
        diesel::insert_into(wiadomosci_uczestnicy::table)
            .values(&uczestnik)
            .execute(conn)
            .is_ok()
    }

    pub fn get_messages(id_uczestnik : i32, conn : &MysqlConnection) -> Vec<WiadomoscUczestnik> {
        let data = wiadomosci_uczestnicy::table
            .order(wiadomosci_uczestnicy::id_wiadomosc.desc())
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