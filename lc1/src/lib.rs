use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (index, current) in nums.iter().enumerate() {
        let index = index as i32;
        let wanted = target - current;
        if let Some(&other_index) = seen.get(&wanted) {
            return vec![index, other_index];
        }
        seen.insert(current, index);
    }
    panic!() // We are guaranteed to have one solution.
}
