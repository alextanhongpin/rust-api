#[get("/world")]
pub fn world() -> &'static str {
  "module other!"
}
