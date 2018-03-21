extern crate r2d2_postgres;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use std::ops::Deref;
use rocket_contrib::Json;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};

type Pool = r2d2::Pool<PostgresConnectionManager>;
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

pub struct CarService {
	cost: f64,
	pool: DbConn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCarRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
	id: Uuid,
	name: String,
	created_at: DateTime<Utc>,
}

impl CarService {
	pub fn new(cost: f64, pool: DbConn) -> Self {
		CarService { cost, pool }
	}

	pub fn charge(&self) -> &f64 {
		&self.cost
	}

	pub fn create_table(&self) {
		self
			.pool
			.execute(
				"CREATE TABLE IF NOT EXISTS car (
				 	id 						UUID PRIMARY KEY,
				 	name 					VARCHAR NOT NULL,
					created_at    TIMESTAMP WITH TIME ZONE
			)",
				&[],
			)
			.unwrap();
	}

	pub fn insert_car(&self, name: String) -> Car {
		let new_car = Car {
			id: Uuid::new_v4(),
			created_at: Utc::now(),
			name: name,
		};

		self
			.pool
			.execute(
				"INSERT INTO car (id, name, created_at) VALUES ($1, $2, $3)",
				&[&new_car.id, &new_car.name, &new_car.created_at],
			)
			.unwrap();
		new_car
	}

	pub fn all(&self) -> Vec<Car> {
		let mut cars: Vec<Car> = vec![];
		for row in &self
			.pool
			.query("SELECT id, name, created_at FROM car", &[])
			.unwrap()
		{
			let car = Car {
				id: row.get(0),
				name: row.get(1),
				created_at: row.get(2),
			};
			cars.push(car);
		}
		println!("got cars {:?}", cars);
		cars
	}
}

#[get("/cars")]
pub fn route(conn: DbConn) -> Json<Vec<Car>> {
	let svc = CarService::new(5.0, conn);
	Json(svc.all())
}
// pub fn route() -> &'static str {
// 	"car route"
// }

#[post("/cars", format = "application/json", data = "<car>")]
pub fn post_car(car: Json<PostCarRequest>, conn: DbConn) -> Json<Car> {
	println!("{:?}", car);
	let name = car.name.clone();
	let svc = CarService::new(4.50, conn);
	// svc.create_table();
	Json(svc.insert_car(name))
}

// #[get("/users/<id>", format = "application/json")]
// fn user(id: usize) -> Json<User> {}

// #[post("/users", format = "application/json", data = "<user>")]
// fn new_user(user: Json<User>) -> Json<User> {
// 	user
// }
