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

#[derive(Queryable)]
pub struct PrincipalRow {
	pub tconst: i32,
	pub primarytitle: Option<String>,
	pub category: String,
	pub job: Option<String>,
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
