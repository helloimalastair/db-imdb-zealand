use crate::{
	models::{Genres, Titles},
	schema::{genres, titles},
	utils::establish_connection,
};
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	thread::available_parallelism,
};

pub fn insert_title() -> u128 {
	let f = File::open("data/title.basics.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 0;
	let mut titles = Vec::new();
	let mut genres = Vec::new();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let col = tmp.trim().split('\t').collect::<Vec<&str>>();
		titles.push(Titles {
			tconst: (&col[0][2..]).parse().unwrap(),
			titletype: if col[1] == "\\N" {
				None
			} else {
				Some(col[1].to_string())
			},
			primarytitle: if col[2] == "\\N" {
				None
			} else {
				Some(col[2].to_string())
			},
			originaltitle: if col[3] == "\\N" {
				None
			} else {
				Some(col[3].to_string())
			},
			isadult: if col[4] == "\\N" {
				None
			} else if col[4] == "0" {
				Some(false)
			} else {
				Some(true)
			},
			startyear: if col[5] == "\\N" {
				None
			} else {
				Some(col[5].parse().unwrap())
			},
			endyear: if col[6] == "\\N" {
				None
			} else {
				Some(col[6].parse().unwrap())
			},
			runtimeminutes: if col[7] == "\\N" {
				None
			} else {
				Some(col[7].parse().unwrap())
			},
		});
		counter += 1;
		if col[8] != "\\N" {
			let title_genres = col[8].split(',').collect::<Vec<&str>>();
			for genre in title_genres {
				genres.push(Genres {
					tconst: (&col[0][2..]).parse().unwrap(),
					genre: genre.to_string(),
				});
				counter += 1;
			}
		}
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	let chunk_size = titles.len() / (available_parallelism().unwrap().get() / 2);
	titles.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(titles::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	let chunk_size = genres.len() / (available_parallelism().unwrap().get() / 2);
	genres.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(genres::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	counter
}
