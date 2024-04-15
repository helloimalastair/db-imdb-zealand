use crate::schema::{
	names::dsl::{names, nconst as names_nconst, primaryname},
	principals::dsl::{
		category, characters, job, nconst as principals_nconst, principals,
		tconst as principals_tconst,
	},
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
	pub primaryname: String,
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
		FilterDsl::filter(principals, principals_tconst.eq(&id))
			.inner_join(names.on(names_nconst.eq(principals_nconst))),
		(names_nconst, primaryname, category, job, characters),
	)
	.limit(10)
	.load::<PrincipalRow>(connection)
}
