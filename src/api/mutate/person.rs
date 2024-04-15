use crate::{
	api::{
		structs::{Failure, SingleSuccess},
		utils::generate_sqid,
	},
	models::{Names, NamesNoId},
	schema::names::dsl::{names, nconst},
};
use diesel::{delete, insert_into, pg::PgConnection, update, ExpressionMethods, RunQueryDsl};
use serde::Serialize;
use warp::{
	http::StatusCode,
	reply::{json, with_status, Json, WithStatus},
};

#[derive(Serialize)]
struct IDSuccess {
	success: bool,
	nconst: String,
}

pub fn post_person(connection: &mut PgConnection, person: NamesNoId) -> WithStatus<Json> {
	let sqid = generate_sqid();
	match insert_into(names)
		.values(&person)
		.get_result::<Names>(connection)
	{
		Ok(a) => match sqid.encode(&[a.nconst as u64]) {
			Ok(b) => with_status(
				json(&IDSuccess {
					success: true,
					nconst: b,
				}),
				StatusCode::OK,
			),
			Err(_) => with_status(
				json(&Failure {
					success: false,
					error: "Something's Wrong",
				}),
				StatusCode::INTERNAL_SERVER_ERROR,
			),
		},
		Err(_) => with_status(
			json(&Failure {
				success: false,
				error: "Something's Wrong",
			}),
			StatusCode::INTERNAL_SERVER_ERROR,
		),
	}
}

pub fn put_person(connection: &mut PgConnection, id: &str, person: NamesNoId) -> WithStatus<Json> {
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
	match update(names)
		.set(&person)
		.filter(nconst.eq(id))
		.execute(connection)
	{
		Ok(_) => with_status(json(&SingleSuccess { success: true }), StatusCode::OK),
		Err(_) => with_status(
			json(&Failure {
				success: false,
				error: "Something's Wrong",
			}),
			StatusCode::INTERNAL_SERVER_ERROR,
		),
	}
}

pub fn delete_person(connection: &mut PgConnection, id: &str) -> WithStatus<Json> {
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
	match delete(names).filter(nconst.eq(id)).execute(connection) {
		Ok(_) => with_status(json(&SingleSuccess { success: true }), StatusCode::OK),
		Err(_) => with_status(
			json(&Failure {
				success: false,
				error: "Something's Wrong",
			}),
			StatusCode::INTERNAL_SERVER_ERROR,
		),
	}
}
