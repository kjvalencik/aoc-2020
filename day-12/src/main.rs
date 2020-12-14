use aoc_common::*;

fn part1(input: &str) -> i64 {
	let mut pos = (0, 0);
	let mut dir = (1, 0);
	let dirs = input
		.trim()
		.lines()
		.map(|line| line.trim())
		.map(|line| (&line[0..1], (&line[1..]).parse::<i64>().unwrap()));

	for (d, n) in dirs {
		match d {
			"N" => pos.1 += n,
			"S" => pos.1 -= n,
			"E" => pos.0 += n,
			"W" => pos.0 -= n,
			"F" => {
				pos.0 += n * dir.0;
				pos.1 += n * dir.1;
			}
			"L" | "R" => {
				let sign = if d == "L" { 1 } else { -1 };

				dir = match (dir, n) {
					((1, 0), 180) => (-1, 0),
					((-1, 0), 180) => (1, 0),
					((0, 1), 180) => (0, -1),
					((0, -1), 180) => (0, 1),
					((1, 0), 90) => (0, sign),
					((1, 0), 270) => (0, -sign),
					((-1, 0), 90) => (0, -sign),
					((-1, 0), 270) => (0, sign),
					((0, 1), 90) => (-sign, 0),
					((0, 1), 270) => (sign, 0),
					((0, -1), 90) => (sign, 0),
					((0, -1), 270) => (-sign, 0),
					_ => panic!("Unexpected turn"),
				}
			}
			_ => panic!("Unexpected input"),
		}
	}

	pos.0.abs() + pos.1.abs()
}

fn part2(input: &str) -> i64 {
	let mut ship = (0, 0);
	let mut waypoint = (10, 1);
	let dirs = input
		.trim()
		.lines()
		.map(|line| line.trim())
		.map(|line| (&line[0..1], (&line[1..]).parse::<i64>().unwrap()));

	for (d, n) in dirs {
		match d {
			"N" => waypoint.1 += n,
			"S" => waypoint.1 -= n,
			"E" => waypoint.0 += n,
			"W" => waypoint.0 -= n,
			"F" => {
				ship.0 += n * waypoint.0;
				ship.1 += n * waypoint.1;
			}
			"L" => {
				waypoint = match n {
					90 => (-waypoint.1, waypoint.0),
					180 => (-waypoint.0, -waypoint.1),
					270 => (waypoint.1, -waypoint.0),
					_ => panic!("Unexpected pivot"),
				}
			}
			"R" => {
				waypoint = match n {
					90 => (waypoint.1, -waypoint.0),
					180 => (-waypoint.0, -waypoint.1),
					270 => (-waypoint.1, waypoint.0),
					_ => panic!("Unexpected pivot"),
				}
			}
			_ => panic!("Unexpected input"),
		}
	}

	ship.0.abs() + ship.1.abs()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = read_stdin()?;

	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));

	Ok(())
}

#[test]
fn test_example() {
	let input = r#"
		F10
		N3
		F7
		R90
		F11
	"#;

	assert_eq!(part1(&input), 25);
	assert_eq!(part2(&input), 286);
}
