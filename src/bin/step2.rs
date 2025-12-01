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
  -　今回は特に思うところはなかった。

  改善する時に考えたこと
  - max_profitよりも通算利益っぽいtotal_profitの方が適切だと感じた
  - 前回の問題のレビューで提案を受けたsplit_firstによる分割で書いてみた。
  https://github.com/t9a-dev/LeetCode_arai60/pull/37#discussion_r2575562337
  remaining（残りの）という語彙も増えたし、すっきり書けていて良いと思う。
  - price - holding_priceはprofit変数に入れた方が丁寧な気がしたが、一度しか使わないのとtotal_profitに加算していることから過剰かなと思って止めた。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let Some((mut holding_price, remaining_prices)) = prices.split_first() else {
            return 0;
        };
        let mut total_profit = 0;

        for price in remaining_prices {
            if price < holding_price {
                holding_price = price;
                continue;
            }

            total_profit += price - holding_price;
            holding_price = price;
        }

        total_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);

        assert_eq!(Solution::max_profit(vec![2, 2, 4, 8]), 6);
        assert_eq!(Solution::max_profit(vec![2, 2, 4, 1]), 2);

        assert_eq!(Solution::max_profit(vec![1, 5]), 4);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
