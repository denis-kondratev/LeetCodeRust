/// # [1140. Stone Game II](https://leetcode.com/problems/stone-game-ii/description/?envType=daily-question&envId=2024-08-20)
///
/// Alice and Bob continue their games with piles of stones. There are a number of piles
/// arranged in a row, and each pile has a positive integer number of stones `piles[i]`.
/// The objective of the game is to end with the most stones.
///
/// Alice and Bob take turns, with Alice starting first. Initially, `M = 1`.
/// On each player's turn, that player can take all the stones in the first `X`
/// remaining piles, where `1 <= X <= 2M`.  Then, we set `M = max(M, X)`.
///
/// The game continues until all the stones have been taken. Assuming Alice and Bob
/// play optimally, return the maximum number of stones Alice can get.
///
/// # Example 1:
/// - Input: `piles` = \[2,7,9,4,4]
/// - Output: 10
/// - Explanation: If Alice takes one pile at the beginning, Bob takes two piles,
///   then Alice takes 2 piles again. Alice can get `2 + 4 + 4 = 10` piles in total.
///   If Alice takes two piles at the beginning, then Bob can take all three piles left.
///   In this case, Alice get `2 + 7 = 9` piles in total. So we return 10 since it's larger.
///
/// ## Example 2:
/// - Input: `piles` = \[1,2,3,4,5,100]
/// - Output: 104
///
/// ## Constraints:
/// - 1 <= `piles.length` <= 100
/// - 1 <= `piles[i]` <= 10<sup>4</sup>
pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::get_stones(&piles, 0, 1).0
    }

    fn get_stones(piles: &Vec<i32>, start: usize, m: usize) -> (i32, usize) {
        if start >= piles.iter().len() {
            return (0, m);
        }

        let next_m1 = Self::get_stones(piles, start + m, m).1;
        let sum1 = Self::get_sum(piles, start, m) + Self::get_stones(piles, start + m + next_m1, next_m1).0;
        let m2 = m * 2;
        let next_m2 = Self::get_stones(piles, start + m2, m2).1;
        let sum2 = Self::get_sum(piles, start, m2) + Self::get_stones(piles, start + m2 + next_m2, next_m2).0;

        if sum1 > sum2 {
            (sum1, m)
        }
        else {
            (sum2, m * 2)
        }
    }

    fn get_sum(piles: &Vec<i32>, start: usize, count: usize) -> i32 {
        if start >= piles.len() {
            return 0;
        }

        (&piles[start..std::cmp::min(start + count, piles.len())])
            .iter()
            .sum()
    }
}
