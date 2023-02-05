#[get("/health")]
pub fn health_check() -> &'static str {
    "OK"
}
