mod crew;
mod principals;

use crate::{
	api::{structs::Failure, utils::generate_sqid},
	models::Names,
	schema::names::dsl::{names, nconst},
};
use crew::CrewRow;
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, SelectDsl},
	ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use principals::PrincipalRow;
use serde::Serialize;
use warp::{
	http::StatusCode,
	reply::{json, with_status, Json, WithStatus},
};

#[derive(Serialize)]
pub struct ResultObject {
	pub success: bool,
	pub nconst: i32,
	pub primaryname: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthyear: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deathyear: Option<i32>,
	pub crew: Vec<CrewRow>,
	pub principals: Vec<PrincipalRow>,
}

pub fn view_person(connection: &mut PgConnection, id: &str) -> WithStatus<Json> {
	let sqid = generate_sqid();
	let id = sqid.decode(id);
	let id = match id.get(0) {
		Some(id) => *id as i32,
		None => {
			return with_status(
				json(&Failure {
					success: false,
					error: "Invalid ID",
				}),
				StatusCode::BAD_REQUEST,
			);
		}
	};
	let res = match SelectDsl::select(FilterDsl::filter(names, nconst.eq(id)), Names::as_select())
		.load(connection)
	{
		Ok(a) => a,
		Err(_) => {
			return with_status(
				json(&Failure {
					success: false,
					error: "Something's Wrong",
				}),
				StatusCode::INTERNAL_SERVER_ERROR,
			);
		}
	};
	let res = match res.get(0) {
		Some(a) => a,
		None => {
			return with_status(
				json(&Failure {
					success: false,
					error: "Not Found",
				}),
				StatusCode::NOT_FOUND,
			);
		}
	};
	let crew = match crew::get_crew(&id, connection) {
		Ok(a) => a,
		Err(_) => {
			return with_status(
				json(&Failure {
					success: false,
					error: "Something's Wrong",
				}),
				StatusCode::INTERNAL_SERVER_ERROR,
			);
		}
	};
	let principals = match principals::get_principals(&id, connection) {
		Ok(a) => a,
		Err(_) => {
			return with_status(
				json(&Failure {
					success: false,
					error: "Something's Wrong",
				}),
				StatusCode::INTERNAL_SERVER_ERROR,
			);
		}
	};
	let res = ResultObject {
		success: true,
		nconst: id,
		primaryname: res.primaryname.to_string(),
		birthyear: res.birthyear,
		deathyear: res.deathyear,
		crew,
		principals,
	};
	return with_status(json(&res), StatusCode::OK);
}
