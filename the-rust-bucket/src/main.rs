#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

#[get("/light")]
fn light() -> &'static str {
    "Light"
}

#[get("/")]
fn base() -> &'static str {
    "Base Page"
}

// dynamic path example
#[get("/hello/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}!", name)
}

// complex dynamic path
#[get("/hello/<name>/<age>/<cool>")]
fn hello_details(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, you are not cool", name)
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello_name, hello_details, light, base])
}