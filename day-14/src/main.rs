use aoc_common::*;

#[derive(Debug)]
struct Operation {
	mem: u64,
	value: u64,
}

impl FromStr for Operation {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.trim().split(']');
		let mem = parts
			.next()
			.ok_or_else(|| Error::from("Expected address"))?
			.trim_start_matches("mem[")
			.parse()?;

		let value = parts
			.next()
			.ok_or_else(|| Error::from("Expected value"))?
			.trim_start_matches(" = ")
			.parse()?;

		Ok(Self { mem, value })
	}
}

#[derive(Debug)]
struct Mask {
	and: u64,
	or: u64,
}

impl Mask {
	fn apply(&self, n: u64) -> u64 {
		(self.and & n) | self.or
	}
}

impl FromStr for Mask {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim().trim_start_matches("mask = ");
		let and = s
			.chars()
			.map(|c| if c == '0' { '0' } else { '1' })
			.collect::<String>();

		let or = s
			.chars()
			.map(|c| if c == '1' { '1' } else { '0' })
			.collect::<String>();

		Ok(Self {
			and: u64::from_str_radix(&and, 2)?,
			or: u64::from_str_radix(&or, 2)?,
		})
	}
}

#[derive(Debug)]
struct Encoder {
	version1: Mask,
	version2: Vec<u64>,
}

impl Encoder {
	fn masks<'a>(&'a self) -> impl Iterator<Item = Mask> + 'a {
		(0..(2usize.pow(self.version2.len() as u32))).map(move |i| {
			let and =
				self.version2.iter().enumerate().fold(!0u64, |acc, (j, k)| {
					if i & (1 << j) == 0 {
						acc & !k
					} else {
						acc
					}
				});

			let or = self.version2.iter().enumerate().fold(
				self.version1.or,
				|acc, (j, k)| if i & (1 << j) > 0 { acc | k } else { acc },
			);

			Mask { and, or }
		})
	}
}

impl FromStr for Encoder {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim();
		let version1 = s.parse()?;
		let version2 = s
			.chars()
			.enumerate()
			.filter(|(_, c)| c == &'X')
			.map(|(i, _)| 1 << (35 - i))
			.collect();

		Ok(Self { version1, version2 })
	}
}

#[derive(Debug)]
struct Command {
	encoder: Encoder,
	ops: Vec<Operation>,
}

impl FromStr for Command {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.trim().lines();
		let encoder = parts
			.next()
			.ok_or_else(|| Error::from("Missing mask"))?
			.parse()?;

		let ops = parts.map(FromStr::from_str).collect::<Result<_, _>>()?;

		Ok(Self { encoder, ops })
	}
}

#[derive(Debug)]
struct Program {
	commands: Vec<Command>,
}

impl FromStr for Program {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let commands = s
			.trim()
			.split("mask = ")
			.skip(1)
			.map(FromStr::from_str)
			.collect::<Result<_, _>>()?;

		Ok(Self { commands })
	}
}

fn part1(input: &Program) -> u64 {
	let mut memory = HashMap::new();

	for command in &input.commands {
		for op in &command.ops {
			memory.insert(op.mem, command.encoder.version1.apply(op.value));
		}
	}

	memory.into_iter().map(|(_, v)| v).sum()
}

fn part2(input: &Program) -> u64 {
	let mut memory = HashMap::new();

	for command in &input.commands {
		for mask in command.encoder.masks() {
			for op in &command.ops {
				memory.insert(mask.apply(op.mem), op.value);
			}
		}
	}

	memory.into_iter().map(|(_, v)| v).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?.parse()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}

#[test]
fn test_mask() -> Result<(), Error> {
	let mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Mask>()?;

	assert_eq!(mask.apply(11), 73);
	assert_eq!(mask.apply(101), 101);
	assert_eq!(mask.apply(0), 64);

	Ok(())
}

#[test]
fn test_part2() -> Result<(), Error> {
	let program = r#"
		mask = 000000000000000000000000000000X1001X
		mem[42] = 100
		mask = 00000000000000000000000000000000X0XX
		mem[26] = 1
	"#
	.parse::<Program>()?;

	assert_eq!(part2(&program), 208);

	Ok(())
}
