use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::kursy_grupy_terminy;
//use crate::schema::kursy_grupy_terminy_obecnosc;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct Termin {
    pub id: i32,
    pub id_grupa : i32,
    pub temat_zajec : String,
    pub komentarz : String,
    pub data_start : i64,
    pub data_koniec : i64,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminNowy {
    pub id_grupa : i32,
    pub temat_zajec : String,
    pub komentarz : String,
    pub data_start : i64,
    pub data_koniec : i64,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminId {
    pub id: i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminIdGrupa {
    pub id_grupa: i32
}

/*

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminObecnosc {
    pub id: i32,
    pub id_termin : i32,
    pub id_uczestnik : i32,
    pub obecnosc : i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminObecnoscNowa {
    pub id_grupa : i32,
    pub data_start : i64,
    pub data_koniec : i64,
    pub temat_zajec : String,
    pub komentarz : String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy_grupy_terminy"]
pub struct TerminObecnoscId {
    pub id: i32
}

*/

impl Termin {

    pub fn add(termin: TerminNowy, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy_terminy::table)
            .values(&termin)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Termin> {
        kursy_grupy_terminy::table
            .find(id)
            .load::<Termin>(conn)
            .expect("Problem z wczytaniem uczestników.")
    }

    pub fn get_grupa(id: i32, conn: &MysqlConnection) -> Vec<Termin>{
        kursy_grupy_terminy::table
            .filter(kursy_grupy_terminy::id_grupa.eq(id))
            .load::<Termin>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        let data = diesel::delete(kursy_grupy_terminy::table
            .filter(kursy_grupy_terminy::id.eq(id))
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

    pub fn update(termin: Termin, conn: &MysqlConnection) -> bool {
        
        let id = termin.id;
        let id_grupa = termin.id_grupa;
        let data_start = termin.data_start;
        let data_koniec = termin.data_koniec;
        let temat_zajec = termin.temat_zajec;
        let komentarz = termin.komentarz;

        let updated_row = diesel::update(kursy_grupy_terminy::table.filter(kursy_grupy_terminy::id.eq(&id)))
            .set((
                kursy_grupy_terminy::id_grupa.eq(id_grupa),
                kursy_grupy_terminy::data_start.eq(data_start),
                kursy_grupy_terminy::data_koniec.eq(data_koniec),
                kursy_grupy_terminy::temat_zajec.eq(temat_zajec),
                kursy_grupy_terminy::komentarz.eq(komentarz)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }
        return true
    }

}