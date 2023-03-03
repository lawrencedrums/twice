pub fn insertion_sort(nums: &mut Vec<u32>) {
    for i in 1..nums.len() {
        let mut j = i;
        let key = nums[i];

        while j > 0 && nums[j-1] > key {
            nums[j] = nums[j-1];
            j -= 1;
        }
        nums[j] = key
    }
}