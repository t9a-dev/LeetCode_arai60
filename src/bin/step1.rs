// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 自然数からなる配列numsを与えられる。（各家屋に置いてある金額を表している）
  家屋から金を強盗するとき、金額が最大になるように家屋を選び、最大の金額を返す。
  制約として隣接する家屋からは金を強盗できない。

  何がわからなかったか
  - 動的計画法による解法。隣接した組み合わせを選ばないようにというところをどのようにコードに落とし込めばよいか分からなかっった。

  何を考えて解いていたか
  - ワンパスで解けそう。配列を先頭から見ていき偶数、奇数の場合の金額の累積和を保持しておき最後に大きい方を返す。
  ↑この方針でWrong Answerとなった。nums=[2, 1, 1, 2]の時に[2,2]のペアは隣接していないので。
  時間切れなので解答を見る

  解法の理解
  - 今の金額nums[i]と前の前の金額nums[i - 2]の合計値を求める。制約により隣接する家屋から金を盗めないので一つ飛ばしで行っている
  - 前の金額nums[i - 1]を前の前の金額nums[i - 2]とする
  - 前の金額と今の金額のうち大きい方を取っておく。言い換えると最大となる金額以外はいらないので上書きして捨てている。

  正解してから気づいたこと
  - 53. Maximum Subarrayの応用問題だと思った。この問題では制約により隣接する部分列ではなく、隣接しない値のペアを見ている。
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        };

        if nums.len() == 1 {
            return nums[0];
        }

        let mut previous_previous_robbed = nums[0];
        let mut previous_robbed = nums[0].max(nums[1]);

        for i in 2..nums.len() {
            let robbed = previous_previous_robbed + nums[i];
            previous_previous_robbed = previous_robbed;
            previous_robbed = previous_robbed.max(robbed);
        }

        previous_robbed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
    fn step1_panic_test() {
        Solution::rob(vec![]);
    }
}
