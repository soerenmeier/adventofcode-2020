
use std::fs::read_to_string;

fn part_one() -> Option<usize> {

	// read input
	let input = read_to_string("./data/input.txt").unwrap();

	// read every line and convert to usize
	let input: Vec<usize> = input.lines()
		.map(|l| l.parse().unwrap())
		.collect();

	for (i, a) in input.iter().enumerate() {
		if i + 1 == input.len() {
			break
		}

		for b in input[(i + 1)..].iter() {
			let sum = a + b;
			if sum == 2020 {
				// part 1
				println!("found result {}", a * b);
				return Some(a * b);
			}
		}
	}

	None
}


fn part_two() -> Option<usize> {

	// read input
	let input = read_to_string("./data/input.txt").unwrap();

	// read every line and convert to usize
	let input: Vec<usize> = input.lines()
		.map(|l| l.parse().unwrap())
		.collect();

	for (i, a) in input.iter().enumerate() {
		if i + 1 == input.len() {
			break
		}

		for (j, b) in input[(i + 1)..].iter().enumerate() {
			if j + 1 == input.len() {
				break
			}

			for c in input[(j + 1)..].iter() {
				let sum = a + b + c;
				if sum == 2020 {
					// part 2
					return Some(a * b * c);
				}
			}
		}
	}

	None
}



fn main() {

	println!("part one {}", part_one().unwrap());

	println!("part two {}", part_two().unwrap());

}
