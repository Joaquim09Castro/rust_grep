use std::{
    fs,
    error::Error,
};

pub struct Config {
	pub query: String,
	pub file_path: String,
}

impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}

		let  query: String = args[1].clone();
		let file_path: String = args[2].clone();

		Ok(Config {query, file_path})
	}
}

pub fn run(config: Config) -> Result< (), Box<dyn Error>> {
	let content: String = fs::read_to_string(config.file_path)?;

	println!("With text: \n{}", content);

	Ok( ())
}