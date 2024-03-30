use crate::home::controllers::home;
use crate::utils::controllers::healthcheck;
use rocket::{Build, Rocket};

pub fn deploy() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![home::index])
        .mount("/healthcheck", routes![healthcheck::index])
}
