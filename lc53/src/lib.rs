pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let node = create_node(&nums);
    node.max_segment
}

pub fn create_node(nums: &[i32]) -> Node {
    let len = nums.len();
    if len == 1 {
        return Node {
            max_suffix: nums[0],
            max_prefix: nums[0],
            all_segment: nums[0],
            max_segment: nums[0],
        };
    }

    let left_node = create_node(&nums[0..len / 2]);
    let right_node = create_node(&nums[len / 2..]);

    merge(left_node, right_node)
}

pub fn merge(left: Node, right: Node) -> Node {
    Node {
        max_prefix: std::cmp::max(left.max_prefix, left.all_segment + right.max_prefix),
        max_suffix: std::cmp::max(right.max_suffix, right.all_segment + left.max_suffix),
        max_segment: std::cmp::max(
            left.max_suffix + right.max_prefix,
            std::cmp::max(left.max_segment, right.max_segment),
        ),
        all_segment: left.all_segment + right.all_segment,
    }
}

pub struct Node {
    max_prefix: i32,
    max_suffix: i32,
    max_segment: i32,
    all_segment: i32,
}
