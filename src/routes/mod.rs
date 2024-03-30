use crate::home::controllers::home;
use rocket::{Build, Rocket};

pub fn deploy() -> Rocket<Build> {
    rocket::build().mount("/", routes![home::index])
}
