pub use std::collections::{HashMap, HashSet};
pub use std::convert::TryFrom;
pub use std::str::FromStr;

pub use once_cell::sync::Lazy;
pub use regex::Regex;

pub type Error = Box<dyn std::error::Error>;

pub trait AocParse<'a> {
	fn parse_lines<F>(&self) -> Result<Vec<F>, <F as FromStr>::Err>
	where
		F: FromStr;

	fn try_from_lines<F>(
		&'a self,
	) -> Result<Vec<F>, <F as TryFrom<&'a str>>::Error>
	where
		F: TryFrom<&'a str>;
}

fn parser<'a, T, E>(
	input: &'a str,
	parse: impl Fn(&'a str) -> Result<T, E>,
) -> Result<Vec<T>, E> {
	input
		.trim()
		.split('\n')
		.map(parse)
		.collect::<Result<_, _>>()
}

impl<'a, T: AsRef<str>> AocParse<'a> for T {
	fn parse_lines<F>(&self) -> Result<Vec<F>, <F as FromStr>::Err>
	where
		F: FromStr,
	{
		parser(self.as_ref(), |s| s.parse())
	}

	fn try_from_lines<F>(
		&'a self,
	) -> Result<Vec<F>, <F as TryFrom<&'a str>>::Error>
	where
		F: TryFrom<&'a str>,
	{
		parser(self.as_ref(), TryFrom::try_from)
	}
}

pub fn read_stdin() -> Result<String, std::io::Error> {
	use std::io::Read;

	let mut buf = String::new();
	std::io::stdin().read_to_string(&mut buf)?;
	Ok(buf)
}
