struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, a) in nums.iter().enumerate() {
            let b = target - a;

            if let Some(&j) = map.get(&b) {
                return vec!(i as i32, j as i32);
            }

            map.insert(*a, i);
        }

        vec!(-1, -1)
    }
}

fn main() {
    println!("Hello, world!");
}
