use aoc_common::*;

fn parse_color(input: &str) -> Result<(usize, &str), Error> {
	let num = input
		.split(" ")
		.next()
		.ok_or_else(|| Error::from("Missing number"))?;

	let color = &input[num.len() + 1..];
	let num = num.parse()?;

	Ok((num, color))
}

fn parse_line(input: &str) -> Result<(&str, Vec<(usize, &str)>), Error> {
	let mut parts = input.trim().split(" bags contain ");
	let color = parts.next().ok_or_else(|| Error::from("Missing color"))?;
	let contains = parts
		.next()
		.ok_or_else(|| Error::from("Missing rule"))?
		.trim_end_matches(" bags.")
		.trim_end_matches(" bag.")
		.split(", ")
		.map(|v| v.trim_end_matches(" bag").trim_end_matches(" bags"))
		.filter(|&bag| bag != "no other")
		.map(parse_color)
		.collect::<Result<_, _>>()?;

	Ok((color, contains))
}

#[derive(Debug)]
struct Puzzle<'a> {
	rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> TryFrom<&'a str> for Puzzle<'a> {
	type Error = Error;

	fn try_from(input: &'a str) -> Result<Self, Self::Error> {
		let rules = input
			.trim()
			.lines()
			.map(parse_line)
			.collect::<Result<_, _>>()?;

		Ok(Puzzle { rules })
	}
}

fn contains_color<'a>(
	reverse: &HashMap<&'a str, HashSet<&'a str>>,
	acc: &mut HashSet<&'a str>,
	color: &str,
) {
	if let Some(colors) = reverse.get(color) {
		for color in colors {
			if acc.insert(color) {
				contains_color(reverse, acc, color);
			}
		}
	}
}

fn part1(input: &Puzzle) -> usize {
	let reverse = input.rules.iter().fold(HashMap::new(), |acc, rule| {
		rule.1.iter().fold(acc, |mut acc, (_, color)| {
			acc.entry(*color)
				.or_insert_with(|| HashSet::new())
				.insert(*rule.0);

			acc
		})
	});

	let mut contains = HashSet::new();

	contains_color(&reverse, &mut contains, "shiny gold");

	contains.len()
}

fn count_bags(input: &Puzzle, color: &str) -> Result<usize, Error> {
	input
		.rules
		.get(color)
		.ok_or_else(|| Error::from("Missing color"))?
		.iter()
		.map(|(n, color)| Ok(n + n * count_bags(input, color)?))
		.sum()
}

fn part2(input: &Puzzle) -> Result<usize, Error> {
	count_bags(input, "shiny gold")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;
	let input = Puzzle::try_from(input.as_str())?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input)?);

	Ok(())
}
