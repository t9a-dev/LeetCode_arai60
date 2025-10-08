// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

use std::collections::HashSet;

/*
  nums1のサイズをN,nums2のサイズをMとする
  時間計算量: O(N + M) HashSetの初期化(from_iter())でO(1)の操作をN回,M回行うため。
  空間計算量: O(N + M) num1,num2のHashSetが同時に存在するため。
*/

/*
  memo:
  ワンライナーで書くとnums1とnums2を両方HashSetにする必要があるので、空間計算量が増える。
  step2_1.rsの実装だと入力のうち、どちらか一方だけをHashSetにして、もう一方はそのまま（in-place）操作するので、
  空間計算量が優れている。
*/

pub struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        HashSet::<_>::from_iter(nums1)
            .intersection(&HashSet::from_iter(nums2))
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort();
        assert_eq!(result, vec![2]);

        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
