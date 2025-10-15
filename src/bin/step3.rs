// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

use std::collections::HashMap;

/*
  入力numsのサイズをNとする。
  時間計算量: O(N)
  空間計算量: O(N)
*/

/*
 1回目: 3分27秒
 2回目: 2分36秒
 3回目: 2分46秒
*/

pub struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut count: i64 = 0;
        let mut prefix_sum: i64 = 0;
        let mut frequency_by_prefix_sum: HashMap<_, _> = HashMap::new();
        frequency_by_prefix_sum.insert(0, 1);

        for num in nums.into_iter() {
            prefix_sum += num as i64;
            let complement = prefix_sum - k;

            if let Some(frequency) = frequency_by_prefix_sum.get(&complement) {
                count += frequency;
            }

            frequency_by_prefix_sum
                .entry(prefix_sum)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        count.try_into().unwrap_or(i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);

        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 1), 3);
        assert_eq!(
            Solution::subarray_sum(vec![i32::MAX, i32::MAX, i32::MAX], i32::MAX),
            3
        );
        assert_eq!(
            Solution::subarray_sum(vec![i32::MIN, i32::MIN, i32::MIN], i32::MIN),
            3
        );
    }
}
