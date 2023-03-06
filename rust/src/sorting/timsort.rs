use std::cmp::min;


static RUN: usize = 32;

pub fn timsort(nums: &mut Vec<i32>) {
	let len = nums.len();

	let mut left = 0;
	while left < len {
		let right = min(left+RUN, len);
		insertion(nums, left, right);
		left += RUN;
	}

	let mut merge_size = RUN;
	while merge_size < len {
		let mut left = 0;
		while left < len {
			let mid = left + merge_size;
			let right = min(left + 2*merge_size, len);
			if mid < right {
				merge(nums, left, mid, right);
			}
			left += 2 * merge_size;
		}
		merge_size *= 2;
	}
}

fn insertion(nums: &mut Vec<i32>, left: usize, right: usize) {
	for i in left+1..right {
		let mut ptr = i;
		let key = nums[i];

		while ptr > left && nums[ptr-1] > key {
			nums[ptr] = nums[ptr-1];
			ptr -= 1;
		}
		nums[ptr] = key;
	}
}

fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
	let mut merged_array = vec![0; right - left];
	let mut l = left;
	let mut r = mid;
	let mut ptr = 0;

	while l < mid && r < right {
		if nums[l] < nums[r] {
			merged_array[ptr] = nums[l];
			l += 1;
		} else {
			merged_array[ptr] = nums[r];
			r += 1;
		}
		ptr += 1;
	}

	while l < mid {
		merged_array[ptr] = nums[l];
		ptr += 1;
		l += 1;
	}
	while r < right {
		merged_array[ptr] = nums[r];
		ptr += 1;
		r += 1;
	}

	// replace slice form left idx to right idx with merged array
	nums.splice(left..right, merged_array.iter().cloned());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sort() {
		let mut nums = vec!(3, 2, 1, 0, 8, 4, 6, 7, 5, 9);
		timsort(&mut nums);
		assert_eq!(nums, vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
	}
}