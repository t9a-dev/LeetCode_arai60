// Step1a
// 目的: 解答の解法を理解したことを確認する
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/solutions/4868897/most-optimized-kadanes-algorithm-java-c-2yt85/

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 株価を表す配列pricesが与えられる。prices[i]はある日の株価である。
  - ある日に株を買った場合に、最大になる利益を値として返す。
  - 利益が発生しない場合は0を返す。
  e.g.
  prices = [1, 2, 3, 4] out = (prices[3] - prices[0]) = 3
  prices = [4, 3, 2, 1] out = 0 // どの日に株を購入しても利益が発生しない

  何を考えて解いていたか
  - 先頭から全走査しながら
    - 最小株価を見つけるたびに最小株価を更新する
    - 今見ている株価prices[i]と最小株価から利益を求めて最大利益を更新する

  正解してから気づいたこと
  - 解法を理解した状態からコードにするまでは距離を感じない。
  - 問題から解法にたどり着くまでに距離を感じる。
  - この解答は最適解だと思うので、前回の問題(House Robber)でやったように再帰処理+memoといった別パターン実装を練習した方が良いと思った。step1b.rsで実装する。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = match prices.len() {
            0 => return 0,
            _ => prices[0],
        };
        let mut max_price = 0;

        for price in prices.into_iter().skip(1) {
            if price < min_price {
                min_price = price;
                continue;
            }

            max_price = max_price.max(price - min_price);
        }

        max_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
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
