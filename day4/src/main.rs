
use std::fs::read_to_string;

// Not a really nice solution
// but without using regex :)

const FIELDS: [&str; 7] = [
	"byr:", // Birth Year
	"iyr:",
	"eyr:",
	"hgt:",
	"hcl:",
	"ecl:",
	"pid:"
	// "cid"
];

fn part_one() -> usize {
	let mut valid = 0;

	let input = read_to_string("./data/input.txt").unwrap();

	// empty line
	'outer: for passport in input.split("\n\n") {
		for field in &FIELDS {
			if !passport.contains(field) {
				continue 'outer
			}
		}
		valid += 1;
	}

	valid
}

fn valid_field( key: &str, value: &str ) -> bool {
	match key {
		// for digits at least 1920 and at most 2002
		"byr" =>
			value.len() == 4
			&& value.parse::<usize>()
				.map(|num|
					num >= 1920 && num <= 2002
				)
				.unwrap_or(false),
		// four digits at least 2010 and at most 2020
		"iyr" =>
			value.len() == 4
			&& value.parse::<usize>()
				.map(|num|
					num >= 2010 && num <= 2020
				)
				.unwrap_or(false),
		// for digits at least 2020 at most 2030
		"eyr" => value.len() == 4
			&& value.parse::<usize>()
				.map(|num|
					num >= 2020 && num <= 2030
				)
				.unwrap_or(false),
		// a number followed by either cm or in
		"hgt" =>
			// valid ending
			value.len() > 2 && ( value.ends_with("cm") || value.ends_with("in") )
			// valid number
			&& value[..(value.len() - 2)].parse::<usize>()
				.map( |num| if value.ends_with("cm") {
					num >= 150 && num <= 193
				} else {// inch
					num >= 59 && num <= 76
				} )
				.unwrap_or(false),
		"hcl" =>
			value.len() == 7
			&& value.starts_with('#')
			&& value[1..].chars().all(|c| match c {
				'0'..='9' | 'a'..='f' => true,
				_ => false
			}),
		"ecl" => match value {
			"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
			_ => false
		},
		"pid" =>
			value.len() == 9
			&& value.parse::<usize>().is_ok(),
		"cid" => true,
		_ => panic!("unrecognized {}", key)
	}
}

fn part_two() -> usize {
	let mut valid = 0;

	let input = read_to_string("./data/input.txt").unwrap();

	// empty line
	'outer: for passport in input.split("\n\n") {
		// fields
		let mut fields = 0;
		for field in passport.split(|c| c == '\n' || c == ' ') {
			let mut field = field.split(':');
			let key = field.next().unwrap();
			let value = field.next().unwrap();
			if !valid_field(key, value) {
				continue 'outer
			}
			if key != "cid" {
				fields += 1;
			}
		}
		if fields >= FIELDS.len() {
			valid += 1;
		}
	}

	valid
}


fn main() {

	println!("part_one {}", part_one());
	println!("part_two {}", part_two());

}


#[test]
fn valid_fields() {
	assert!(valid_field("byr", "2002"));
	assert!(!valid_field("byr", "2003"));

	assert!(valid_field("hgt", "60in"));
	assert!(valid_field("hgt", "190cm"));
	assert!(!valid_field("hgt", "190in"));
	assert!(!valid_field("hgt", "190"));

	assert!(valid_field("hcl", "#123abc"));
	assert!(!valid_field("hcl", "#123abz"));
	assert!(!valid_field("hcl", "123abz"));

	assert!(valid_field("ecl", "brn"));
	assert!(!valid_field("ecl", "wat"));

	assert!(valid_field("pid", "000000001"));
	assert!(!valid_field("pid", "0123456789"));
}