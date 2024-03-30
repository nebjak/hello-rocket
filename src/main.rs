#[macro_use]
extern crate rocket;

mod home;
mod routes;

#[launch]
fn rocket() -> _ {
    routes::deploy()
}
