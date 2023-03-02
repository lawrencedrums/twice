pub fn bubble_sort(nums: &mut Vec<u32>) {
    for _ in 0..nums.len() {
        let mut swapped = false;
        for i in 1..nums.len() {
            if nums[i-1] > nums[i] {
                let temp = nums[i-1];
                nums[i-1] = nums[i];
                nums[i] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
