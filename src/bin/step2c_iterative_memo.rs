// Step2c_iterative_memo.rs
// 目的: 動的計画法の練習をステップに分けて行う。反復処理+メモ化アプローチ(ボトムアップ)
// https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/

/*
  問題の理解
  - 自然数からなる配列numsを与えられる。（各家屋に置いてある金額を表している）
  家屋から金を強盗するとき、金額が最大になるように家屋を選び、最大の金額を返す。
  制約として隣接する家屋からは金を強盗できない。

  メモ化について
  - iが重複する箇所について計算結果をメモリ上に保存しておいてそれらを参照するようにする。

  計算量の見積もり
    n = nums.len()
    時間計算量: O(n) numsを一度全走査する
    空間計算量: O(n) メモで計算結果を全て保持している

  考え方
  - 今の家から金を盗む時は、前の前の家からも盗める
    - nums[i] + nums[i - 2]
  - 今の家から盗まないときは、1つ前の家から盗める
    - nums[i - 1]
  - それぞれの選択から得られる利益のうち大きい方を取っておく。

  所感
  - 空間計算量はO(n)となるものの、step1で写経した解答(空間計算量O(1))よりは変数命名に悩まなくて良いのですんなりと書けるなと感じた。
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }

        let mut max_amounts = vec![nums[0], nums[1].max(nums[0])];

        for i in 2..nums.len() {
            let max_amount = max_amounts[i - 1].max(max_amounts[i - 2] + nums[i]);
            max_amounts.push(max_amount);
        }

        max_amounts.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_recursive_memo_test() {
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
    fn step2b_recursive_memo_panic_test() {
        Solution::rob(vec![]);
    }
}
