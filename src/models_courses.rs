use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::kursy;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy"]
pub struct Kurs {
    pub id: i32,
    pub kod : String,
    pub nazwa : String,
    pub ects : i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy"]
pub struct KursNowy {
    pub kod : String,
    pub nazwa : String,
    pub ects : i32
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "kursy"]
pub struct KursId {
    pub id: i32
}

impl Kurs {

    pub fn add(kurs: KursNowy, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy::table)
            .values(&kurs)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id: i32, conn: &MysqlConnection) -> Vec<Kurs> {
        kursy::table
            .find(id)
            .load::<Kurs>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Kurs> {
        kursy::table
            .order(kursy::id.desc())
            .load::<Kurs>(conn)
            .expect("Problem z wczytaniem kursów.")
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy::table
            .filter(kursy::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn update(kurs: Kurs, conn: &MysqlConnection) -> bool {
        
        let id = kurs.id;
        let kod = kurs.kod;
        let nazwa = kurs.nazwa;
        let ects = kurs.ects;

        let updated_row = diesel::update(kursy::table.filter(kursy::id.eq(&id)))
            .set((
                kursy::kod.eq(kod),
                kursy::nazwa.eq(nazwa),
                kursy::ects.eq(ects)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

}