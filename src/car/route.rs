use db::client::Connection;
use rocket_contrib::Json;

use car::store::Store;
use car::model::Car;
use car::model::PostRequest;

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
