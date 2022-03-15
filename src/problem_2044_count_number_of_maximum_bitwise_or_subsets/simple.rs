pub struct Solution;

impl Solution {
    fn dfs(index: usize, current: usize, nums: &Vec<i32>, max: usize) -> i32 {
        if index == nums.len() {
            if current == max {
                return 1;
            }
            return 0;
        };
        Self::dfs(index + 1, current | nums[index] as usize, nums, max) + Self::dfs(index + 1, current, nums, max)
    }

    fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max = nums.iter().fold(0, |acc, &n| acc | n);
        Self::dfs(0, 0, &nums, max as usize)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //
impl super::Solution for Solution {
    fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        Self::count_max_or_subsets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}