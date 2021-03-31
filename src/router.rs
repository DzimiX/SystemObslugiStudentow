#[get("/")]
pub fn index() -> &'static str {
    "System Obsługi Studentów API v0.1"
}

#[get("/testy")]
pub fn testy() -> &'static str {
    "testy"
}

#[get("/testsql")]
pub fn test_sql() -> &'static str {
    "tu będzie wspaniała odpowiedź z bazy danych"
}