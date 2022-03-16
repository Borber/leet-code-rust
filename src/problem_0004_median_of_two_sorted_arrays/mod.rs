pub mod simple;

pub trait Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S:Solution>() {
        let test_case = [
            ((&[1,3] as &[_], &[2] as &[_]), 2.00000),
            ((&[1,2], &[3,4]), 2.50000),
            ((&[0,0], &[0,0]), 0.00000)
        ];
        for ((nums1, nums2), expected) in test_case {
            assert_eq!(S::find_median_sorted_arrays(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }

}