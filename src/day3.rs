use std::fs;

pub fn run(filename: &str) {
	let data = fs::read_to_string(filename)
		.expect("error reading file");
	let mut vec = data.split("\n").collect::<Vec<&str>>();
    let mut ones = 0;
    let mut zeros = 0;
	let mut gamma_rate: String = String::with_capacity(vec.len());
	let mut epsilon_rate: String = String::with_capacity(vec.len());

	for i in 0..vec[0].len() {
		for j in 0..vec.len() {
			let bits = vec[j].as_bytes();
			let bit = bits[i] as char;
			if bit == '1' {
				ones += 1;
			} else {
				zeros += 1;
			}
		}

		gamma_rate.push(if ones > zeros {'1'} else {'0'});
		epsilon_rate.push(if ones > zeros {'0'} else {'1'});
		ones = 0;
		zeros = 0;
	}

	let gamma_rate_num = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
	let epsilon_rate_num = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
	println!("{}", gamma_rate_num * epsilon_rate_num);

	let oxygen_generator = get_oxygen_generator(&vec);
	let co2_scrubber = get_co2_scrubber(&vec);

	println!("{}", oxygen_generator * &co2_scrubber);
}

fn get_oxygen_generator(vec_arg: &Vec<&str>) -> i32 {
	let mut ones = 0;
	let mut zeros = 0;
	let mut vec = vec_arg.clone();
	let mut new_vec: Vec<&str> = Vec::new();

	for i in 0..vec[0].len() {
		for j in 0..vec.len() {
			let bits = vec[j].as_bytes();
			let bit = bits[i] as char;
			if bit == '1' {
				ones += 1;
			} else {
				zeros += 1;
			}
		}
		if ones > zeros || ones == zeros {
			for k in 0..vec.len() {
				if vec[k].as_bytes()[i] as char == '1' {
					new_vec.push(vec[k]);
				}
			}
		} else {
			for k in 0..vec.len() {
				if vec[k].as_bytes()[i] as char == '0' {
					new_vec.push(vec[k]);
				}
			}
		}
		ones = 0;
		zeros = 0;
		vec = new_vec.clone();
		new_vec.clear();

		if vec.len() == 1 {
			let oxygen_generator = isize::from_str_radix(vec[0], 2).unwrap();
			return oxygen_generator.try_into().unwrap();
		}
	}

	-1
}

fn get_co2_scrubber(vec_arg: &Vec<&str>) -> i32 {
	let mut ones = 0;
	let mut zeros = 0;
	let mut vec = vec_arg.clone();
	let mut new_vec: Vec<&str> = Vec::new();

	for i in 0..vec[0].len() {
		for j in 0..vec.len() {
			let bits = vec[j].as_bytes();
			let bit = bits[i] as char;
			if bit == '1' {
				ones += 1;
			} else {
				zeros += 1;
			}
		}
		if zeros < ones || ones == zeros {
			for k in 0..vec.len() {
				if vec[k].as_bytes()[i] as char == '0' {
					new_vec.push(vec[k]);
				}
			}
		} else {
			for k in 0..vec.len() {
				if vec[k].as_bytes()[i] as char == '1' {
					new_vec.push(vec[k]);
				}
			}
		}
		ones = 0;
		zeros = 0;
		vec = new_vec.clone();
		new_vec.clear();

		if vec.len() == 1 {
			let co2_scrubber = isize::from_str_radix(vec[0], 2).unwrap();
			return co2_scrubber.try_into().unwrap();
		}
	}

	-1
}
