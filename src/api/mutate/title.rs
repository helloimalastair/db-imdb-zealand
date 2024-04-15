use crate::{
	api::{
		structs::{Failure, SingleSuccess},
		utils::generate_sqid,
	},
	models::{TitleNoId, Titles},
	schema::titles::dsl::{tconst, titles},
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
	tconst: String,
}

pub fn post_title(connection: &mut PgConnection, title: TitleNoId) -> WithStatus<Json> {
	let sqid = generate_sqid();
	match insert_into(titles)
		.values(&title)
		.get_result::<Titles>(connection)
	{
		Ok(a) => match sqid.encode(&[a.tconst as u64]) {
			Ok(b) => with_status(
				json(&IDSuccess {
					success: true,
					tconst: b,
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

pub fn put_title(connection: &mut PgConnection, id: &str, title: TitleNoId) -> WithStatus<Json> {
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
	match update(titles)
		.set(&title)
		.filter(tconst.eq(id))
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

pub fn delete_title(connection: &mut PgConnection, id: &str) -> WithStatus<Json> {
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
	match delete(titles).filter(tconst.eq(id)).execute(connection) {
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
