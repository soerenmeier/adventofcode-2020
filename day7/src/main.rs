
use std::fs::read_to_string;
use std::collections::{ HashMap, HashSet };

fn trim_bags(s: &str) -> &str {
	let s = s.trim_end_matches("bags");
	let s = s.trim_end_matches("bag");
	s.trim_end()
}


fn part_one() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();

	// bag > contained in
	let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

	for rule in input.lines() {
		let rule = rule.trim_end_matches('.');
		let cont_pos = rule.find("contain").unwrap();
		let main_bag = trim_bags(rule[..cont_pos].trim());

		for inner_bag in rule[(cont_pos + "contains".len())..].trim().split(',') {
			let inner_bag = inner_bag.trim();
			assert!(inner_bag.ends_with("bags") || inner_bag.ends_with("bag"));
			if inner_bag == "no other bags" {
				break
			}

			let space = inner_bag.find(' ').unwrap();
			//let amount = inner_bag[..space].parse::<usize>().unwrap();

			let inner_bag = trim_bags(&inner_bag[(space + 1)..]);

			let v = map.entry(inner_bag).or_insert(vec![]);
			v.push(main_bag);
		}
	}

	// now we start at shiny gold
	
	let mut hash_set = HashSet::new();
	count_occurences("shiny gold", &map, &mut hash_set);
	hash_set.len()
}

fn count_occurences<'a>(bag: &'a str, map: & HashMap<&'a str, Vec<&'a str>>, main_bags: &mut HashSet<&'a str>) {
	let list = match map.get(bag) {
		Some(list) => list,
		None => return
	};

	for main_bag in list {
		main_bags.insert(main_bag);
		count_occurences(main_bag, map, main_bags);
	}
}

fn part_two() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();

	// bag > containing amount bag
	let mut map: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();

	for rule in input.lines() {
		let rule = rule.trim_end_matches('.');
		let cont_pos = rule.find("contain").unwrap();
		let main_bag = trim_bags(rule[..cont_pos].trim());

		let mut list = vec![];

		for inner_bag in rule[(cont_pos + "contains".len())..].trim().split(',') {
			let inner_bag = inner_bag.trim();
			assert!(inner_bag.ends_with("bags") || inner_bag.ends_with("bag"));
			if inner_bag == "no other bags" {
				break
			}

			let space = inner_bag.find(' ').unwrap();
			let amount = inner_bag[..space].parse::<usize>().unwrap();

			let inner_bag = trim_bags(&inner_bag[(space + 1)..]);

			list.push((amount, inner_bag));
		}

		if list.len() > 0 {
			map.insert(main_bag, list);
		}
	}

	// now we start at shiny gold
	
	count_containing("shiny gold", &map)
}

fn count_containing(bag: &str, map: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
	let list = match map.get(bag) {
		Some(list) => list,
		None => return 0
	};
	let mut c = 0;

	for (amount, bag) in list {
		c += amount;
		c += amount * count_containing(bag, map);
	}

	c
}

fn main() {

	println!("part_one {}", part_one());
	println!("part_two {}", part_two());

}
