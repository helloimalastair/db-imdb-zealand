use crate::{models::Crew, schema::crew, utils::establish_connection};
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	thread::available_parallelism,
};

pub fn insert_crew() -> u128 {
	let f = File::open("data/title.crew.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 0;
	let mut crew = Vec::new();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let col = tmp.trim().split('\t').collect::<Vec<&str>>();
		if col[1] != "\\N" {
			let directors = col[1].split(',').collect::<Vec<&str>>();
			for director in directors {
				crew.push(Crew {
					tconst: (&col[0][2..]).parse().unwrap(),
					nconst: (&director[2..]).parse().unwrap(),
					isdirector: true,
				});
				counter += 1;
			}
		}
		if col[2] != "\\N" {
			let writers = col[2].split(',').collect::<Vec<&str>>();
			for writer in writers {
				crew.push(Crew {
					tconst: (&col[0][2..]).parse().unwrap(),
					nconst: (&writer[2..]).parse().unwrap(),
					isdirector: false,
				});
				counter += 1;
			}
		}
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	let chunk_size = crew.len() / (available_parallelism().unwrap().get() / 2);
	crew.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(crew::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	counter
}
