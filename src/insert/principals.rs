use crate::{models::Principal, schema::principals, utils::establish_connection};
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	thread::available_parallelism,
};

pub fn insert_principals() -> u128 {
	let f = File::open("data/title.principals.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 0;
	let mut principals = Vec::new();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let col: Vec<&str> = tmp.trim().split('\t').collect();
		principals.push(Principal {
			tconst: (&col[0][2..]).parse().unwrap(),
			ordering: col[1].parse().unwrap(),
			nconst: (&col[2][2..]).parse().unwrap(),
			category: col[3].to_string(),
			job: if col[4] == "\\N" {
				None
			} else {
				Some(col[4].to_string())
			},
			characters: if col[5] == "\\N" {
				None
			} else {
				Some(col[5][2..col[5].len() - 2].to_string())
			},
		});
		counter += 1;
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	let chunk_size = principals.len() / (available_parallelism().unwrap().get() / 2);
	principals.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(principals::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	counter
}
