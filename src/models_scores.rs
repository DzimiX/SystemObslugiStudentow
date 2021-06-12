use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use chrono::{Local};

use crate::schema::kursy_grupy_oceny;
use crate::schema::kursy_grupy_ocena_koncowa;
use crate::schema::kursy_grupy_ankiety;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_oceny"]
pub struct Ocena {
    pub id: i32,
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub ocena : f32,
    pub waga : f32,
    pub komentarz : String,
    pub data : i64
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_oceny"]
pub struct OcenaNowa {
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub ocena : f32,
    pub waga : f32,
    pub komentarz : String,
    pub data : i64
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_oceny"]
pub struct OcenaId {
    pub id: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_oceny"]
pub struct OcenaUczestnikId {
    pub id_uczestnik: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_oceny"]
pub struct OcenaGrupaUczestnikId {
    pub id_grupa: i32,
    pub id_uczestnik: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_ocena_koncowa"]
pub struct OcenaKoncowa {
    pub id: i32,
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub ocena : f32,
    pub zaakceptowana : bool,
    pub data_zaakceptowana : i64,
    pub data_ocena : i64
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_ocena_koncowa"]
pub struct OcenaKoncowaNowa {
    pub id_grupa : i32,
    pub id_uczestnik : i32,
    pub ocena : f32,
    pub zaakceptowana : bool,
    pub data_zaakceptowana : i64,
    pub data_ocena : i64
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_ocena_koncowa"]
pub struct OcenaKoncowaId {
    pub id: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_ocena_koncowa"]
pub struct OcenaKoncowaUczestnikId {
    pub id_uczestnik: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_ocena_koncowa"]
pub struct OcenaKoncowaGrupaUczestnikId {
    pub id_grupa: i32,
    pub id_uczestnik: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_ankiety"]
pub struct Ankieta {
    pub id: i32,
    pub id_grupa: i32,
    pub id_ocena_koncowa: i32,
    pub feedback: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_ankiety"]
pub struct AnkietaNowa {
    pub id_grupa: i32,
    pub id_ocena_koncowa: i32,
    pub feedback: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_ankiety"]
pub struct AnkietaTekst {
    pub feedback: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "kursy_grupy_ankiety"]
pub struct AnkietaIdGrupa {
    pub id_grupa: i32
}

impl Ocena {

    pub fn add(ocena: OcenaNowa, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy_oceny::table)
            .values(&ocena)
            .execute(conn)
            .is_ok()
    }

    pub fn get(uczestnik: OcenaUczestnikId, conn: &MysqlConnection) -> Vec<Ocena> {
        kursy_grupy_oceny::table
            .find(uczestnik.id_uczestnik)
            .load::<Ocena>(conn)
            .expect("Problem z wczytaniem ocen.")
    }

