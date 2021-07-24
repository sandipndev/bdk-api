#[macro_use]
extern crate rocket;

#[launch]
pub fn server() -> _ {
    rocket::build()
}
