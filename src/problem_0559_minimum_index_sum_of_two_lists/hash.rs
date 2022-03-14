pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let hash = list1.into_iter()
            .enumerate()
            .map(|(i, k)| (k, i))
            .collect::<HashMap<_,_>>();
        let mut min = usize::MAX;
        let mut result = vec![];
        list2.into_iter().enumerate().for_each(|(i, k)|{
            if let Some(&j) = hash.get(&k) {
                match min.cmp(&(i + j)) {
                    Ordering::Greater => {
                        min = i + j;
                        result.clear();
                        result.push(k);
                    }
                    Ordering::Equal => {
                        result.push(k);
                    }
                    _ => {}
                }
            }
        });
        result
    }

}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        Self::find_restaurant(list1, list2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}