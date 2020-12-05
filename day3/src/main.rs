
use std::fs::read_to_string;

struct Map<'a> {
	lines: Vec<&'a str>
}

impl<'a> Map<'a> {

	fn new(s: &'a str) -> Self {
		Self {
			lines: s.lines().collect()
		}
	}

	fn is_tree(&self, x: usize, y: usize) -> bool {
		// we panic if y is > lines.len
		// x is modulo
		let line = self.lines[y];
		let len = line.len();

		line.as_bytes()[x % len] == b'#'
	}

	fn count_trees(&self, c_x: usize, c_y: usize) -> usize {
		let mut trees = 0;
		let mut x = 0;
		let mut y = 0;

		while y < self.lines.len() {
			if self.is_tree(x, y) {
				trees += 1;
			}
			x += c_x;
			y += c_y;
		}

		trees
	}

}


fn part_one() -> usize {

	let input = read_to_string("./data/input.txt").unwrap();
	let map = Map::new(&input);

	map.count_trees(3, 1)
}

fn part_two() -> usize {

	let mut res = 1;
	let input = read_to_string("./data/input.txt").unwrap();
	let map = Map::new(&input);

	res *= map.count_trees(1, 1);
	res *= map.count_trees(3, 1);
	res *= map.count_trees(5, 1);
	res *= map.count_trees(7, 1);
	res *= map.count_trees(1, 2);

	res
}


fn main() {

	println!("part one {}", part_one());
	println!("part two {}", part_two());


}