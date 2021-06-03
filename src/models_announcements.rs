use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::ogloszenia;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "ogloszenia"]
pub struct Ogloszenie {
    pub id: i32,
    pub nadawca: String,
    pub temat: String,
    pub widok_od: i64,
    pub widok_do: i64,
    pub dane: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "ogloszenia"]
pub struct OgloszenieNowe {
    pub nadawca: String,
    pub temat: String,
    pub widok_od: i64,
    pub widok_do: i64,
    pub dane: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "ogloszenia"]
pub struct OgloszenieId {
    pub id: i32,
}

impl Ogloszenie {

    pub fn add(ogloszenie: OgloszenieNowe, conn: &MysqlConnection) -> bool {
        diesel::insert_into(ogloszenia::table)
            .values(&ogloszenie)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32,conn: &MysqlConnection) -> Vec<Ogloszenie> {
        ogloszenia::table
            .find(id)
            .load::<Ogloszenie>(conn)
            .expect("Problem z wczytaniem ogłoszenia.")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Ogloszenie> {
        ogloszenia::table
            .order(ogloszenia::id.desc())
            .load::<Ogloszenie>(conn)
            .expect("Problem z wczytaniem ogłoszenia.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(ogloszenia::table
            .filter(ogloszenia::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn update(ogloszenie: Ogloszenie, conn: &MysqlConnection) -> bool {
        
        let id = ogloszenie.id;
        let nadawca = ogloszenie.nadawca;
        let temat = ogloszenie.temat;
        let widok_od = ogloszenie.widok_od;
        let widok_do = ogloszenie.widok_do;
        let dane = ogloszenie.dane;

        let updated_row = diesel::update(ogloszenia::table.filter(ogloszenia::id.eq(&id)))
            .set((
                ogloszenia::nadawca.eq(nadawca),
                ogloszenia::temat.eq(temat),
                ogloszenia::widok_od.eq(widok_od),
                ogloszenia::widok_do.eq(widok_do),
                ogloszenia::dane.eq(dane),
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }
    
}