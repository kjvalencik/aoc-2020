use aoc_common::*;

#[derive(Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
	fn new(input: &str) -> Self {
		let grid = input
			.trim()
			.lines()
			.map(|line| line.trim().chars().collect())
			.collect();

		Self(grid)
	}
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for line in self.0.iter() {
			writeln!(f, "{}", line.iter().collect::<String>())?;
		}

		Ok(())
	}
}

fn part1(input: &Grid) -> usize {
	let mut prev = Grid::new("");
	let mut next = input.clone();
	let height = input.0.len();
	let width = input.0[0].len();

	let is_occupied = |grid: &[Vec<char>], x, y| {
		if x < 0 || y < 0 {
			return 0;
		}

		let x = x as usize;
		let y = y as usize;
		let seat = grid.get(y).and_then(|row: &Vec<char>| row.get(x));

		if seat == Some(&'#') {
			1
		} else {
			0
		}
	};

	let count_occupied = |grid: &[Vec<char>], x, y| -> usize {
		let x = x as i64;
		let y = y as i64;

		is_occupied(grid, x - 1, y)
			+ is_occupied(grid, x + 1, y)
			+ is_occupied(grid, x, y - 1)
			+ is_occupied(grid, x, y + 1)
			+ is_occupied(grid, x - 1, y - 1)
			+ is_occupied(grid, x + 1, y - 1)
			+ is_occupied(grid, x - 1, y + 1)
			+ is_occupied(grid, x + 1, y + 1)
	};

	while prev.0 != next.0 {
		prev = next;
		next = prev.clone();

		for y in 0..height {
			for x in 0..width {
				let count = count_occupied(&prev.0, x, y);

				match prev.0[y][x] {
					'L' => {
						if count == 0 {
							next.0[y][x] = '#';
						}
					}
					'#' => {
						if count >= 4 {
							next.0[y][x] = 'L';
						}
					}
					_ => {}
				}
			}
		}
	}

	next.0
		.iter()
		.flat_map(|row| row.iter())
		.filter(|&&c| c == '#')
		.count()
}

fn part2(input: &Grid) -> usize {
	let mut prev = Grid::new("");
	let mut next = input.clone();
	let height = input.0.len();
	let width = input.0[0].len();

	fn is_occupied(
		grid: &[Vec<char>],
		x: usize,
		y: usize,
		op: impl Fn((i64, i64)) -> (i64, i64),
	) -> usize {
		let height = grid.len() as i64;
		let width = grid[0].len() as i64;
		let mut pos = op((x as i64, y as i64));

		while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
			match grid[pos.1 as usize][pos.0 as usize] {
				'#' => {
					return 1;
				}
				'L' => {
					return 0;
				}
				_ => {}
			}

			pos = op(pos);
		}

		0
	}

	let count_occupied = |grid: &[Vec<char>], x, y| -> usize {
		is_occupied(grid, x, y, |(x, y)| (x - 1, y))
			+ is_occupied(grid, x, y, |(x, y)| (x + 1, y))
			+ is_occupied(grid, x, y, |(x, y)| (x, y - 1))
			+ is_occupied(grid, x, y, |(x, y)| (x, y + 1))
			+ is_occupied(grid, x, y, |(x, y)| (x - 1, y - 1))
			+ is_occupied(grid, x, y, |(x, y)| (x + 1, y - 1))
			+ is_occupied(grid, x, y, |(x, y)| (x - 1, y + 1))
			+ is_occupied(grid, x, y, |(x, y)| (x + 1, y + 1))
	};

	while prev.0 != next.0 {
		prev = next;
		next = prev.clone();

		for y in 0..height {
			for x in 0..width {
				let count = count_occupied(&prev.0, x, y);

				match prev.0[y][x] {
					'L' => {
						if count == 0 {
							next.0[y][x] = '#';
						}
					}
					'#' => {
						if count >= 5 {
							next.0[y][x] = 'L';
						}
					}
					_ => {}
				}
			}
		}
	}

	next.0
		.iter()
		.flat_map(|row| row.iter())
		.filter(|&&c| c == '#')
		.count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = Grid::new(&read_stdin()?);

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}

#[test]
fn test_part1() {
	let input = Grid::new(
		r#"
		L.LL.LL.LL
		LLLLLLL.LL
		L.L.L..L..
		LLLL.LL.LL
		L.LL.LL.LL
		L.LLLLL.LL
		..L.L.....
		LLLLLLLLLL
		L.LLLLLL.L
		L.LLLLL.LL
	"#,
	);

	assert_eq!(part1(&input), 37);
}

#[test]
fn test_part2() {
	let input = Grid::new(
		r#"
		L.LL.LL.LL
		LLLLLLL.LL
		L.L.L..L..
		LLLL.LL.LL
		L.LL.LL.LL
		L.LLLLL.LL
		..L.L.....
		LLLLLLLLLL
		L.LLLLLL.L
		L.LLLLL.LL
	"#,
	);

	assert_eq!(part2(&input), 26);
}
