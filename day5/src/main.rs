
use std::fs::read_to_string;

/*
128 rows
0..=127

7Chars
F=0..=63
 B=32..=63
B=64..=127
*/

fn binary_space_partitioning(
	entries: &str,
	lower_char: char,
	mut from: usize,
	mut to: usize// is from..=to (inclusive)
) -> usize {
	for entry in entries.chars() {
		// means the upper bound
		let dif = (to + 1 - from) / 2;
		if entry == lower_char {
			to = from + dif - 1;
		} else {
			from += dif;
		}
	}
	from
}

#[test]
fn test_binary_space_partitioning() {
	assert_eq!(binary_space_partitioning("FBFBBF", 'F', 0, 127), 44);
	assert_eq!(binary_space_partitioning("BFFFBBF", 'F', 0, 127), 70);
	assert_eq!(binary_space_partitioning("RRR", 'L', 0, 7), 7);
	assert_eq!(binary_space_partitioning("FFFBBBF", 'F', 0, 127), 14);
	assert_eq!(binary_space_partitioning("RRR", 'L', 0, 7), 7);
	assert_eq!(binary_space_partitioning("BBFFBBF", 'F', 0, 127), 102);
	assert_eq!(binary_space_partitioning("RLL", 'L', 0, 7), 4);
}

fn row_col(boarding_pass: &str) -> (usize, usize) {
	let row = binary_space_partitioning(
		&boarding_pass[..7], 'F', 0, 127
	);
	let col = binary_space_partitioning(
		&boarding_pass[7..], 'L', 0, 7
	);
	(row, col)
}

fn seat_id(row: usize, col: usize) -> usize {
	row * 8 + col
}


fn part_one() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();

	input.lines()
		.map(|boarding_pass| {
			let (row, col) = row_col(boarding_pass);
			seat_id(row, col)
		})
		.max()
		.unwrap()
}

fn part_two() -> usize {

	// find seat

	let input = read_to_string("./data/input.txt").unwrap();

	// we have 128 rows
	// and 8 cols
	let mut missing = [false; 128 * 8];

	for boarding_pass in input.lines() {

		let (row, col) = row_col(boarding_pass);
		let i = row * 8 + col;
		missing[i] = true;

	}

	for (mut id, _) in missing[1..].iter().enumerate().filter(|(_, &m)| !m) {
		id += 1;
		if *missing.get(id - 1).unwrap_or(&false)
			&& *missing.get(id + 1).unwrap_or(&false) {
			// found
			return id
		}
	}

	panic!("seat not found")
}


fn main() {

	println!("part one {}", part_one());
	println!("part two {}", part_two());

}