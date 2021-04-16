#[get("/courses")]
pub fn page() -> String{
  return format!("Test kursy");
}