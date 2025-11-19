// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums.len()
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 1分20秒
  2回目: 1分20秒
  3回目: 1分14秒
*/

/*
  所感
  - アルゴリズムの威力を感じる問題だった。
    - 問題文を見て素直に手作業で行おうとすると様々なことを考慮しながら、多くの手順を必要とするが、アルゴリズムを適用することで非常にシンプルに答えを求められるので。
*/

pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        let mut max_sum = nums[0];
        let mut sum = nums[0];

        for num in nums.into_iter().skip(1) {
            sum = num.max(sum + num);
            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::max_sub_array(vec![-1, -2]), -1);
        assert_eq!(Solution::max_sub_array(vec![1, -1, 1]), 1);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
    }

    #[test]
    #[should_panic(expected = "nums must not be empty.")]
    fn step3_panic_test() {
        Solution::max_sub_array(vec![]);
    }
}
