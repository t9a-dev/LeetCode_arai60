// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums1.len()
  m = nums2.len()
  k = min(k,n*m)
  時間計算量: O(n + k log n) <- O(k log n)かと思ったが違った
  空間計算量: O(n + k) <- O(n)かと思ったが違った
*/

/*
  1回目: 8分33秒
  2回目: 7分14秒
  3回目: 5分32秒
*/

use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

pub struct Solution {}
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if k <= 0 {
            return vec![];
        }

        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }

        let mut k = k as usize;
        let pairs_count = nums1.len() * nums2.len();
        k = min(pairs_count, k);

        let mut top_k_smallest_pairs = vec![];
        let mut smallest_by_sum: BinaryHeap<(Reverse<i64>, (usize, usize))> = BinaryHeap::new();

        for (i, num1) in nums1.iter().enumerate() {
            let sum = num1 + nums2[0];
            smallest_by_sum.push((Reverse(sum.into()), (i, 0)));
        }

        while let Some((_, (i, j))) = smallest_by_sum.pop() {
            if top_k_smallest_pairs.len() == k {
                break;
            }

            top_k_smallest_pairs.push(vec![nums1[i], nums2[j]]);

            let next_j = j + 1;
            if let Some(num2) = nums2.get(next_j) {
                let sum = nums1[i] + *num2;
                smallest_by_sum.push((Reverse(sum.into()), (i, next_j)));
            }
        }

        top_k_smallest_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 2]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 4),
            vec![vec![1, 1], vec![1, 2], vec![1, 3]]
        );

        let result: Vec<Vec<_>> = Vec::new();
        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 0),
            result
        );
    }
}
