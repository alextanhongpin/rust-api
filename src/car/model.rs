use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
	pub id: Uuid,
	pub name: String,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
	pub name: String,
}