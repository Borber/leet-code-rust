pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        const INVALID_INDEX: usize = usize::MAX;
        let s = s.into_bytes();
        let mut result = 0;
        let mut start = 0;
        let mut byte_locations = [INVALID_INDEX; 256];

        for (i, byte) in s.iter().copied().enumerate() {
            let old_index = mem::replace(&mut byte_locations[byte as usize], i);
            if old_index == INVALID_INDEX {
                result = result.max(i - start + 1);
            } else {
                for removed_byte in s[start..old_index].iter().copied() {
                    byte_locations[removed_byte as usize] = INVALID_INDEX;
                }
                start = old_index + 1;
            }
        }
        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        Self::length_of_longest_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}