use crate::schema::{
	crew::dsl::{crew, isdirector, nconst as crew_nconst, tconst as crew_tconst},
	titles::dsl::{primarytitle, tconst as titles_tconst, titles},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, SelectDsl},
	ExpressionMethods, JoinOnDsl, QueryDsl, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct CrewRow {
	pub tconst: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primarytitle: Option<String>,
	pub isdirector: bool,
}

pub fn get_crew(
	id: &i32,
	connection: &mut PgConnection,
) -> Result<Vec<CrewRow>, diesel::result::Error> {
	SelectDsl::select(
		FilterDsl::filter(crew, crew_nconst.eq(&id))
			.inner_join(titles.on(titles_tconst.eq(crew_tconst))),
		(titles_tconst, primarytitle, isdirector),
	)
	.limit(10)
	.load::<CrewRow>(connection)
}
