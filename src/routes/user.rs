#[get("/user")]
pub fn page() -> String{
  return format!("Test user");
}