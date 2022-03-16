pub struct Solution;

use std::cmp::Ordering;

impl Solution {

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mid = (nums1.len() + nums2.len()) / 2;
        let jo = (nums1.len() + nums2.len()) % 2;
        match (nums1.clone().first(), nums2.clone().first()) {
            (None, None) => 0f64,
            (_, None) => {
                if jo == 0 {
                    (nums1[mid - 1] + nums1[mid]) as f64 / 2 as f64
                } else {
                    nums1[mid] as f64
                }
            }
            (None, _) => {
                if jo == 0 {
                    (nums2[mid - 1] + nums2[mid]) as f64 / 2 as f64
                } else {
                    nums2[mid] as f64
                }
            }
            (_, _) => {
                let mut i1 = 0;
                let mut i2 = 0;
                let mut num1 = 0;
                let mut num2 = 0;
                loop{
                    if i1 + i2 > mid {
                        break
                    }
                    num2 = num1;
                    if i1 == nums1.len() {
                        num1 = nums2[i2];
                        i2 += 1;
                        continue
                    }
                    if i2 == nums2.len() {
                        num1 = nums1[i1];
                        i1 += 1;
                        continue
                    }
                    match nums1[i1].cmp(&nums2[i2]){
                        Ordering::Less => {
                            num1 = nums1[i1];
                            i1 += 1;
                        }
                        Ordering::Equal => {
                            num1 = nums1[i1];
                            i1 += 1;
                        }
                        Ordering::Greater => {
                            num1 = nums2[i2];
                            i2 += 1;
                        }
                    }
                }
                if jo == 0 {
                    (num1 + num2) as f64 / 2 as f64
                } else {
                    num1 as f64
                }
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