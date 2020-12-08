use aoc_common::*;

fn parse_answers(input: &str) -> [bool; 26] {
	input.trim().chars().fold([false; 26], |mut acc, c| {
		acc[c as usize - 'a' as usize] = true;
		acc
	})
}

fn parse_group(input: &str) -> Vec<[bool; 26]> {
	input.trim().lines().map(parse_answers).collect()
}

fn count_groups(
	input: &[Vec<[bool; 26]>],
	initial: bool,
	op: impl Fn(bool, bool) -> bool,
) -> usize {
	let combine_group = |group: &Vec<[bool; 26]>| {
		group.iter().fold([initial; 26], |acc, person| {
			person.iter().enumerate().fold(acc, |mut acc, (i, &a)| {
				acc[i] = op(acc[i], a);
				acc
			})
		})
	};

	input
		.iter()
		.map(combine_group)
		.map(|group| group.iter().filter(|&&x| x).count())
		.sum()
}

fn part1(input: &[Vec<[bool; 26]>]) -> usize {
	count_groups(input, false, |x, y| x || y)
}

fn part2(input: &[Vec<[bool; 26]>]) -> usize {
	count_groups(input, true, |x, y| x && y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;
	let input = input
		.trim()
		.split("\n\n")
		.map(parse_group)
		.collect::<Vec<_>>();

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}
