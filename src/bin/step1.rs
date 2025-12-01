// Step1
// 目的: 方法を思いつく

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

  何がわからなかったか
  - 手作業でやることを考えたときに、
    - 最小株価を見つける。
    - 最大株価を見つける。
    - これらの株価の差を見る。ただし、最小株価の位置iは最大株価のiよりも小さい必要がある。
    この解法をより単純な考え方にする方法がわからなかった。

  何を考えて解いていたか
  - 2重ループで組み合わせを見れば解けると思う。ある時点の株価よりも高い株価の時の利益を都度更新していき、最終的に最大の利益を返す。
  計算量はO(N ^ 2)となる。入力の制約からNは10 ^ 5なので、10 000 000 000回計算することになる。筋が悪そう。もう少し良さそうな方法を考えて思いつかなければこのナイーブな実装をする。
  - pricesの中で最小を探す。indexをdayとする。線形探索なのでO(N)
    pricesの中で最大を探す。indexをdayとする。線形探索なのでO(N)
    min_price_day < max_price_day であれば max_price - min_price を答えとして返す。
    条件を満たさなければ0を返す。
    この方法だと利益自体は発生するケースを取りこぼす。
  時間切れなのでナイーブな実装だけする。
  Time Limit Exceeded となる実装が出来上がった。
  解答を見る。

  解法の理解
  https://leetcode.com/problems/best-time-to-buy-and-sell-stock/solutions/4868897/most-optimized-kadanes-algorithm-java-c-2yt85/
  - pricesを先頭から全走査しながら、buyに見つけたprices[i]の最小値を入れている。
  - prices[i] - buy で利益を求めて、利益が今までに見つけた利益よりも大きければ利益を更新している。
  解法は理解できので、step1a.rsで実装する。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /*
          このコードはLeetCode採点システム上で Time Limit Exceeded となるコードです。
        */
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;
        for i in 0..prices.len() {
            for j in i..prices.len() {
                if prices[j] < prices[i] {
                    continue;
                }

                max_profit = max_profit.max(prices[j] - prices[i]);
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
