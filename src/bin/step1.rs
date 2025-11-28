// Step1
// 目的: 方法を思いつく

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

  何がわからなかったか
  - 家が円形だとnums[0]とnums[nums.len() - 1]が隣接しているのでこのケースをどうやって検知して、除外すればよいか分からなかった。

  何を考えて解いていたか
  - まず問題の制約内で考えられるエッジケースを含んだテストケースを考える。
  - nums.len()==3のときある家から盗むと他のすべての家は隣接しているので最大値を返せば良い
  - 直前の問題と同じ解き方をすると何が問題になるか。
    - nums[i - 1].max(nums[i] + nums[i - 2])
    累積和の情報しか引き継がないので、その累積和に含まれる金がもともとどの家にあったのかの情報が失われている。
  時間切れなので解答を見る。

  解法の理解
  - 他の人の解答を見て理解する
    https://github.com/nanae772/leetcode-arai60/pull/35/files
    https://github.com/h1rosaka/arai60/pull/38/files
    https://github.com/docto-rin/leetcode/pull/41/files
  以下の考え方で解けると理解した。
    - 最初の家nums[0]から探索するときは最後の家nums[nums.len() - 1]は探索対象から外す。
    - 最初の家nums[0]を探索しないときは最後の家nums[nums.len() - 1]は探索対象に含める。
  実装してみる。

  正解してから気づいたこと
  - 解法を見ると問題をいかにシンプルに捉えられるかがポイントだったと思った。
  解法をみて自然に思いついた実装以外（空間計算量O(1)）も練習のために実装する。step1a.rs
*/

use house_robber::Robber;

/*
  Solution::rob内でRobber::robとできないようスコープ切るためにmodを利用している。
*/
mod house_robber {
    use std::collections::HashMap;
    pub struct Robber {}

    impl Robber {
        pub fn collect_max_amount(targets: &[i32]) -> i32 {
            let mut amount_cache: HashMap<isize, i32> = HashMap::new();
            Self::rob((targets.len() - 1) as isize, targets, &mut amount_cache)
        }

        fn rob(i: isize, targets: &[i32], amount_cache: &mut HashMap<isize, i32>) -> i32 {
            if i < 0 {
                return 0;
            }

            if let Some(amount_cache) = amount_cache.get(&i) {
                return *amount_cache;
            }

            let amount = Self::rob(i - 1, targets, amount_cache)
                .max(Self::rob(i - 2, targets, amount_cache) + targets[i as usize]);
            amount_cache.insert(i, amount);
            amount
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        match nums.len() {
            1 => return nums[0],
            2 => return nums[0].max(nums[1]),
            3 => return nums[0].max(nums[1]).max(nums[2]),
            _ => (),
        };

        let with_first_targets = &nums[0..nums.len() - 1];
        let with_out_first_targets = &nums[1..nums.len()];

        Robber::collect_max_amount(with_first_targets)
            .max(Robber::collect_max_amount(with_out_first_targets))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
