use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use rocket_contrib::Json;
use db::Connection;

pub struct CarService {
	pool: Connection,
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
	pub fn new(pool: Connection) -> Self {
		CarService { pool }
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
pub fn route(conn: Connection) -> Json<Vec<Car>> {
	let svc = CarService::new(conn);
	Json(svc.all())
}

#[post("/cars", format = "application/json", data = "<car>")]
pub fn post_car(car: Json<PostCarRequest>, conn: Connection) -> Json<Car> {
	println!("{:?}", car);
	let name = car.name.clone();
	let svc = CarService::new(conn);
	// svc.create_table();
	Json(svc.insert_car(name))
}