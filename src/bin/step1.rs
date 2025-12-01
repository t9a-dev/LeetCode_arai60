// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数配列pricesが与えられる。prices[i]は株価を表している。株の売買を行って得られる最大利益を解として返す。
  利益が発生しなければ0を返す。売買するかを決められるので損失がでるような売買は行わない。
  制約
    - 一度に保有できる株式は最大1株のみ
    - 同じ日に複数回の売買は可能

  何がわからなかったか
  - 特になし

  何を考えて解いていたか
  - prices=[7, 1, 5, 3, 6, 4]の時
    - day=1の時買い、day=2の時売る。 profit=5-1=4
    - day=3の時買い、day=4の時売る。 profit=6-3=3
    max_profit=4+3=7
  - prices=[1, 2, 3, 4, 5]の時
    - day=0の時買い、day=4の時売る。 profit=5-1=4
    max_profit=4
  - prices=[7, 6, 4, 3, 2, 1]の時
    - どの時点で買っても利益が発生しないので、max_profit=0

  - prices=[1,2,3,4,5]のときはいきなり5-1で答えを求めなくても、5-4-3-2-1と答えを求めていけば大丈夫そう。
  今持っている株prices[i]よりも安い株を見つけたら、
    - 売買候補株を更新する 売買はスキップ
  今持っている株よりも高い株を見つけたら
    - 利益を計算して通算利益を更新
    - 今持っている株を購入した株(prices[i + 1])で更新する。

  Leet Code採点システムは一発でAcceptedになった。
  しかし、手元のテストケースを通らないコード（比較の方向を間違えた）を一度書いてしまったので、ホワイトボードでのコーディングだとアウトだなと思った。

  正解してから気づいたこと
  - 売買候補株を保持しておく変数名がもう少し適切なものがありそう。min_priceはpricesの中の最小を常に指すわけではないので少し忌避感がある。
  - 前回の問題のレビューで提案のあったsplit_firstによる分割を行うように改善する。
  - 再帰による実装も練習のためにしておくと良さそう。前回の問題(121. Best Time to Buy and Sell Stock)では入力のサイズが大きく、スタック深さがそのまま入力のサイズとなるので忌避感があった。
  実装前に制約上の入力サイズでスタックサイズ上限が7MBと仮定した場合に1スタックフレームあたりに詰めるサイズを見積もってみる。
  入力の制約 prices.length <= 3 * (10 ^ 4) = 30_000
  スタックサイズ上限7MBとすると、7 * 1024 * 1024 / 30_000 = 約244Byte となり、1スタックフレームあたり244Byteまでならスタックオーバーフローとならない。
  余裕そうなのでstep1a.rsで再帰による実装を練習する。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let Some(mut candidate_stock) = prices.get(0) else {
            return 0;
        };
        let mut max_profit = 0;

        for price in &prices[1..] {
            if price < candidate_stock {
                candidate_stock = price;
                continue;
            }

            max_profit += price - candidate_stock;
            candidate_stock = price;
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
