#[get("/")]
pub fn index() -> &'static str {
    "System Obsługi Studentów API v0.1"
}

#[get("/testy")]
pub fn testy() -> &'static str {
    println!("x");
    // baza danych odpytanie
    "testy"
}

#[get("/testsql")]
pub fn test_sql() -> &'static str {
    "tu będzie wspaniała odpowiedź z bazy danych"
}

#[get("/login/<login>/<password>")]
pub fn login(login: String, password: String) -> String {
    return format!("Przesłany login: {} przesłane hasło: {}", login, password);
}

//use crate::routes::user;

