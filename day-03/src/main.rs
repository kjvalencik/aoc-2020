use aoc_common::*;

#[derive(Debug, PartialEq)]
enum Square {
	Open,
	Tree,
}

impl TryFrom<char> for Square {
	type Error = Error;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'.' => Ok(Square::Open),
			'#' => Ok(Square::Tree),
			_ => Err("Invalid map tile".into()),
		}
	}
}

#[derive(Debug)]
struct Map(Vec<Vec<Square>>);

impl Map {
	fn slope(&self, slope: (usize, usize)) -> usize {
		self.0
			.iter()
			.step_by(slope.1)
			.enumerate()
			.filter(|(y, line)| line[y * slope.0 % line.len()] == Square::Tree)
			.count()
	}
}

impl FromStr for Map {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let map = s
			.trim()
			.lines()
			.map(|line| {
				line.chars()
					.map(|c| TryFrom::try_from(c))
					.collect::<Result<Vec<_>, _>>()
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Map(map))
	}
}

fn part1(map: &Map) -> usize {
	map.slope((3, 1))
}

fn part2(map: &Map) -> usize {
	let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

	slopes
		.into_iter()
		.map(|slope| map.slope(slope))
		.fold(1, |acc, x| acc * x)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let map = read_stdin()?.parse::<Map>()?;

	println!("Part 1: {}", part1(&map));
	println!("Part 1: {}", part2(&map));

	Ok(())
}
