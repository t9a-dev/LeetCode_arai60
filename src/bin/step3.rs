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
  1回目: 2分29秒
  2回目: 2分20秒
  3回目: 2分41秒
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        let (mut two_previous_robbed_money, mut previous_robbed_money) = match nums.len() {
            1 => return nums[0],
            _ => (nums[0], nums[0].max(nums[1])),
        };

        for i in 2..nums.len() {
            let robbed_money = nums[i] + two_previous_robbed_money;
            two_previous_robbed_money = previous_robbed_money;
            previous_robbed_money = previous_robbed_money.max(robbed_money);
        }

        previous_robbed_money
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
        assert_eq!(Solution::rob(vec![4, 0, 1]), 5);
        assert_eq!(Solution::rob(vec![4, 0]), 4);
        assert_eq!(Solution::rob(vec![2, 3]), 3);
        assert_eq!(Solution::rob(vec![2]), 2);
        assert_eq!(Solution::rob(vec![0]), 0);
    }

    #[test]
    #[should_panic]
    fn step3_panic_test() {
        Solution::rob(vec![]);
    }
}
