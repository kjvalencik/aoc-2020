use aoc_common::*;

fn part1(input: &[u64]) -> u64 {
	let mut prev = 0;
	let mut diff1 = 0;
	let mut diff3 = 0;

	for n in input.iter().skip(1) {
		match n - prev {
			1 => diff1 += 1,
			3 => diff3 += 1,
			_ => {}
		}

		prev = *n;
	}

	diff1 * diff3
}

fn part2(input: &[u64]) -> u64 {
	let mut prev3 = 1;
	let mut prev2 = 1;
	let mut prev1 = if input[2] <= 3 { 2 } else { 1 };

	for i in 3..input.len() {
		let mut next = prev1;

		if input[i] - input[i - 2] <= 3 {
			next += prev2;
		}

		if input[i] - input[i - 3] <= 3 {
			next += prev3;
		}

		prev3 = prev2;
		prev2 = prev1;
		prev1 = next;
	}

	prev1
}

fn arrange_adapters(mut input: Vec<u64>) -> Vec<u64> {
	input.push(0);
	input.sort();
	input.push(input[input.len() - 1] + 3);
	input
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?.parse_lines().map(arrange_adapters)?;

	println!("Part 1: {}", part1(&input));
	println!("Part 1: {}", part2(&input));

	Ok(())
}

#[test]
fn test_small() {
	let input = arrange_adapters(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);

	assert_eq!(part1(&input), 7 * 5);
	assert_eq!(part2(&input), 8);
}

#[test]
fn test_large() {
	let input = arrange_adapters(vec![
		28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11,
		1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
	]);

	assert_eq!(part1(&input), 22 * 10);
	assert_eq!(part2(&input), 19208);
}
