use aoc_common::*;

fn binary(input: &str) -> usize {
	let size = 2usize.pow(input.len() as u32);

	input
		.chars()
		.fold((0, size), |(x, y), c| match c {
			'F' | 'L' => (x, x + (y - x) / 2),
			_ => (x + (y - x) / 2, y),
		})
		.0
}

fn seat_id((x, y): &(usize, usize)) -> usize {
	x * 8 + y
}

fn part1(input: &[(usize, usize)]) -> Result<usize, Error> {
	input
		.iter()
		.map(seat_id)
		.max()
		.ok_or_else(|| "Expected at least one line of input".into())
}

fn part2(input: &[(usize, usize)]) -> Result<usize, Error> {
	let seats = input.iter().map(seat_id).collect::<HashSet<usize>>();

	for x in 1..128 {
		for y in 0..8 {
			let id = seat_id(&(x, y));

			if !seats.contains(&id)
				&& seats.contains(&(id + 1))
				&& seats.contains(&(id - 1))
			{
				return Ok(id);
			}
		}
	}

	Err("Expected to have a seat".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;
	let input = input
		.trim()
		.lines()
		.map(|line| (binary(&line[0..7]), binary(&line[7..])))
		.collect::<Vec<_>>();

	println!("Part 1: {}", part1(&input)?);
	println!("Part 2: {}", part2(&input)?);

	Ok(())
}
