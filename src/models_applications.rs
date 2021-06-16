use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::sprawy;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "sprawy"]
pub struct Sprawy {
    pub id: i32,
    pub id_uzytkownik: i32, 
    pub temat: String,
    pub data: i64,
    pub status: String,
    pub decyzja: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "sprawy"]
pub struct SprawyNowe {
    pub id_uzytkownik: i32,
    pub temat: String,
    pub data: i64,
    pub status: String,
    pub decyzja: String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "sprawy"]
pub struct SprawyId {
    pub id: i32,

}

impl Sprawy {

    pub fn add(sprawy: SprawyNowe, conn: &MysqlConnection) -> bool {
        diesel::insert_into(sprawy::table)
            .values(&sprawy)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id_uzytkownik: i32, conn: &MysqlConnection) -> Vec<Sprawy> {
        sprawy::table
            .find(id_uzytkownik)
            .load::<Sprawy>(conn)
            .expect("Problem z wczytaniem spraw.")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Sprawy> {
        sprawy::table
            .order(sprawy::id.desc())
            .load::<Sprawy>(conn)
            .expect("Problem z wczytaniem spraw.")
    }

    pub fn delete(id_uzytkownik: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(sprawy::table
            .filter(sprawy::id_uzytkownik.eq(id_uzytkownik))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }
    
    pub fn update(sprawy: Sprawy, conn: &MysqlConnection) -> bool {
       
        let id_uzytkownik = sprawy.id_uzytkownik;
        let temat = sprawy.temat;
        let status = sprawy.status;
        let decyzja = sprawy.decyzja;

        let updated_row = diesel::update(sprawy::table.filter(sprawy::id_uzytkownik.eq(&id_uzytkownik)))
            .set((
                sprawy::temat.eq(temat),
                sprawy::status.eq(status),
                sprawy::decyzja.eq(decyzja),
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }
}