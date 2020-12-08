use aoc_common::*;

#[derive(Debug)]
enum Operation {
	Accumulator,
	Jump,
	Noop,
}

impl FromStr for Operation {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"acc" => Ok(Operation::Accumulator),
			"jmp" => Ok(Operation::Jump),
			"nop" => Ok(Operation::Noop),
			_ => Err(Error::from("Invalid operation")),
		}
	}
}

#[derive(Debug)]
struct Instruction {
	operation: Operation,
	value: i64,
}

impl FromStr for Instruction {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		let operation = parts
			.next()
			.ok_or_else(|| Error::from("Missing operation"))?
			.parse()?;

		let value = parts
			.next()
			.ok_or_else(|| Error::from("Missing value"))?
			.parse()?;

		Ok(Instruction { operation, value })
	}
}

fn execute(instructions: &[Instruction]) -> Result<i64, i64> {
	let mut acc = 0;
	let mut executed = vec![false; instructions.len()];
	let mut i = 0;

	while i < instructions.len() {
		if executed[i] {
			return Err(acc);
		}

		executed[i] = true;

		let instruction = &instructions[i];

		match instruction.operation {
			Operation::Accumulator => {
				i += 1;
				acc += instruction.value;
			}
			Operation::Jump => {
				i = (instruction.value + (i as i64)) as usize;
			}
			Operation::Noop => {
				i += 1;
			}
		}
	}

	Ok(acc)
}

fn part1(instructions: &[Instruction]) -> i64 {
	match execute(instructions) {
		Ok(_) => panic!("Expected to have an infinite loop"),
		Err(n) => n,
	}
}

fn part2(instructions: &mut [Instruction]) -> i64 {
	for i in 0..instructions.len() {
		let instruction = &mut instructions[i];
		let operation = match instruction.operation {
			Operation::Jump => {
				std::mem::replace(&mut instruction.operation, Operation::Noop)
			}
			Operation::Noop => {
				std::mem::replace(&mut instruction.operation, Operation::Jump)
			}
			_ => continue,
		};

		if let Ok(n) = execute(instructions) {
			return n;
		}

		instructions[i].operation = operation;
	}

	unreachable!("No solution to Part 2")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut input = read_stdin()?.parse_lines()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&mut input));

	Ok(())
}
