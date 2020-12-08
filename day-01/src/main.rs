use aoc_common::*;

fn part1(input: &[i64]) -> i64 {
	for (i, m) in input.iter().enumerate() {
		for (j, n) in input.iter().enumerate() {
			if i == j {
				continue;
			}

			if m + n == 2020 {
				return m * n;
			}
		}
	}

	unreachable!("No solution!");
}

fn part2(input: &[i64]) -> i64 {
	for (i, m) in input.iter().enumerate() {
		for (j, n) in input.iter().enumerate() {
			if i == j || m + n > 2020 {
				continue;
			}

			for (k, l) in input.iter().enumerate() {
				if j == k {
					continue;
				}

				if m + n + l == 2020 {
					return m * n * l;
				}
			}
		}
	}

	unreachable!("No solution!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?.parse_lines()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}
