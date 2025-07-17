/// # [57. Insert Interval](https://leetcode.com/problems/insert-interval/)
///
/// You are given an array of non-overlapping `intervals` where `intervals[i]` =
/// \[start<sub>i</sub>, end<sub>i</sub>] represent the start and the end of the ith interval
/// and `intervals` is sorted in ascending order by start<sub>i</sub>. You are also given an
/// `newInterval` = \[start, end] that represents the start and end of another interval.
///
/// Insert `newInterval` into intervals such that intervals is still sorted in ascending order
/// by start<sub>i</sub> and `intervals` are not have any overlaps (merge overlapping intervals
/// if necessary).
///
/// Return `intervals` after the insertion.
///
/// **Note** that you don't need to modify intervals in-place. You can make a new array
/// and return it.
///
/// ## Example 1:
/// - Input: `intervals` = `[[1,3],[6,9]]`, `newInterval` = `[2,5]`
/// - Output: `[[1,5],[6,9]]`
///
/// ## Example 2:
/// - Input: `intervals` = `[[1,2],[3,5],[6,7],[8,10],[12,16]]`, `newInterval` = `[4,8]`
/// - Output: `[[1,2],[3,10],[12,16]]`
/// Explanation: Because the new interval `[4,8]` overlaps with `[3,5]`, `[6,7]`, `[8,10]`.
///
/// ## Constraints:
/// - 0 <= `intervals.length` <= 10<sup>4</sup>
/// - `intervals[i].length` == 2
/// - 0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>5</sup>
/// - `intervals` is sorted by `start`<sub>i</sub> in ascending order.
/// - `newInterval.length` == 2
/// - 0 <= `start` <= `end` <= 10<sup>5</sup>
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }

        let mut result = Vec::new();
        let mut i = 0;
        let mut new_interval = (new_interval[0], new_interval[1]);

        while i < intervals.len() {
            let interval = &intervals[i];
            let interval = (interval[0], interval[1]);

            if interval.0 > new_interval.1 {
                break;
            }

            if interval.1 >= new_interval.0 {
                new_interval.0 = std::cmp::min(new_interval.0, interval.0);
                new_interval.1 = std::cmp::max(new_interval.1, interval.1);
            } else {
                result.push(vec![interval.0, interval.1]);
            }

            i += 1;
        }

        result.push(vec![new_interval.0, new_interval.1]);
        result.extend_from_slice(&intervals[i..]);
        result
    }
}
