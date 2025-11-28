// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  - Vecのような可変長配列の一部分を別のところへ渡す時にコピーが行われないかはだいぶ気にするようになってきた。
  Pythonだとスライスでコピーが発生する仕様らしいので覚えておく必要があると思った。
  https://github.com/Kaichi-Irie/leetcode-python/pull/6#discussion_r2171435011
  同じ指摘を見つけたのでPythonにおけるスライスの暗黙的なコピーはよくある落とし穴っぽいと思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/49#discussion_r2217263665

  改善する時に考えたこと
  - 重複しているループのコードをまとめる。
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
                one_before_max = current_max;
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
    fn step2_test() {
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
