
use std::fs::read_to_string;

/*
26 yes-or-no
a-z
*/

// expecting a-z
fn id(c: char) -> usize {
	c as usize - b'a' as usize
}

#[test]
fn test_id() {
	assert_eq!(id('a'), 0);
	assert_eq!(id('z'), 25);
}


fn part_one() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();

	let mut sum = 0;

	for group in input.split("\n\n") {
		let mut list = [false; 26];
		for person in group.split('\n') {
			for c in person.chars() {
				list[id(c)] = true;
			}
		}
		sum += list.iter().filter(|&&b| b).count();
	}

	sum
}

fn part_two() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();

	let mut sum = 0;

	for group in input.split("\n\n") {
		let mut list = [0; 26];
		let mut persons = 0;
		for person in group.split('\n') {
			persons += 1;
			for c in person.chars() {
				list[id(c)] += 1;
			}
		}
		sum += list.iter().filter(|&&c| c == persons).count();
	}

	sum
}


fn main() {

	println!("part one {}", part_one());
	println!("part two {}", part_two());

}