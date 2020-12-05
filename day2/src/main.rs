

use std::fs::read_to_string;

// 10-16 r: nrrrrkrjtxwrrrwx



fn part_one() -> usize {

	// read input
	let input = read_to_string("./data/input.txt").unwrap();
	let mut valid = 0;

	for line in input.lines() {

		// split white space
		let mut line = line.split(' ');

		let mut range = line.next().unwrap().split('-');
		let from: usize = range.next().unwrap()
			.parse().unwrap();
		let to: usize = range.next().unwrap()
			.parse().unwrap();

		let letter = line.next().unwrap()
			.chars().nth(0).unwrap();

		let password = line.next().unwrap();

		// now lets count occurences
		let occurences = password.chars()
			.filter(|&c| c == letter)
			.count();

		if occurences >= from && occurences <= to {
			valid += 1;
		}

	}

	valid
}

fn part_two() -> usize {

	// read input
	let input = read_to_string("./data/input.txt").unwrap();
	let mut valid = 0;

	for line in input.lines() {

		// split white space
		let mut line = line.split(' ');

		let mut range = line.next().unwrap().split('-');
		let from: usize = range.next().unwrap()
			.parse().unwrap();
		let to: usize = range.next().unwrap()
			.parse().unwrap();

		let letter = line.next().unwrap()
			.chars().nth(0).unwrap();

		let password = line.next().unwrap();

		let letter_from = password.chars().nth(from-1).unwrap();
		let letter_to = password.chars().nth(to-1).unwrap();

		if letter == letter_from && letter == letter_to {
			// not valid
		} else if letter == letter_from || letter == letter_to {
			valid += 1;
		}

	}

	valid
}


fn main() {

	println!("part one {}", part_one());
	println!("part two {}", part_two());

}