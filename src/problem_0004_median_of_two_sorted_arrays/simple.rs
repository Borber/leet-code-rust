pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {(nums1, nums2)} else { (nums2, nums1) };
        match (nums1.clone().iter().next(), nums2.clone().iter().next()) {
            (None, None) => 0f64,
            (num1, None) => *num1.unwrap() as f64,
            (None, num2) => *num2.unwrap() as f64,
            (_, _) => {
                0f64
            }
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