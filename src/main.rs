#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

// Import external crates
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;

mod other;
mod db;
mod car;

use rocket_contrib::Json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[derive(FromForm)]
struct Search {
    query: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Dynamic segments
#[get("/hi/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {}. You are {} years old.", name, age)
}

#[post("/users", format = "application/json", data = "<user>")]
fn new_user(user: Json<User>) -> Json<User> {
    user
}

#[get("/search?<search>")]
fn search(search: Search) -> String {
    format!("got search query: {}", search.query)
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount(
            "/",
            routes![
                index,
                hello,
                other::world,
                car::route,
                car::post_car,
                new_user,
                search
            ],
        )
        .launch();
}
