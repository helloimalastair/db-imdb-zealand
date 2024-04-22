use crate::schema::{
	crew::dsl::{crew, isdirector, nconst as crew_nconst, tconst as crew_tconst},
	names::dsl::{names, nconst as names_nconst, primaryname},
};
use diesel::{
	pg::PgConnection,
	query_dsl::methods::{FilterDsl, SelectDsl},
	ExpressionMethods, JoinOnDsl, QueryDsl, Queryable, RunQueryDsl,
};
#[derive(Queryable)]
pub struct CrewRow {
	pub tconst: i32,
	pub primaryname: String,
	pub isdirector: bool,
}

pub fn get_crew(
	id: &i32,
	connection: &mut PgConnection,
) -> Result<Vec<CrewRow>, diesel::result::Error> {
	SelectDsl::select(
		FilterDsl::filter(crew, crew_tconst.eq(&id))
			.inner_join(names.on(names_nconst.eq(crew_nconst))),
		(names_nconst, primaryname, isdirector),
	)
	.limit(10)
	.load::<CrewRow>(connection)
}
