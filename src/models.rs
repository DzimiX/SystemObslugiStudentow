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
use crate::schema::wiadomosci;
use crate::schema::wiadomosci_uczestnicy;
use crate::schema::uzytkownicy;
use crate::schema::ogloszenia;
use crate::schema::zapisy;
use crate::schema::uzytkownicy_dane;
use crate::schema::kursy;
use crate::schema::kursy_grupy;
use crate::schema::kursy_grupy_oceny;
use crate::schema::kursy_grupy_ocena_koncowa;
use crate::schema::kursy_grupy_uczestnicy;
use crate::schema::sprawy;


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
        diesel::delete(kursy_grupy::table
            .filter(kursy_grupy::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
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
        let id_grupa = ocena.id_grupa;
        let id_uczestnik = ocena.id_uczestnik;
        let waga = ocena.waga;
        let komentarz = ocena.komentarz;
        let ocena = ocena.ocena;

        let updated_row = diesel::update(kursy_grupy_oceny::table.filter(kursy_grupy_oceny::id.eq(&id)))
            .set((
                kursy_grupy_oceny::id_grupa.eq(id_grupa),
                kursy_grupy_oceny::id_uczestnik.eq(id_uczestnik),
                kursy_grupy_oceny::ocena.eq(ocena),
                kursy_grupy_oceny::waga.eq(waga),
                kursy_grupy_oceny::komentarz.eq(komentarz)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }
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

impl OcenaKoncowa {

    pub fn add(ocena: OcenaKoncowaNowa, conn: &MysqlConnection) -> bool {
        diesel::insert_into(kursy_grupy_ocena_koncowa::table)
            .values(&ocena)
            .execute(conn)
            .is_ok()
    }

    pub fn update(ocena: OcenaKoncowa, conn: &MysqlConnection) -> bool {
        
        let id = ocena.id;
        let id_grupa = ocena.id_grupa;
        let id_uczestnik = ocena.id_uczestnik;
        let data_ocena = ocena.data_ocena;
        let data_zaakceptowana = ocena.data_zaakceptowana;
        let zaakceptowana = ocena.zaakceptowana;
        let ocena = ocena.ocena;

        let updated_row = diesel::update(kursy_grupy_ocena_koncowa::table.filter(kursy_grupy_ocena_koncowa::id.eq(&id)))
            .set((
                kursy_grupy_ocena_koncowa::id_grupa.eq(id_grupa),
                kursy_grupy_ocena_koncowa::id_uczestnik.eq(id_uczestnik),
                kursy_grupy_ocena_koncowa::data_ocena.eq(data_ocena),
                kursy_grupy_ocena_koncowa::data_zaakceptowana.eq(data_zaakceptowana),
                kursy_grupy_ocena_koncowa::zaakceptowana.eq(zaakceptowana),
                kursy_grupy_ocena_koncowa::ocena.eq(ocena)
            ))
            .execute(conn)
            .is_ok();

        if updated_row == false {
            return false
        }

        return true
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(kursy_grupy_ocena_koncowa::table
            .filter(kursy_grupy_ocena_koncowa::id.eq(id))
        )
        .execute(conn)
        .expect("Błąd.");
    
        return true
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

    pub fn accept(id: i32, conn: &MysqlConnection) -> bool {       
    
        let data_zaakceptowana = Local::now().timestamp();

        let updated_row = diesel::update(kursy_grupy_ocena_koncowa::table.filter(kursy_grupy_ocena_koncowa::id.eq(&id)))
            .set((
                kursy_grupy_ocena_koncowa::zaakceptowana.eq(true),
                kursy_grupy_ocena_koncowa::data_zaakceptowana.eq(data_zaakceptowana)
            ))
            .execute(conn)
            .is_ok();

        return true
    }
}

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
        
        let id = sprawy.id;
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