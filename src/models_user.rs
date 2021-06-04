use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::uzytkownicy;
use crate::schema::uzytkownicy_hasla;
use crate::schema::uzytkownicy_dane;

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_dane"]
pub struct DaneOsobowe {
    pub id_uzytkownik: i32,
    pub miasto : String,
    pub ulica : String,
    pub nr_domu : String,
    pub kod_pocztowy : String,
    pub pesel : String,
    pub nr_dowodu : String
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "uzytkownicy_dane"]
pub struct DaneOsoboweId {
    pub id_uzytkownik: i32
}

impl Uzytkownik {
    pub fn add(uzytkownik: NowyUzytkownik, conn: &MysqlConnection) -> bool {
        diesel::insert_into(uzytkownicy::table)
            .values(&uzytkownik)
            .execute(conn)
            .is_ok()
    }

    pub fn update(uzytkownik: Uzytkownik, conn: &MysqlConnection) -> bool {
        let id = uzytkownik.id;
        let imie = uzytkownik.imie;
        let nazwisko = uzytkownik.nazwisko;

        let updated_row = diesel::update(uzytkownicy::table.filter(uzytkownicy::id.eq(&id)))
            .set((
                uzytkownicy::imie.eq(imie),
                uzytkownicy::nazwisko.eq(nazwisko),
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(uzytkownicy::table
            .filter(uzytkownicy::id.eq(id))
        )
        .execute(conn)
        .is_ok();

        return true
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Uzytkownik> {
        uzytkownicy::table
            .order(uzytkownicy::id.desc())
            .load::<Uzytkownik>(conn)
            .expect("Problem z wczytaniem użytkowników.")
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

impl DaneOsobowe {

    pub fn add(dane_osobowe: DaneOsobowe, conn: &MysqlConnection) -> bool {
        diesel::insert_into(uzytkownicy_dane::table)
            .values(&dane_osobowe)
            .execute(conn)
            .is_ok()
    }

    pub fn get(id_uzytkownik: i32, conn: &MysqlConnection) -> Vec<DaneOsobowe> {
        uzytkownicy_dane::table
            .find(id_uzytkownik)
            .load::<DaneOsobowe>(conn)
            .expect("Problem z wczytaniem danych użytkownika.")
    }

    pub fn delete(id_uzytkownik: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(uzytkownicy_dane::table
            .filter(uzytkownicy_dane::id_uzytkownik.eq(id_uzytkownik))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
    }

    pub fn update(dane_osobowe: DaneOsobowe, conn: &MysqlConnection) -> bool {
        
        let id_uzytkownik = dane_osobowe.id_uzytkownik;
        let miasto = dane_osobowe.miasto;
        let ulica = dane_osobowe.ulica;
        let nr_domu = dane_osobowe.nr_domu;
        let kod_pocztowy = dane_osobowe.kod_pocztowy;
        let pesel = dane_osobowe.pesel;
        let nr_dowodu =  dane_osobowe.nr_dowodu;

        let updated_row = diesel::update(uzytkownicy_dane::table.filter(uzytkownicy_dane::id_uzytkownik.eq(&id_uzytkownik)))
            .set((
                uzytkownicy_dane::miasto.eq(miasto),
                uzytkownicy_dane::ulica.eq(ulica),
                uzytkownicy_dane::nr_domu.eq(nr_domu),
                uzytkownicy_dane::kod_pocztowy.eq(kod_pocztowy),
                uzytkownicy_dane::pesel.eq(pesel),
                uzytkownicy_dane::nr_dowodu.eq(nr_dowodu)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

}