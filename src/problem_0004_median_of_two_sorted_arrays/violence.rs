pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = [nums1, nums2].concat();
        nums.sort();
        if nums.len() % 2 == 0 {
            ((nums[nums.len() / 2] + (nums[nums.len() / 2 - 1])) as f64) / 2.0
        } else {
            nums[nums.len() / 2] as f64
        }
    }
}

impl super::Solution for Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Self::find_median_sorted_arrays(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}