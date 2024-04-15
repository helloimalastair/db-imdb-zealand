use database::{insert::*, utils::format};
use spinners::{Spinner, Spinners::SimpleDotsScrolling};
use std::{io::Write, thread::spawn};

fn main() {
	let mut spinner = Spinner::new(
		SimpleDotsScrolling,
		"Inserting data into the database".to_owned(),
	);
	std::io::stdout().flush().unwrap();
	let start = std::time::Instant::now();
	let name_handle = spawn(insert_name);
	let crew_handle = spawn(insert_crew);
	let title_handle = spawn(insert_title);
	let principals_handle = spawn(insert_principals);
	let akas_handle = spawn(insert_akas);
	let mut rows: u128 = 0;
	rows += name_handle.join().unwrap();
	rows += crew_handle.join().unwrap();
	rows += title_handle.join().unwrap();
	rows += principals_handle.join().unwrap();
	rows += akas_handle.join().unwrap();
	spinner.stop_with_message(format(rows, start));
}
