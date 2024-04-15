use crate::schema::*;
use diesel::{pg::Pg, AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crew)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Crew {
	pub tconst: i32,
	pub nconst: i32,
	pub isdirector: bool,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = genres)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Genres {
	pub tconst: i32,
	pub genre: String,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = names)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Names {
	pub nconst: i32,
	pub primaryname: String,
	pub birthyear: Option<i32>,
	pub deathyear: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = names)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct NamesNoId {
	pub primaryname: String,
	pub birthyear: Option<i32>,
	pub deathyear: Option<i32>,
}

#[derive(Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = titles)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Titles {
	pub tconst: i32,
	pub titletype: Option<String>,
	pub primarytitle: Option<String>,
	pub originaltitle: Option<String>,
	pub isadult: Option<bool>,
	pub startyear: Option<i32>,
	pub endyear: Option<i32>,
	pub runtimeminutes: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = titles)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct TitleNoId {
	pub titletype: Option<String>,
	pub primarytitle: Option<String>,
	pub originaltitle: Option<String>,
	pub isadult: Option<bool>,
	pub startyear: Option<i32>,
	pub endyear: Option<i32>,
	pub runtimeminutes: Option<i32>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = principals)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Principal {
	pub tconst: i32,
	pub ordering: i32,
	pub nconst: i32,
	pub category: String,
	pub job: Option<String>,
	pub characters: Option<String>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = akas)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Akas {
	pub tconst: i32,
	pub ordering: i32,
	pub title: Option<String>,
	pub region: Option<String>,
	pub language: Option<String>,
	pub is_original_title: Option<bool>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = aka_types)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct AkaTypes {
	pub tconst: i32,
	pub ordering: i32,
	pub type_name: String,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = aka_attributes)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct AkaAttributes {
	pub tconst: i32,
	pub ordering: i32,
	pub attribute: String,
}
