pub struct CarService {
	cost: f64
}

impl CarService {
	pub fn new(cost: f64) -> Self {
		CarService{
			cost
		}
	}
	pub fn charge(&self) -> &f64 {
		&self.cost
	}
}

#[get("/car")]
pub fn route() -> &'static str {
  "car route"
}
