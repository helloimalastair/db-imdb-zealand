use std::time::Instant;

use diesel::{pg::PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
	PgConnection::establish(&"postgres://postgres:password@localhost/imdb")
		.unwrap_or_else(|_| panic!("Error connecting to Postgres database."))
}

fn fmt_num(val: &u128) -> String {
	let mut val = val.to_string();
	let mut rows = String::new();
	while val.len() > 3 {
		rows = format!(",{}", &val[val.len() - 3..]) + &rows;
		val = val[..val.len() - 3].to_string();
	}
	val + &rows
}

// Add commas every three digits
pub fn format(rows: u128, start: Instant) -> String {
	let elapsed = start.elapsed();
	let rate = rows / elapsed.as_secs() as u128;
	let mut nanos = elapsed.as_nanos();
	let minutes = nanos / 60_000_000_000;
	nanos %= 60_000_000_000;
	let seconds = nanos / 1_000_000_000;
	nanos %= 1_000_000_000;
	let millis = nanos / 1_000_000;
	nanos %= 1_000_000;
	let micros = nanos / 1_000;
	nanos %= 1_000;
	format!(
		"Inserted {} rows in {} min, {} sec, {} ms, {} μs, {} ns. Average speed: {} rows/s",
		fmt_num(&rows),
		minutes,
		seconds,
		millis,
		micros,
		nanos,
		fmt_num(&rate)
	)
}
