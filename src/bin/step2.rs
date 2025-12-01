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
  - 動的計画法の定義について。あまり深く考えたことはなかったが個人的にはそこまで重要では無いかなという感想。
  https://github.com/olsen-blue/Arai60/pull/37/files#r2030022958

  - 自分も初期値でprices[0]を使うなら&prices[1..]からループを回した方が違和感を感じないと思った。
  ループ回数が減ってパフォーマンスが〜ということではなくて、直前でprices[0]を使っているのでこれを飛ばす方が自然だという感覚。
  https://github.com/docto-rin/leetcode/pull/42#discussion_r2471529109

  - min_priceの更新を常に行うという方針。ifの条件を間違える可能性を排除できるので常にminでやるのもありだと思った。
  https://github.com/docto-rin/leetcode/pull/42#discussion_r2471449749

  - 最初に思いつく動的計画法の解法として、一次元DPの方が自然な気がするので練習のために実装する。step2a.rs
  https://github.com/olsen-blue/Arai60/pull/37/files#diff-0474f0ee7711182f0e97bb4047531dc4c65356748eafab139512400ac88c5c0bR10

  改善する時に考えたこと
  - prices.into_iter().skip(1)の部分はfor loopでは&prices[1..]の方が自然かも
  prices.into_iter().skip(1).map(|x| ...) のようにするならメソッドチェーンで書けるのでskip()で良いかなという感覚。
  - min_priceの更新ifで判定せずに常に最小を更新していくほうがシンプルになる
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = match prices.len() {
            0 => return 0,
            _ => prices[0],
        };
        let mut max_price = 0;

        for price in &prices[1..] {
            min_price = min_price.min(*price);
            max_price = max_price.max(*price - min_price);
        }

        max_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
