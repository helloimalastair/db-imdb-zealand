use crate::{
	api::{
		structs::{Failure, SearchParams},
		utils::generate_sqid,
	},
	models::Names,
	schema::names::dsl::{names, primaryname},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
	RunQueryDsl, SelectableHelper, TextExpressionMethods,
};
use serde::Serialize;
use warp::{
	http::StatusCode,
	reply::{json, with_status, Json, WithStatus},
};

#[derive(Serialize)]
struct ResultItem {
	id: String,
	name: String,
}

#[derive(Serialize)]
struct ResultObject {
	success: bool,
	results: Vec<ResultItem>,
}

pub fn search_people(connection: &mut PgConnection, params: SearchParams) -> WithStatus<Json> {
	let sqid = generate_sqid();
	let query = format!("%{}%", params.query);
	let res = match names
		.filter(primaryname.like(&query))
		.limit(5)
		.select(Names::as_select())
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
	let mut res: Vec<ResultItem> = res
		.into_iter()
		.map(|a| ResultItem {
			id: sqid.encode(&[a.nconst.try_into().unwrap()]).unwrap(),
			name: a.primaryname.to_string(),
		})
		.collect();
	res.sort_by(|a, b| a.name.cmp(&b.name));
	let res = ResultObject {
		success: true,
		results: res,
	};
	return with_status(json(&res), StatusCode::OK);
}
