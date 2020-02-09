#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
#[macro_use] extern crate rocket;

use rocket_contrib::json::{Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

let mut bands = vec![];

#[derive(Deserialize)]
struct Band {
    name: String,
    average_age: String,
}

#[put("/band", data = "<band>")]
fn band(band: Json<Band>) -> &'static str {
    "Todo!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, band]).launch();
}
