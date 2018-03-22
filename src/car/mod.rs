use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use rocket_contrib::Json;

use db::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
	name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
	id: Uuid,
	name: String,
	created_at: DateTime<Utc>,
}

pub struct Store {}

impl Store {
	pub fn create_table(conn: Connection) {
			conn
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

	pub fn create(conn: Connection, name: String) -> Car {
		let new_car = Car {
			id: Uuid::new_v4(),
			created_at: Utc::now(),
			name: name,
		};

		conn
			.execute(
				"INSERT INTO car (id, name, created_at) VALUES ($1, $2, $3)",
				&[&new_car.id, &new_car.name, &new_car.created_at],
			)
			.unwrap();
		new_car
	}

	pub fn all(conn: Connection) -> Vec<Car> {
		let mut cars: Vec<Car> = vec![];
		for row in &conn
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
		cars
	}
}

// Routes

#[get("/cars")]
pub fn get_cars(conn: Connection) -> Json<Vec<Car>> {
	Json(Store::all(conn))
}

#[post("/cars", format = "application/json", data = "<car>")]
pub fn post_car(car: Json<PostRequest>, conn: Connection) -> Json<Car> {
	let name = car.name.clone();
	// Store.create_table();
	Json(Store::create(conn, name))
}