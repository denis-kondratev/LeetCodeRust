struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;

        loop {
            if nums1[i] == nums2[j] {
                return nums1[i];
            }

            if nums1[i] < nums2[j] {
                i += 1;

                if i == nums1.len() {
                    return -1;
                }
            } else {
                j += 1;

                if j == nums2.len() {
                    return -1;
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
