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
