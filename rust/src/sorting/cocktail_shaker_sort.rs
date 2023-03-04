pub fn cocktail_shaker_sort<T: Ord>(nums: &mut [T]) {
	if nums.len() < 1 {
		return;
	}

	let len = nums.len();

	loop {
		let mut swapped = false;

		for i in 1..len {
			if nums[i-1] > nums[i] {
				nums.swap(i-1, i);
				swapped = true;
			}
		}
		if !swapped {
			break;
		}

		swapped = false;

		for i in (1..len).rev() {
			if nums[i-1] > nums[i] {
				nums.swap(i-1, i);
				swapped = true;
			}
		}
		if !swapped {
			break;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sort() {
		let mut nums = vec![3, 5, 1, 9, 6];
		cocktail_shaker_sort(&mut nums);
		assert_eq!(nums, vec![1, 3, 5, 6, 9]);
	}

	#[test]
	fn test_sort_empty_array() {
		let mut nums = Vec::<i32>::new();
		cocktail_shaker_sort(&mut nums);
		assert_eq!(nums, vec![]);
	}

	#[test]
	fn test_sort_one_element() {
		let mut nums = vec![1];
		cocktail_shaker_sort(&mut nums);
		assert_eq!(nums, vec![1]);
	}
}