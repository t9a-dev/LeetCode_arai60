// Step1a
// 目的: 別の解法を練習する。空間計算量がO(1)となる解法を実装する。

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 家毎に置いてある金額を表す配列numsが与えられる。家から盗める金額の最大値を答えとして返す。
  制約として隣接する家からは金を盗めない。
  家は円形に並んでいるのでnums[0],nums[nums.len() - 1]は隣接していることになる。

  解法について
  - 前回の問題(198.House Robber)のレビューで提案してもらった解法で解いてみた。
  存在しない家(nums[-2],nums[-1])の金を0として仮定している部分が考え方として番兵に近い気がする。
  https://github.com/t9a-dev/LeetCode_arai60/pull/35#discussion_r2564647801
  自分でこの解法を思いつくまでの距離は大分ある気がするものの、何をしているのかは理解できている。

  何を考えて解いていたか
  以下の二通りのケースでそれぞれの最大値を求めて、最後に大きい方を確認して返す。
  - 最初の家[0]を含み最後の家を除く場合(0..nums.len()-1)
  - 最初の家[0]を含まず最後の家を含む場合(1..nums.len())

  正解してから気づいたこと
  - 最大値を求めるループのコードが重複しているのでまとめられる。
  - テストコードで検知できたが、nums.len() == 1のケースをチェックしないと想定通り動かない。
  コーディング試験で行われるようなホワイトボード上でのコーディングだとテストコードによるテスト実行はできないので、配列の境界周りのロジックは大分注意が必要だと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };
        if nums.len() == 1 {
            return nums[0];
        }

        let mut two_before_max = 0;
        let mut one_before_max = 0;
        for i in 0..nums.len() - 1 {
            let current_max = one_before_max.max(two_before_max + nums[i]);
            two_before_max = one_before_max;
            one_before_max = current_max;
        }
        let with_first_max = one_before_max;

        two_before_max = 0;
        one_before_max = 0;
        for i in 1..nums.len() {
            let current_max = one_before_max.max(two_before_max + nums[i]);
            two_before_max = one_before_max;
            one_before_max = current_max;
        }
        let with_out_first_max = one_before_max;

        with_first_max.max(with_out_first_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
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
