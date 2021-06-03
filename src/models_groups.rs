use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::kursy_grupy;
use crate::schema::kursy_grupy_uczestnicy;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy"]
pub struct Grupa {
    pub id: i32,
    pub id_kursu : i32,
    pub id_zapisy : i32,
    pub kod_grupy : String,
    pub termin : String,
    pub sala : String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy"]
pub struct GrupaNowa {
    pub id_kursu : i32,
    pub id_zapisy : i32,
    pub kod_grupy : String,
    pub termin : String,
    pub sala : String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy"]
pub struct GrupaId {
    pub id: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy"]
pub struct GrupaKursId {
    pub id_kursu: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy"]
pub struct GrupaZapisyKursId {
    pub id_zapisy: i32,
    pub id_kursu: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_uczestnicy"]
pub struct Uczestnik {
    pub id: i32,
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub czy_prowadzacy : bool
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_uczestnicy"]
pub struct UczestnikNowy {
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub czy_prowadzacy : bool
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_uczestnicy"]
pub struct UczestnikId {
    pub id: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_uczestnicy"]
pub struct UczestnikGrupaId {
    pub id_grupa: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_uczestnicy"]
pub struct UczestnikGrupaUczestnikId {
    pub id_grupa: i32,
    pub id_uczestnik: i32, // id_użytkownika
}

impl Grupa {

    pub fn add(grupa: GrupaNowa, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy::table)
            .values(&grupa)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Grupa> {
        kursy_grupy::table
            .find(id)
            .load::<Grupa>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn get_kurs(grupa: GrupaKursId, conn: &MysqlConnection) -> Vec<Grupa> {
        kursy_grupy::table
            .filter(kursy_grupy::id_kursu.eq(grupa.id_kursu))
            .load::<Grupa>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn get_kurs_zapisy(grupa: GrupaZapisyKursId, conn: &MysqlConnection) -> Vec<Grupa> {
        kursy_grupy::table
            .filter(kursy_grupy::id_zapisy.eq(grupa.id_zapisy))
            .filter(kursy_grupy::id_kursu.eq(grupa.id_kursu))
            .load::<Grupa>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        let data = diesel::delete(kursy_grupy::table // Result<usize,diesel::result::Error>
            .filter(kursy_grupy::id.eq(id))
        )
        .execute(conn);

        match data {
            Ok(data) => {
                if data == 0 {
                    return false;
                }
                return true;
            },
            Err(_error) => {
                return false;
            }
        };
    }

    pub fn update(grupa: Grupa, conn: &MysqlConnection) -> bool {
        
        let id = grupa.id;
        let id_kursu = grupa.id_kursu;
        let id_zapisy = grupa.id_zapisy;
        let kod_grupy = grupa.kod_grupy;
        let termin = grupa.termin;
        let sala = grupa.sala;

        let updated_row = diesel::update(kursy_grupy::table.filter(kursy_grupy::id.eq(&id)))
            .set((
                kursy_grupy::id_kursu.eq(id_kursu),
                kursy_grupy::id_zapisy.eq(id_zapisy),
                kursy_grupy::kod_grupy.eq(kod_grupy),
                kursy_grupy::termin.eq(termin),
                kursy_grupy::sala.eq(sala)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

}

impl Uczestnik {

    pub fn add(uczestnik: UczestnikNowy, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy_uczestnicy::table)
            .values(&uczestnik)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Uczestnik> {
        kursy_grupy_uczestnicy::table
            .find(id)
            .load::<Uczestnik>(conn)
            .expect("Problem z wczytaniem uczestników.")
    }

    pub fn get_uczestnik(uczestnik: UczestnikId, conn: &MysqlConnection) -> i32 {
        let data : Result<Uczestnik,diesel::result::Error> = kursy_grupy_uczestnicy::table
            .filter(kursy_grupy_uczestnicy::id.eq(uczestnik.id))
            .first(conn);

        match data {
            Ok(data) => {
                return data.id_uczestnik;
            },
            Err(_error) => return -1,
        };
    }

    pub fn get_grupa_uczestnicy(uczestnik: UczestnikGrupaId, conn: &MysqlConnection) -> Vec<Uczestnik> {
        kursy_grupy_uczestnicy::table
            .filter(kursy_grupy_uczestnicy::id_grupa.eq(uczestnik.id_grupa))
            .order(kursy_grupy_uczestnicy::czy_prowadzacy.desc())
            .load::<Uczestnik>(conn)
            .expect("Problem z wczytaniem uczestników.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_uczestnicy::table
            .filter(kursy_grupy_uczestnicy::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn delete_grupa_uczestnik(uczestnik: UczestnikGrupaUczestnikId, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_uczestnicy::table
            .filter(kursy_grupy_uczestnicy::id_grupa.eq(uczestnik.id_grupa))
            .filter(kursy_grupy_uczestnicy::id_uczestnik.eq(uczestnik.id_uczestnik))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn get_id_from_grupa_uczestnik(uczestnik: UczestnikGrupaUczestnikId, conn: &MysqlConnection) -> i32 {
        let data : Result<Uczestnik,diesel::result::Error> = kursy_grupy_uczestnicy::table
            .filter(kursy_grupy_uczestnicy::id_grupa.eq(uczestnik.id_grupa))
            .filter(kursy_grupy_uczestnicy::id_uczestnik.eq(uczestnik.id_uczestnik))
            .first(conn);

        match data {
            Ok(data) => {
                return data.id;
            },
            Err(_error) => return -1,
        };
    }

    pub fn update(uczestnik: Uczestnik, conn: &MysqlConnection) -> bool {
        
        let id = uczestnik.id;
        let id_grupa = uczestnik.id_grupa;
        let id_uczestnik = uczestnik.id_uczestnik;
        let czy_prowadzacy = uczestnik.czy_prowadzacy;

        let updated_row = diesel::update(kursy_grupy_uczestnicy::table.filter(kursy_grupy_uczestnicy::id.eq(&id)))
            .set((
                kursy_grupy_uczestnicy::id_grupa.eq(id_grupa),
                kursy_grupy_uczestnicy::id_uczestnik.eq(id_uczestnik),
                kursy_grupy_uczestnicy::czy_prowadzacy.eq(czy_prowadzacy)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

}