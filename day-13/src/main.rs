use aoc_common::*;

fn part1(input: &str) -> usize {
	let mut lines = input.trim().lines();
	let id = lines.next().unwrap().trim().parse::<usize>().unwrap();

	lines
		.next()
		.unwrap()
		.trim()
		.split(',')
		.filter(|&b| b != "x")
		.map(|b| b.parse::<usize>().unwrap())
		.map(|b| (b, b - (id % b)))
		.min_by(|(_, a), (_, b)| a.cmp(b))
		.map(|(id, wait)| id * wait)
		.unwrap()
}

fn part2(input: &str) -> usize {
	input
		.trim()
		.lines()
		.skip(1)
		.flat_map(|line| line.trim().split(','))
		.enumerate()
		.filter(|(_, b)| b != &"x")
		.map(|(min, b)| (min, b.parse::<usize>().unwrap()))
		.fold((1, 0), |(repeat, offset), (min, id)| {
			(0..)
				.step_by(repeat)
				.map(|n| offset + n)
				.find(|n| (n + min) % id == 0)
				.map(|n| (repeat * id, n))
				.unwrap()
		})
		.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}

#[test]
fn test_part1() {
	let input = r#"
		939
		7,13,x,x,59,x,31,19
	"#;

	assert_eq!(part1(&input), 295);
}

#[test]
fn test_part2() {
	let input = r#"
		939
		7,13,x,x,59,x,31,19
	"#;

	assert_eq!(part2(&input), 1068781);
	//panic!();
}
