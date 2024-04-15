use database::{
	api::{
		delete_person, delete_title, post_person, post_title, put_person, put_title, search_people,
		search_titles, structs::SearchParams, view_person, view_title,
	},
	models::{NamesNoId, TitleNoId},
	utils::establish_connection,
};
use std::io::{self, Write};
use warp::Filter;

#[tokio::main]
async fn main() {
	let api = warp::path("person")
		.and(
			warp::get()
				.and(
					warp::query::<SearchParams>()
						.map(|params: SearchParams| {
							search_people(&mut establish_connection(), params)
						})
						.or(warp::path::param()
							.map(|id: String| view_person(&mut establish_connection(), &id))),
				)
				.or(warp::post()
					.and(warp::body::json())
					.map(|person: NamesNoId| post_person(&mut establish_connection(), person))
					.or(warp::put()
						.and(warp::body::json())
						.and(warp::path::param())
						.map(|person: NamesNoId, id: String| {
							put_person(&mut establish_connection(), &id, person)
						})
						.or(warp::delete()
							.and(warp::path::param())
							.map(|id: String| delete_person(&mut establish_connection(), &id))))),
		)
		.or(warp::path("title").and(
			warp::get()
				.and(
					warp::query::<SearchParams>()
						.map(|params: SearchParams| {
							search_titles(&mut establish_connection(), params)
						})
						.or(warp::path::param()
							.map(|id: String| view_title(&mut establish_connection(), &id))),
				)
				.or(warp::post()
					.and(warp::body::json())
					.map(|title: TitleNoId| post_title(&mut establish_connection(), title))
					.or(warp::put()
						.and(warp::body::json())
						.and(warp::path::param())
						.map(|title: TitleNoId, id: String| {
							put_title(&mut establish_connection(), &id, title)
						}))
					.or(warp::delete()
						.and(warp::path::param())
						.map(|id: String| delete_title(&mut establish_connection(), &id)))),
		));
	let server = warp::serve(api).run(([127, 0, 0, 1], 3030));
	println!("Listening on http://localhost:3030/");
	io::stdout().flush().unwrap(); // Flush the stdout buffer
	server.await;
}
