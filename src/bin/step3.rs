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
  1回目: 3分20秒
  2回目: 3分55秒
  3回目: 3分05秒
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let collect_max_amount = |targets: &[i32]| {
            let mut two_before_max = 0;
            let mut one_before_max = 0;
            for target in targets {
                let current_max = one_before_max.max(two_before_max + *target);
                two_before_max = one_before_max;
                one_before_max = one_before_max.max(current_max);
            }
            one_before_max
        };

        let with_first_max_amount = collect_max_amount(&nums[0..nums.len() - 1]);
        let with_out_first_max_amount = collect_max_amount(&nums[1..nums.len()]);

        with_first_max_amount.max(with_out_first_max_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
        assert_eq!(Solution::rob(vec![6, 2, 3, 4, 6]), 10);
        assert_eq!(Solution::rob(vec![6, 6, 3, 4, 6]), 12);
        assert_eq!(Solution::rob(vec![6, 6, 3, 4, 6, 7, 8]), 20);

        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![2, 1]), 2);

        assert_eq!(Solution::rob(vec![]), 0);
    }
}
