use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::zapisy;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "zapisy"]
pub struct Zapisy {
    pub id: i32,
    pub nazwa: String,
    pub czy_publiczne: bool
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "zapisy"]
pub struct ZapisyNowe {
    pub nazwa: String,
    pub czy_publiczne: bool
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "zapisy"]
pub struct ZapisyId {
    pub id: i32,
}

impl Zapisy {

    pub fn add(zapisy: ZapisyNowe, conn: &MysqlConnection) -> bool {
        diesel::insert_into(zapisy::table)
            .values(&zapisy)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Zapisy> {
        zapisy::table
            .find(id)
            .load::<Zapisy>(conn)
            .expect("Problem z wczytaniem zapisów.")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Zapisy> {
        zapisy::table
            .order(zapisy::id.desc())
            .load::<Zapisy>(conn)
            .expect("Problem z wczytaniem zapisów.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(zapisy::table
            .filter(zapisy::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn update(zapisy: Zapisy, conn: &MysqlConnection) -> bool {
        
        let id = zapisy.id;
        let nazwa = zapisy.nazwa;
        let czy_publiczne = zapisy.czy_publiczne;

        let updated_row = diesel::update(zapisy::table.filter(zapisy::id.eq(&id)))
            .set((
                zapisy::nazwa.eq(nazwa),
                zapisy::czy_publiczne.eq(czy_publiczne),
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

}