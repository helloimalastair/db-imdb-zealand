use crate::{models::Names, schema::names, utils::establish_connection};
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	thread::available_parallelism,
};

pub fn insert_name() -> u128 {
	let f = File::open("data/name.basics.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 0;
	let mut names = Vec::new();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let col: Vec<&str> = tmp.trim().split('\t').collect();
		names.push(Names {
			nconst: (&col[0][2..]).parse().unwrap(),
			primaryname: col[1].to_string(),
			birthyear: if col[2] == "\\N" {
				None
			} else {
				Some(col[2].parse().unwrap())
			},
			deathyear: if col[3] == "\\N" {
				None
			} else {
				Some(col[3].parse().unwrap())
			},
		});
		counter += 1;
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	let chunk_size = names.len() / (available_parallelism().unwrap().get() / 2);
	names.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(names::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	counter
}
