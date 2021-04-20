table! {
    miasta (id) {
        id -> Integer,
        nazwa -> Varchar,
    }
}

table! {
    ogloszenia (id) {
        id -> Integer,
        id_nadawca -> Integer,
        nadawca -> Varchar,
        temat -> Varchar,
        widok_od -> Datetime,
        widok_do -> Datetime,
    }
}

table! {
    ogloszenia_tresc (id) {
        id -> Integer,
        id_ogloszenie -> Integer,
        dane -> Mediumtext,
    }
}

table! {
    tokeny (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        id_uprawnienie -> Integer,
        token -> Varchar,
        data -> Datetime,
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
    uzytkownicy_dane (id) {
        id -> Integer,
        id_uzytkownik -> Integer,
        id_miasto -> Integer,
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
        nadawca -> Varchar,
        temat -> Varchar,
        priorytet -> Integer,
    }
}

table! {
    wiadomosci_tresc (id) {
        id -> Integer,
        id_wiadomosc -> Integer,
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

joinable!(ogloszenia -> uzytkownicy (id_nadawca));
joinable!(ogloszenia_tresc -> ogloszenia (id_ogloszenie));
joinable!(tokeny -> uprawnienia (id_uprawnienie));
joinable!(tokeny -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_dane -> miasta (id_miasto));
joinable!(uzytkownicy_dane -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_hasla -> uzytkownicy (id_uzytkownik));
joinable!(uzytkownicy_uprawnienia -> uprawnienia (id_uprawnienie));
joinable!(uzytkownicy_uprawnienia -> uzytkownicy (id_uzytkownik));
joinable!(wiadomosci_tresc -> wiadomosci (id_wiadomosc));
joinable!(wiadomosci_uczestnicy -> uzytkownicy (id_uczestnik));
joinable!(wiadomosci_uczestnicy -> wiadomosci (id_wiadomosc));

allow_tables_to_appear_in_same_query!(
    miasta,
    ogloszenia,
    ogloszenia_tresc,
    tokeny,
    uprawnienia,
    uzytkownicy,
    uzytkownicy_dane,
    uzytkownicy_hasla,
    uzytkownicy_uprawnienia,
    wiadomosci,
    wiadomosci_tresc,
    wiadomosci_uczestnicy,
);
