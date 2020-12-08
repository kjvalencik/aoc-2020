use aoc_common::*;

static PUZZLE_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\d+)-(\d+) (.): (.*)").expect("Invalid regex"));

struct Policy {
	lower: usize,
	upper: usize,
	letter: char,
}

impl Policy {
	fn part1(&self, password: &str) -> bool {
		let count = password.chars().filter(|&c| c == self.letter).count();

		self.lower <= count && count <= self.upper
	}

	fn part2(&self, password: &str) -> bool {
		let letter = self.letter.to_string();
		let a = &password[(self.lower - 1)..=(self.lower - 1)];
		let b = &password[(self.upper - 1)..=(self.upper - 1)];

		(a == letter) ^ (b == letter)
	}
}

struct PuzzleLine<'a> {
	password: &'a str,
	policy: Policy,
}

impl<'a> TryFrom<&'a str> for PuzzleLine<'a> {
	type Error = Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let caps = PUZZLE_REGEX.captures(s).ok_or("Failed to parse input")?;
		let policy = Policy {
			lower: caps.get(1).ok_or("Missing lower")?.as_str().parse()?,
			upper: caps.get(2).ok_or("Missing upper")?.as_str().parse()?,
			letter: caps
				.get(3)
				.ok_or("Missing letter")?
				.as_str()
				.chars()
				.next()
				.ok_or("Missing letter")?,
		};

		Ok(PuzzleLine {
			password: caps.get(4).ok_or("Missing password")?.as_str(),
			policy,
		})
	}
}

fn part1(input: &[PuzzleLine]) -> usize {
	input
		.iter()
		.filter(|item| item.policy.part1(item.password))
		.count()
}

fn part2(input: &[PuzzleLine]) -> usize {
	input
		.iter()
		.filter(|item| item.policy.part2(item.password))
		.count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;
	let input = input.try_from_lines()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}
