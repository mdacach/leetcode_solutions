pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let max_value = *nums.iter().max().unwrap();
    if max_value <= 0 {
        return max_value;
    }

    let mut answer = nums[0];
    let mut sum_until_now = std::cmp::max(0, nums[0]);

    for num in nums.into_iter().skip(1) {
        if sum_until_now + num >= 0 {
            sum_until_now += num;
        } else {
            sum_until_now = 0;
        }
        answer = std::cmp::max(answer, sum_until_now);
    }

    answer
}
