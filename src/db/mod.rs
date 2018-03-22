extern crate r2d2_postgres;

use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};

use std::ops::Deref;

pub type Pool = r2d2::Pool<PostgresConnectionManager>;
static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn connect() -> Pool {
	// posgres://postgres@172.20.21.232/rust_api
    let manager =
        PostgresConnectionManager::new(DATABASE_URL, TlsMode::None)
            .unwrap();
    r2d2::Pool::new(manager).expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<PostgresConnectionManager>);

// Attemps to retrieve a single connection from the managed database pool.
// If no pool is currently managed, fails with an `InternalServerError` status.
// If no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
		let pool = request.guard::<State<Pool>>()?;
		match pool.get() {
			Ok(conn) => Outcome::Success(Connection(conn)),
			Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
		}
	}
}

// For the convenience of using an &Connectio as an PostgresConnectionManager
impl Deref for Connection {
	type Target = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
