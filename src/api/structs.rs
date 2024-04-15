use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchParams {
	pub query: String,
}

#[derive(Serialize)]
pub struct Failure<'a> {
	pub success: bool,
	pub error: &'a str,
}

#[derive(Serialize)]
pub struct SingleSuccess {
	pub success: bool,
}
