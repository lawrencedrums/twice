pub fn selection_sort(nums: &mut Vec<u32>) {
    for i in 0..nums.len() {
        let mut min = i;
        let mut j = i + 1;

        while j < nums.len() {
            if nums[j] < nums[min] {
                min = j;
            }
            j += 1;
        }

        if min != i {
            let temp = nums[min];
            nums[min] = nums[i];
            nums[i] = temp;
        }
    }
}
