mod crew;
mod principals;

use crate::{
	api::{structs::Failure, utils::generate_sqid},
	models::Titles,
	schema::titles::dsl::{tconst, titles},
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
	pub tconst: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub titletype: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primarytitle: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub originaltitle: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub isadult: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub startyear: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub endyear: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub runtimeminutes: Option<i32>,
	pub crew: Vec<CrewRow>,
	pub principals: Vec<PrincipalRow>,
}

pub fn view_title(connection: &mut PgConnection, id: &str) -> WithStatus<Json> {
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
	let res = match SelectDsl::select(
		FilterDsl::filter(titles, tconst.eq(id)),
		Titles::as_select(),
	)
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
	let mut res = ResultObject {
		success: true,
		tconst: id,
		titletype: match &res.titletype {
			Some(a) => Some(a.to_string()),
			None => None,
		},
		primarytitle: match &res.primarytitle {
			Some(a) => Some(a.to_string()),
			None => None,
		},
		originaltitle: match &res.originaltitle {
			Some(a) => Some(a.to_string()),
			None => None,
		},
		isadult: res.isadult,
		startyear: res.startyear,
		endyear: res.endyear,
		runtimeminutes: res.runtimeminutes,
		crew,
		principals,
	};
	if res.originaltitle.eq(&res.primarytitle) {
		res.originaltitle = None;
	}
	return with_status(json(&res), StatusCode::OK);
}
