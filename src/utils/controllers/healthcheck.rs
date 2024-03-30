use crate::utils::services::healthcheck::run_healthcheck;

#[get("/")]
pub fn index() -> &'static str {
    run_healthcheck()
}
