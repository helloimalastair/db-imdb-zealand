mod crew;
mod principals;

use crate::{
	api::{structs::Failure, utils::generate_sqid},
	models::Names,
	schema::names::dsl::{names, nconst},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, SelectDsl},
	ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use serde::Serialize;
use warp::{
	http::StatusCode,
	reply::{json, with_status, Json, WithStatus},
};


#[derive(Serialize)]
pub struct CrewWithSqid {
	pub tconst: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primarytitle: Option<String>,
	pub isdirector: bool,
}

#[derive(Serialize)]
pub struct PrincipalWithSqid {
	pub tconst: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primaryname: Option<String>,
	pub category: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub job: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub characters: Option<String>,
}

#[derive(Serialize)]
pub struct ResultObject {
	pub success: bool,
	pub nconst: i32,
	pub primaryname: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthyear: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deathyear: Option<i32>,
	pub crew: Vec<CrewWithSqid>,
	pub principals: Vec<PrincipalWithSqid>,
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
	let crew: Vec<CrewWithSqid> = match crew::get_crew(&id, connection) {
		Ok(a) => a.iter().map(|a| {
			CrewWithSqid {
				tconst: sqid.encode(&[a.tconst as u64]).unwrap(),
				primarytitle: a.primarytitle.clone(),
				isdirector: a.isdirector,
			}
		}).collect(),
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
	let principals: Vec<PrincipalWithSqid> = match principals::get_principals(&id, connection) {
		Ok(a) => a.iter().map(|a| {
			PrincipalWithSqid {
				tconst: sqid.encode(&[a.tconst as u64]).unwrap(),
				primaryname: a.primarytitle.clone(),
				category: a.category.clone(),
				job: a.job.clone(),
				characters: a.characters.clone(),
			}
		}).collect(),
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
