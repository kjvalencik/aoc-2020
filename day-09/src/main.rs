use aoc_common::*;

fn part1(input: &[u64]) -> u64 {
	let preamble = 25;

	input
		.iter()
		.enumerate()
		.skip(preamble)
		.map(|(i, n)| (&input[(i - preamble)..i], n))
		.find(|(nums, &n)| {
			for x in nums.iter() {
				for y in nums.iter() {
					if x != y && x + y == n {
						return false;
					}
				}
			}

			true
		})
		.map(|(_, n)| *n)
		.expect("No solution to Part 1")
}

fn part2(n: u64, input: &[u64]) -> u64 {
	for i in 0..(input.len() - 1) {
		let mut sum = input[i];

		for (j, x) in input.iter().enumerate().skip(i + 1) {
			sum += x;

			if sum == n {
				let range = &input[i..=j];
				let min = range.iter().min().unwrap();
				let max = range.iter().max().unwrap();

				return min + max;
			}

			if sum > n {
				break;
			}
		}
	}

	panic!("No solution to Part 2")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?.parse_lines()?;
	let part1 = part1(&input);

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2(part1, &input));

	Ok(())
}
