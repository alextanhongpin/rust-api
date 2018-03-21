#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

// Import external crates
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;

// Import modules
mod other;
mod car;

// Use modules
// use car::CarService;
// use std::thread;
use std::ops::Deref;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
// use uuid::Uuid;
// use chrono::offset::Utc;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

// Take from environment variable
// static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an PostgresConnectionManager
impl Deref for DbConn {
    type Target = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;
    //    = note: expected type `&r2d2_postgres::PostgresConnectionManager`
    //   found type `&r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>`

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// An alias to the type for a pool of PostgresConnectionManager
type Pool = r2d2::Pool<PostgresConnectionManager>;

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

// #[get("/tasks")]
// fn get_tasks(conn: DbConn) ->

fn main() {
    // let car = CarService::new(4.50, pool);
    // let cost = car.charge();
    // println!("got charged: {}", cost);

    // println!("Hello, world!");
    rocket::ignite()
        .manage(init_pool())
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

fn init_pool() -> Pool {
    let manager =
        PostgresConnectionManager::new("posgres://postgres@192.168.8.102/rust_api", TlsMode::None)
            .unwrap();
    r2d2::Pool::new(manager).expect("db pool") //.unwrap()
}
