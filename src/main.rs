#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

// Import external crates
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;

// Import modules
mod other;
mod car;


// Use modules
use car::CarService;
use std::thread;
use r2d2_postgres::{PosgresConnectionManager, TlsMode};
use uuid:Uuid;
use chrono::offset::Utc;
use rocket_contrib::Json;

// An alias to the type for a pool of PostgresConnectionManager
type Pool = r2d2::PooledConnectionManager<PostgresConnectionManager>;

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
    let car = CarService::new(4.50);
    let cost = car.charge();
    println!("got charged: {}", cost);

    // println!("Hello, world!");
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, hello, other::world, car::route, new_user, search])
        .launch();
}


fn init_pool() -> Pool {
    let manager = PostgresConnectionManager::new("posgres://postgres@localhost/rustweb", TlsMode::None)
        .unwrap();
    r2d2::Pool::new(manager).unwrap()
}