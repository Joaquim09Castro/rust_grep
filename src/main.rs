use std::{
	env,
	process,
};

use rust_grep::structs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config: structs::Config = structs::Config::build(&args).unwrap_or_else( |err: &str| {
		eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});

	if let Err(e) = rust_grep::run(config) {
		eprintln!("Application error: {e}");
		process::exit(1);
	}
}
