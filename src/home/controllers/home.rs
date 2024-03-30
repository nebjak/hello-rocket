use crate::home::services::home::hello;

#[get("/")]
pub fn index() -> &'static str {
    hello()
}
