pub fn bubble_sort(nums: &mut Vec<i32>) {
    loop {        
        let mut swapped = false;

        for i in 1..nums.len() {
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