    pub fn get_grupa_student(uczestnik: OcenaGrupaUczestnikId, conn: &MysqlConnection) -> Vec<Ocena> {
        kursy_grupy_oceny::table
            .filter(kursy_grupy_oceny::id_grupa.eq(uczestnik.id_grupa))
            .filter(kursy_grupy_oceny::id_uczestnik.eq(uczestnik.id_uczestnik))
            .load::<Ocena>(conn)
            .expect("Problem z wczytaniem ocen.")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Ocena> {
        kursy_grupy_oceny::table
            .order(kursy_grupy_oceny::id.desc())
            .load::<Ocena>(conn)
            .expect("Problem z wczytaniem ocen.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_oceny::table
            .filter(kursy_grupy_oceny::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn delete_grupa_uczestnik(ocena: OcenaGrupaUczestnikId, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_oceny::table
            .filter(kursy_grupy_oceny::id_grupa.eq(ocena.id_grupa))
            .filter(kursy_grupy_oceny::id_uczestnik.eq(ocena.id_uczestnik))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn update(ocena: Ocena, conn: &MysqlConnection) -> bool {
        
        let id = ocena.id;
        let waga = ocena.waga;
        let komentarz = ocena.komentarz;
        let ocena = ocena.ocena;
        let data = Local::now().timestamp();

        let updated_row = diesel::update(kursy_grupy_oceny::table.filter(kursy_grupy_oceny::id.eq(&id)))
            .set((
                kursy_grupy_oceny::ocena.eq(ocena),
                kursy_grupy_oceny::waga.eq(waga),
                kursy_grupy_oceny::komentarz.eq(komentarz),
                kursy_grupy_oceny::data.eq(data)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

    pub fn average(uczestnik: OcenaGrupaUczestnikId, conn: &MysqlConnection) -> f32 {

        let data = kursy_grupy_oceny::table
            .filter(kursy_grupy_oceny::id_grupa.eq(uczestnik.id_grupa))
            .filter(kursy_grupy_oceny::id_uczestnik.eq(uczestnik.id_uczestnik))
            .load::<Ocena>(conn)
            .expect("Błąd podczas wczytania ocen.");

        let ilosc = data.len();
        let mut srednia = 0.0;
        let mut waga = 0.0;

        if ilosc != 0 {
            for post in data {
                srednia += post.ocena*post.waga;
                waga += post.waga;
            }
            srednia = srednia / waga;
        } else {
            srednia = -1.0;
        }

        return srednia;
    }
}

impl OcenaKoncowa {

    pub fn add(ocena: OcenaKoncowaNowa, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy_ocena_koncowa::table)
            .values(&ocena)
            .execute(conn)
            .is_ok()
    }

    pub fn get_by_id(ocena: OcenaKoncowaId, conn: &MysqlConnection) -> OcenaKoncowa {
        let data : Result<OcenaKoncowa,diesel::result::Error> = kursy_grupy_ocena_koncowa::table
            .find(ocena.id)
            .first(conn);

        let error_data = OcenaKoncowa {
            id: -1,
            id_grupa : -1,
            id_uczestnik : -1,
            ocena : -1.0,
            zaakceptowana : false,
            data_zaakceptowana : -1,
            data_ocena : -1
        };

        match data {
            Ok(data) => {
                return data;
            },
            Err(_error) => return error_data,
        };
    }

    pub fn update(ocena: OcenaKoncowa, conn: &MysqlConnection) -> bool {
        
        let zaakceptowana = OcenaKoncowa::get(ocena.id, &conn);

        if zaakceptowana == false {
            let id = ocena.id;
            let data_ocena = Local::now().timestamp();
            let ocena = ocena.ocena;

            let updated_row = diesel::update(kursy_grupy_ocena_koncowa::table.filter(kursy_grupy_ocena_koncowa::id.eq(&id)))
                .set((
                    kursy_grupy_ocena_koncowa::data_ocena.eq(data_ocena),
                    kursy_grupy_ocena_koncowa::ocena.eq(ocena)
                ))
                .execute(conn)
                .is_ok();

            if updated_row == false {
                return false;
            }

            return true;
        } else {
            return false;
        }
    }

    pub fn accept(ocena: OcenaKoncowaId, conn: &MysqlConnection) -> bool {
        
        let zaakceptowana = OcenaKoncowa::get(ocena.id, &conn);

        if zaakceptowana == false {
            let data_zaakceptowana = Local::now().timestamp();
        
            let updated_row = diesel::update(kursy_grupy_ocena_koncowa::table.filter(kursy_grupy_ocena_koncowa::id.eq(ocena.id)))
                .set((
                    kursy_grupy_ocena_koncowa::zaakceptowana.eq(true),
                    kursy_grupy_ocena_koncowa::data_zaakceptowana.eq(data_zaakceptowana),
                ))
                .execute(conn)
                .is_ok();

            if updated_row == false {
                return false;
            }

            return true;
        } else {
            return false;
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_ocena_koncowa::table
            .filter(kursy_grupy_ocena_koncowa::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> bool {
        let data : Result<OcenaKoncowa,diesel::result::Error> = kursy_grupy_ocena_koncowa::table
            .filter(kursy_grupy_ocena_koncowa::id.eq(id))
            .first(conn);

        match data {
            Ok(data) => return data.zaakceptowana,
            Err(_error) => return false,
        };
    }
    
    pub fn get_all(uczestnik: OcenaKoncowaUczestnikId, conn: &MysqlConnection) -> Vec<OcenaKoncowa> {
        kursy_grupy_ocena_koncowa::table
            .find(uczestnik.id_uczestnik)
            .load::<OcenaKoncowa>(conn)
            .expect("Problem z wczytaniem ocen.")
    }

    pub fn get_grupa_student(uczestnik: OcenaKoncowaGrupaUczestnikId, conn: &MysqlConnection) -> Vec<OcenaKoncowa> {
        kursy_grupy_ocena_koncowa::table
            .filter(kursy_grupy_ocena_koncowa::id_grupa.eq(uczestnik.id_grupa))
            .filter(kursy_grupy_ocena_koncowa::id_uczestnik.eq(uczestnik.id_uczestnik))
            .load::<OcenaKoncowa>(conn)
            .expect("Problem z wczytaniem ocen.")
    }
}

impl Ankieta {

    pub fn new_feedback(feedback: AnkietaNowa, conn: &MysqlConnection) -> bool {       
        diesel::insert_into(kursy_grupy_ankiety::table)
            .values(&feedback)
            .execute(conn)
            .is_ok()
    }

    pub fn group_feedback(group: AnkietaIdGrupa, conn: &MysqlConnection) -> Vec<Ankieta> {
        kursy_grupy_ankiety::table
            .filter(kursy_grupy_ankiety::id_grupa.eq(group.id_grupa))
            .load::<Ankieta>(conn)
            .expect("Problem z wczytaniem ankiet.")
    }

}