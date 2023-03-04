pub fn top_down_merge_sort<T: Ord + Copy>(nums: &mut [T]) {
	if nums.len() <= 1 {
		return;
	}

	let mid = nums.len() / 2;
	top_down_merge_sort(&mut nums[..mid]);
	top_down_merge_sort(&mut nums[mid..]);
	merge(nums, mid);
}

pub fn bottom_up_merge_sort<T: Ord + Copy>(nums: &mut [T]) {}

fn merge<T: Ord + Copy>(nums: &mut [T], mid: usize) {
	let left_half = nums[..mid].to_vec();
	let right_half = nums[mid..].to_vec();

	let mut l = 0;
	let mut r = 0;

	for num in nums {
		if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
			*num = left_half[l];
			l += 1;
		} else {
			*num = right_half[r];
			r += 1;
		}
	}
}
