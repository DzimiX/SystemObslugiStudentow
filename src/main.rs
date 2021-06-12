#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod schema;
mod static_html;

mod models_user;
mod models_auth;
mod models_messages;
mod models_announcements;
mod models_enrollment;
mod models_courses;
mod models_groups;
mod models_scores;
mod models_applications;

mod router_user;
mod router_messages;
mod router_announcements;
mod router_enrollment;
mod router_courses;
mod router_groups;
mod router_scores;
mod router_applications;
mod router_auth;
mod router_groups_frequency;

mod db;

static DATABASE_URL : &str = "mysql://root@127.0.0.1/sosbeta";

static ADMINISTRATOR : i32 = 5;
static PRACOWNIK     : i32 = 4;
static PROWADZACY    : i32 = 3;
static STUDENT       : i32 = 2;
static UZYTKOWNIK    : i32 = 1;

static OCENY : &'static[f32] = &[2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5];

fn main() {
    println!("ZAMIAST http:://127.0.0.1:9090/ KORZYSTAĆ Z http://localhost.:9090/ (związane z polityką cookies - w cookies trzymany jest token z autoryzacji)");
    rocket();
}

fn rocket() {
    let pool = db::init_pool(DATABASE_URL.to_string());

    rocket::ignite()
        .manage(pool)
        .register(catchers![
            static_html::not_found,
        ])
        .mount("/", routes![
            static_html::index,
            static_html::all,
            static_html::all_id,
        ])
        .mount("/api", routes![
            router_user::uzytkownicy_index,
            router_user::uzytkownicy_nowy,
            router_user::uzytkownik,
            router_user::uzytkownik_publiczne,
            router_user::uzytkownik_nowe_haslo,
            router_user::uzytkownik_uprawnienie_najwyzsze,
            router_user::uzytkownik_uprawnienia,
            router_user::uzytkownik_uprawnienie_nowe,
            router_user::uzytkownik_uprawnienie_usun,
            router_user::uzytkownik_uprawnienie_usun_wszystkie,
            router_user::uzytkownik_aktualizuj,
            router_user::uzytkownik_usun,
            router_user::uzytkownik_usun_haslo,
            router_user::uprawnienie,

            router_messages::wiadomosci_nowa,
            router_messages::wiadomosci_pokaz,
            router_messages::wiadomosci_dodajodbiorce,
            router_messages::wiadomosci_domnie,

            router_announcements::ogloszenia,
            router_announcements::ogloszenia_nowe,
            router_announcements::ogloszenia_aktualizuj,
            router_announcements::ogloszenia_usun,

            router_enrollment::zapisy,
            router_enrollment::zapisy_id,
            router_enrollment::zapisy_nowe,
            router_enrollment::zapisy_aktualizuj,
            router_enrollment::zapisy_usun,

            router_user::dane_osobowe_pokaz,
            router_user::dane_osobowe_nowe,
            router_user::dane_osobowe_aktualizuj,
            router_user::dane_osobowe_usun,

            router_courses::kurs,
            router_courses::kursy,
            router_courses::kursy_usun,
            router_courses::kursy_aktualizuj,
            router_courses::kursy_zapisy,
            router_courses::kursy_nowe,

            router_groups::grupa,
            router_groups::grupy,
            router_groups::grupy_zapisy,
            router_groups::grupy_usun,
            router_groups::grupy_aktualizuj,
            router_groups::grupy_nowe,

            router_groups::uczestnik_id,
            router_groups::uczestnicy_grupa,
            router_groups::uczestnicy_nowe,
            router_groups::uczestnicy_aktualizuj,
            router_groups::uczestnik_usun,
            router_groups::uczestnik_grupa_usun,
            router_groups::uczestnik_prowadzone,
            router_groups::uczestnik_grupy,

            router_scores::ocena_grupa_uczestnik,
            router_scores::ocena_nowa,
            router_scores::ocena_aktualizuj,
            router_scores::ocena_usun,
            router_scores::ocena_uczestnik_usun,
            router_scores::ocena_uczestnik_srednia,

            router_scores::ocena_koncowa_akceptuj,
            router_scores::ocena_koncowa_grupa_uczestnik,
            router_scores::ocena_koncowa_uczestnik,
            router_scores::ocena_koncowa_nowa,
            router_scores::ocena_koncowa_aktualizuj,
            router_scores::ocena_koncowa_usun,

            router_applications::sprawy,
            router_applications::sprawy_nowe,
            router_applications::sprawy_aktualizuj,
            router_applications::sprawy_usun,
            router_applications::sprawy_pokaz,
            
            router_auth::logowanie,
            router_auth::autoryzacja,
        ])
        .launch();
}