use std::fs;

pub fn run(filename: &str) {
	let data = fs::read_to_string(filename)
		.expect("error reading file");
	let vec = data.split("\n").collect::<Vec<&str>>();
	let mut vec_nums: Vec<i32> = Vec::new();
	let mut prev_num = 0;
	let mut prev_sum = 0;
	let mut counter = -1;
	let mut counter_three = -1;

	for s in vec {
		let num = s.parse::<i32>().unwrap();
		if num > prev_num {
			counter += 1;
		}
		prev_num = num;
		vec_nums.push(num);
	}
	println!("{}", counter);

	for i in 0..vec_nums.len() {
		if i > vec_nums.len() - 3 {
			break;
		}
		let sum = vec_nums[i] + vec_nums[i+1] + vec_nums[i+2];
		if sum > prev_sum {
			counter_three += 1;
		}
		prev_sum = sum;
	}
	println!("{}", counter_three);
}