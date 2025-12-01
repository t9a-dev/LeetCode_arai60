// Step2a
// 目的: 一次元DPによる実装方法を練習しておく。

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  解法の考え方
  - pricesを先頭から全走査しつつ、最小の価格を更新する
  - 今見ている株価prices[i]と最小の価格差を計算してmax_profits配列に入れていく
  - 最後にmax_profits配列の中から最大の値を返す

  n = prices.len()
  時間計算量: O(n)
  空間計算量: O(n)

  max_profitsを配列で管理せず、変数で管理すると空間計算量がO(1)となる。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = match prices.len() {
            0 => return 0,
            _ => prices[0],
        };
        let mut max_profits = vec![0];

        for price in &prices[1..] {
            min_price = min_price.min(*price);
            max_profits.push(*price - min_price);
        }

        max_profits.into_iter().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 2, 5, 3, 6, 1]), 4);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_profit(vec![4, 3, 2, 1]), 0);

        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0); // 株を購入できないので利益は0で意味が通ると思う
    }
}
