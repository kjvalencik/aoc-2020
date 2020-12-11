use aoc_common::*;

use std::collections::HashMap;

enum Height {
	Inches(u16),
	Centimeters(u16),
}

impl FromStr for Height {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let height = if s.ends_with("in") {
			let height = s.trim_end_matches("in").parse()?;

			if !(59..=76).contains(&height) {
				return Err("Invalid Height".into());
			}

			Height::Inches(height)
		} else if s.ends_with("cm") {
			let height = s.trim_end_matches("cm").parse()?;

			if !(150..=193).contains(&height) {
				return Err("Invalid Height".into());
			}

			Height::Centimeters(height)
		} else {
			return Err("Invalid Height".into());
		};

		Ok(height)
	}
}

enum EyeColor {
	Amber,
	Blue,
	Brown,
	Grey,
	Green,
	Hazel,
	Other,
}

impl FromStr for EyeColor {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"amb" => Ok(EyeColor::Amber),
			"blu" => Ok(EyeColor::Blue),
			"brn" => Ok(EyeColor::Brown),
			"gry" => Ok(EyeColor::Grey),
			"grn" => Ok(EyeColor::Green),
			"hzl" => Ok(EyeColor::Hazel),
			"oth" => Ok(EyeColor::Other),
			_ => Err("Invalid eye color".into()),
		}
	}
}

struct PassportId(String);

impl FromStr for PassportId {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() != 9 {
			return Err("Incorrect passport id length".into());
		}

		Ok(PassportId(s.to_string()))
	}
}

#[allow(dead_code)]
struct Passport {
	birth_year: u16,
	issue_year: u16,
	expiration_year: u16,
	height: Height,
	hair_color: Color,
	eye_color: EyeColor,
	passport_id: PassportId,
	country_id: Option<String>,
}

struct Color(String);

impl FromStr for Color {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() != 7 {
			return Err("Incorrect color length".into());
		}

		if &s[0..1] != "#" {
			return Err("Invalid color".into());
		}

		let is_valid_hex = s
			.chars()
			.skip(1)
			.all(|c| ('a'..='f').contains(&c) || ('0'..='9').contains(&c));

		if !is_valid_hex {
			return Err("Invalid color".into());
		}

		Ok(Color(s.to_string()))
	}
}

impl TryFrom<&HashMap<&str, &str>> for Passport {
	type Error = Error;

	fn try_from(v: &HashMap<&str, &str>) -> Result<Self, Self::Error> {
		let birth_year = v.get("byr").ok_or("Missing birth year")?.parse()?;

		if !(1920..=2002).contains(&birth_year) {
			return Err("Invalid birth year".into());
		}

		let issue_year = v.get("iyr").ok_or("Missing issue year")?.parse()?;

		if !(2010..=2020).contains(&issue_year) {
			return Err("Invalid issue year".into());
		}

		let expiration_year =
			v.get("eyr").ok_or("Missing expiration year")?.parse()?;

		if !(2020..=2030).contains(&expiration_year) {
			return Err("Invalid expiration year".into());
		}

		let height = v.get("hgt").ok_or("Missing height")?.parse()?;

		let hair_color = v.get("hcl").ok_or("Missing hair color")?.parse()?;

		let eye_color = v.get("ecl").ok_or("Missing eye color")?.parse()?;

		let passport_id = v.get("pid").ok_or("Missing Passport ID")?.parse()?;

		let country_id = v.get("cid").map(|v| v.to_string());

		Ok(Passport {
			birth_year,
			issue_year,
			expiration_year,
			height,
			hair_color,
			eye_color,
			passport_id,
			country_id,
		})
	}
}

fn parse_passport(input: &str) -> Result<HashMap<&str, &str>, Error> {
	let pattern = ['\n', ' '];

	input
		.split(&pattern[..])
		.map(|item| {
			let mid = item.find(':').ok_or("Missing `:`")?;
			let (left, right) = item.split_at(mid);

			Ok((left, &right[1..]))
		})
		.collect()
}

fn part1(input: &[HashMap<&str, &str>]) -> usize {
	input
		.iter()
		.filter(|map| {
			map.len() >= 8 || (map.len() == 7 && !map.contains_key("cid"))
		})
		.count()
}

fn part2(input: &[HashMap<&str, &str>]) -> usize {
	input
		.iter()
		.filter(|&map| Passport::try_from(map).is_ok())
		.count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;
	let input = input
		.trim()
		.split("\n\n")
		.map(parse_passport)
		.collect::<Result<Vec<_>, _>>()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}
