pub fn radix_sort(nums: &mut Vec<i32>) {
	let max = match nums.iter().max() {
		Some(&max) => max,
		None => return,
	};

	let mut place = 1;
	while max / place > 0 {
		counting_sort(nums, place);
		place *= 10;
	}
}

fn counting_sort(nums: &mut Vec<i32>, place: i32) {
	let len = nums.len();
	let mut res = vec![0; len];
	let mut counter = vec![0; 10];

	// calculate count of elements
	for i in 0..len {
		let digit = (nums[i] / place) % 10;
		counter[digit as usize] += 1;
	}

	// calculate cumulative count
	for i in 1..10 {
		counter[i] += counter[i-1];
	}

	// place the elements in sorted order
	for i in (0..len).rev() {
		let digit = (nums[i] / place) % 10;
		counter[digit as usize] -= 1;
		res[counter[digit as usize]] = nums[i];
	}
	
	for i in 0..len {
		nums[i] = res[i];
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sort() {
		let mut nums = vec![2, 43, 85, 111, 16, 7, 596, 78];
		radix_sort(&mut nums);
		assert_eq!(nums, vec![2, 7, 16, 43, 78, 85, 111, 596]);
	}

	#[test]
	fn test_sort_empty_array() {
		let mut nums = Vec::<i32>::new();
		radix_sort(&mut nums);
		assert_eq!(nums, vec![]);
	}

	#[test]
	fn test_sort_one_element() {
		let mut nums = vec![1];
		radix_sort(&mut nums);
		assert_eq!(nums, vec![1]);
	}
}