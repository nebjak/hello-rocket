#[macro_use]
extern crate rocket;

mod home;
mod routes;
mod utils;

#[launch]
fn rocket() -> _ {
    routes::deploy()
}
