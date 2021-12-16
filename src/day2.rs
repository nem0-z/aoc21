use std::fs;

pub fn run(filename: &str) {
	let mut horizontal = 0;
	let mut depth = 0;
	let mut aim = 0;
	let data = fs::read_to_string(filename)
		.expect("error reading file");
	let vec = data.split("\n").collect::<Vec<&str>>();

	for row in vec {
		let mut cmd = row.split(" ");
		let direction = String::from(cmd.next().unwrap());
		let value = cmd.next().unwrap().parse::<i32>().unwrap();
		match direction.as_str() {
			"forward" => {
				horizontal += value;
				depth += aim * value;
			},
			"down"	  => aim += value,
			"up"	  => aim -= value,
			_         => println!("bad input"),
		}
	}

	println!("horizontal: {}, depth: {}, res: {}", horizontal, depth, horizontal * depth);
}