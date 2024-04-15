use crate::{
	models::{AkaAttributes, AkaTypes, Akas},
	schema::{aka_attributes, aka_types, akas},
	utils::establish_connection,
};
use rayon::prelude::*;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	thread::available_parallelism,
};

pub fn insert_akas() -> u128 {
	let f = File::open("data/title.akas.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 0;
	let mut akas = Vec::new();
	let mut aka_types = Vec::new();
	let mut aka_attributes = Vec::new();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let col: Vec<&str> = tmp.trim().split('\t').collect();
		let ordering = (&col[1]).parse().unwrap();
		akas.push(Akas {
			tconst: (&col[0][2..]).parse().unwrap(),
			ordering,
			title: if col[2] == "\\N" {
				None
			} else {
				Some(col[2].to_string())
			},
			region: if col[3] == "\\N" {
				None
			} else {
				Some(col[3].to_string())
			},
			language: if col[4] == "\\N" {
				None
			} else {
				Some(col[4].to_string())
			},
			is_original_title: if col[7] == "0" {
				Some(false)
			} else if col[7] == "1" {
				Some(true)
			} else {
				None
			},
		});
		counter += 1;
		if col[5] != "\\N" {
			aka_types.push(AkaTypes {
				tconst: (&col[0][2..]).parse().unwrap(),
				ordering,
				type_name: col[5].to_string(),
			});
			counter += 1;
		}
		if col[6] != "\\N" {
			aka_attributes.push(AkaAttributes {
				tconst: (&col[0][2..]).parse().unwrap(),
				ordering,
				attribute: col[6].to_string(),
			});
			counter += 1;
		}
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	let chunk_size = akas.len() / (available_parallelism().unwrap().get() / 2);
	akas.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(akas::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	let chunk_size = aka_types.len() / (available_parallelism().unwrap().get() / 2);
	aka_types.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(aka_types::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	let chunk_size = aka_attributes.len() / (available_parallelism().unwrap().get() / 2);
	aka_attributes.par_chunks(chunk_size).for_each_init(
		|| establish_connection(),
		|conn, chunk| {
			diesel::copy_in(aka_attributes::table)
				.from_insertable(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	counter
}
