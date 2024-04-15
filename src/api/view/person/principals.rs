use crate::schema::{
	principals::dsl::{
		category, characters, job, nconst as principals_nconst, principals,
		tconst as principals_tconst,
	},
	titles::dsl::{primarytitle, tconst as titles_tconst, titles},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, SelectDsl},
	ExpressionMethods, JoinOnDsl, QueryDsl, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct PrincipalRow {
	pub tconst: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primarytitle: Option<String>,
	pub category: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub job: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub characters: Option<String>,
}

pub fn get_principals(
	id: &i32,
	connection: &mut PgConnection,
) -> Result<Vec<PrincipalRow>, diesel::result::Error> {
	SelectDsl::select(
		FilterDsl::filter(principals, principals_nconst.eq(&id))
			.inner_join(titles.on(titles_tconst.eq(principals_tconst))),
		(titles_tconst, primarytitle, category, job, characters),
	)
	.limit(10)
	.load::<PrincipalRow>(connection)
}
