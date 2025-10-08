// Step2_1
// 目的: step2で違和感を感じたif文の部分を修正する

/*
  他の人のコードを読んで考えたこと
  - 標準ライブラリのHashSet::intersection()メソッド(実装はhashbrownクレート)の実装を確認していたところ、
  if文でよりスッキリかける書き方を見つけた。
  https://docs.rs/hashbrown/latest/src/hashbrown/set.rs.html#799-809

  改善する時に考えたこと
  - step2で書いたif分の部分が冗長で違和感を感じたので、より簡潔な形になるように書きなおす。
*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersected = HashSet::<_>::with_capacity(nums1.len().min(nums2.len()));
        let (smaller_nums, larger_nums) = if nums1.len() <= nums2.len() {
            (nums1, HashSet::<_>::from_iter(nums2))
        } else {
            (nums2, HashSet::<_>::from_iter(nums1))
        };

        for small in smaller_nums {
            if larger_nums.contains(&small) {
                intersected.insert(small);
            }
        }

        intersected.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_1_test() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort();
        assert_eq!(result, vec![2]);

        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
