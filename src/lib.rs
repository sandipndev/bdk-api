#[macro_use]
extern crate rocket;

mod routes;

#[launch]
pub fn server() -> _ {
    rocket::build().mount("/key", routes![routes::keys::key_generate])
}
