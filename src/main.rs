#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod other;

// use rocket::request::FromForm;
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
// #[get("/users/<id>", format = "application/json")]
// fn user(id: usize) -> Json<User> {}

#[get("/search?<search>")]
fn search(search: Search) -> String {
    format!("got search query: {}", search.query)
}

fn main() {
    // println!("Hello, world!");
    rocket::ignite()
        .mount("/", routes![index, hello, other::world, new_user, search])
        .launch();
}
