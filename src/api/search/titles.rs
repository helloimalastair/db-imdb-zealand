use crate::{
	api::{
		structs::{Failure, SearchParams},
		utils::generate_sqid,
	},
	models::{Akas, Titles},
	schema::{
		akas::dsl::{akas, title},
		titles::{
			dsl::{primarytitle, titles},
			originaltitle,
		},
	},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
	BoolExpressionMethods, RunQueryDsl, SelectableHelper, TextExpressionMethods,
};
use serde::Serialize;
use warp::{
	http::StatusCode,
	reply::{json, with_status, Json, WithStatus},
};

#[derive(Serialize)]
struct ResultItem {
	id: String,
	title: String,
}

#[derive(Serialize)]
struct ResultObject {
	success: bool,
	results: Vec<ResultItem>,
}

pub fn search_titles(connection: &mut PgConnection, params: SearchParams) -> WithStatus<Json> {
	println!("Searching for titles");
	let sqid = generate_sqid();
	let query = format!("%{}%", params.query);
	let res = titles
		.filter(primarytitle.like(&query).or(originaltitle.like(&query)))
		.limit(5)
		.select(Titles::as_select())
		.load(connection);
	if res.is_err() {
		return with_status(
			json(&Failure {
				success: false,
				error: "Something's Wrong",
			}),
			StatusCode::INTERNAL_SERVER_ERROR,
		);
	}
	let mut res: Vec<ResultItem> = res
		.unwrap()
		.iter()
		.map(|a| ResultItem {
			id: sqid.encode(&[a.tconst.try_into().unwrap()]).unwrap(),
			title: if a.primarytitle.is_some() {
				a.primarytitle.as_ref().unwrap().to_string()
			} else {
				a.originaltitle.as_ref().unwrap().to_string()
			},
		})
		.collect();
	if res.len() == 5 {
		res.sort_by(|a, b| a.title.cmp(&b.title));
		let res = ResultObject {
			success: true,
			results: res,
		};
		return with_status(json(&res), StatusCode::OK);
	}
	let minires = akas
		.filter(title.like(&query))
		.limit(5 - res.len() as i64)
		.select(Akas::as_select())
		.load(connection);
	if minires.is_err() {
		return with_status(
			json(&Failure {
				success: false,
				error: "Something's Wrong",
			}),
			StatusCode::INTERNAL_SERVER_ERROR,
		);
	}
	for a in minires.unwrap().into_iter() {
		res.push(ResultItem {
			id: sqid.encode(&[a.tconst.try_into().unwrap()]).unwrap(),
			title: a.title.as_ref().unwrap().to_string(),
		});
	}
	res.sort_by(|a, b| a.title.cmp(&b.title));
	let res = ResultObject {
		success: true,
		results: res,
	};
	return with_status(json(&res), StatusCode::OK);
}
