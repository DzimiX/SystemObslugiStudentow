table! {
    kursy (id) {
        id -> Integer,
        kod -> Varchar,
        nazwa -> Varchar,
        ects -> Integer,
    }
}

table! {
    kursy_grupy (id) {
        id -> Integer,
        id_kursu -> Integer,
        id_zapisy -> Integer,
        kod_grupy -> Varchar,
        termin -> Varchar,
        sala -> Varchar,
    }
}

table! {
    kursy_grupy_ankiety (id) {
        id -> Integer,
        id_grupa -> Integer,
        id_ocena_koncowa -> Integer,
        feedback -> Mediumtext,
    }
}

table! {
    kursy_grupy_ocena_koncowa (id) {
        id -> Integer,
        id_grupa -> Integer,
        id_uczestnik -> Integer,
        ocena -> Float,
        zaakceptowana -> Bool,
        data_zaakceptowana -> Bigint,
        data_ocena -> Bigint,
    }
}

table! {
    kursy_grupy_oceny (id) {
        id -> Integer,
        id_grupa -> Integer,
        id_uczestnik -> Integer,
        ocena -> Float,
        waga -> Float,
        komentarz -> Varchar,
        data -> Bigint,
    }
}

table! {
    kursy_grupy_terminy (id) {
        id -> Integer,
        id_grupa -> Integer,
        temat_zajec -> Varchar,
        komentarz -> Varchar,
        data_start -> Bigint,
        data_koniec -> Bigint,
    }
}

table! {
    kursy_grupy_terminy_obecnosc (id) {
        id -> Integer,
        id_termin -> Integer,
        id_uczestnik -> Integer,
        obecnosc -> Integer,
    }
}

table! {
    kursy_grupy_uczestnicy (id) {
        id -> Integer,
        id_grupa -> Integer,
        id_uczestnik -> Integer,
        czy_prowadzacy -> Bool,
    }
}

table! {
    ogloszenia (id) {
        id -> Integer,
        nadawca -> Varchar,
        temat -> Varchar,
        widok_od -> Bigint,
        widok_do -> Bigint,
        dane -> Mediumtext,
    }
}

table! {
    sprawy (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        temat -> Varchar,
        data -> Bigint,
        status -> Varchar,
        decyzja -> Varchar,
    }
}

table! {
    tokeny (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        id_uprawnienie -> Integer,
        token -> Varchar,
        data -> Bigint,
    }
}

table! {
    uprawnienia (id) {
        id -> Integer,
        nazwa -> Varchar,
    }
}

table! {
    uzytkownicy (id) {
        id -> Integer,
        login -> Varchar,
        imie -> Varchar,
        nazwisko -> Varchar,
    }
}

table! {
    uzytkownicy_dane (id_uzytkownik) {
        id_uzytkownik -> Integer,
        miasto -> Varchar,
        ulica -> Varchar,
        nr_domu -> Varchar,
        kod_pocztowy -> Varchar,
        pesel -> Varchar,
        nr_dowodu -> Varchar,
    }
}

table! {
    uzytkownicy_hasla (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        haslo -> Varchar,
    }
}

table! {
    uzytkownicy_uprawnienia (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        id_uprawnienie -> Integer,
    }
}

table! {
    wiadomosci (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        temat -> Varchar,
        data -> Bigint,
        dane -> Mediumtext,
    }
}

table! {
    wiadomosci_uczestnicy (id) {
        id -> Integer,
        id_wiadomosc -> Integer,
        id_uczestnik -> Integer,
    }
}

table! {
    zapisy (id) {
        id -> Integer,
        nazwa -> Varchar,
        czy_publiczne -> Bool,
    }
}

joinable!(kursy_grupy -> kursy (id_kursu));
joinable!(kursy_grupy -> zapisy (id_zapisy));
joinable!(kursy_grupy_ankiety -> kursy_grupy (id_grupa));
joinable!(kursy_grupy_ankiety -> kursy_grupy_ocena_koncowa (id_ocena_koncowa));
joinable!(kursy_grupy_ocena_koncowa -> kursy_grupy (id_grupa));
joinable!(kursy_grupy_ocena_koncowa -> kursy_grupy_uczestnicy (id_uczestnik));
joinable!(kursy_grupy_oceny -> kursy_grupy (id_grupa));
joinable!(kursy_grupy_oceny -> kursy_grupy_uczestnicy (id_uczestnik));
joinable!(kursy_grupy_terminy -> kursy_grupy (id_grupa));
joinable!(kursy_grupy_terminy_obecnosc -> kursy_grupy_terminy (id_termin));
joinable!(kursy_grupy_terminy_obecnosc -> kursy_grupy_uczestnicy (id_uczestnik));
joinable!(kursy_grupy_uczestnicy -> kursy_grupy (id_grupa));
joinable!(kursy_grupy_uczestnicy -> uzytkownicy (id_uczestnik));
joinable!(sprawy -> uzytkownicy (id_uzytkownik));
joinable!(tokeny -> uprawnienia (id_uprawnienie));
joinable!(tokeny -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_dane -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_hasla -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_uprawnienia -> uprawnienia (id_uprawnienie));
joinable!(uzytkownicy_uprawnienia -> uzytkownicy (id_uzytkownik));
joinable!(wiadomosci -> uzytkownicy (id_uzytkownik));
joinable!(wiadomosci_uczestnicy -> uzytkownicy (id_uczestnik));
joinable!(wiadomosci_uczestnicy -> wiadomosci (id_wiadomosc));

allow_tables_to_appear_in_same_query!(
    kursy,
    kursy_grupy,
    kursy_grupy_ankiety,
    kursy_grupy_ocena_koncowa,
    kursy_grupy_oceny,
    kursy_grupy_terminy,
    kursy_grupy_terminy_obecnosc,
    kursy_grupy_uczestnicy,
    ogloszenia,
    sprawy,
    tokeny,
    uprawnienia,
    uzytkownicy,
    uzytkownicy_dane,
    uzytkownicy_hasla,
    uzytkownicy_uprawnienia,
    wiadomosci,
    wiadomosci_uczestnicy,
    zapisy,
);
