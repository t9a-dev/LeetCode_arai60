// Step1a
// 目的: 練習のために再帰による実装を行う

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

  再帰関数の設計
  - 基本ケース
    - 売買対象がなければ0を返す。
  - 再帰ケース
    - 持っている株よりも売買候補の株が安ければ、所持する株を安い株に更新して日付を一日進めて再帰に入る。
    - 持っている株を売って利益を計算する。次の日付の売買を行うために再帰に入る。
    - 計算した利益を全て合算して返す。

  step1で見積もった空間計算量（スタックオーバーフローしないか）
    入力の制約 prices.length <= 3 * (10 ^ 4) = 30_000
    スタックサイズ上限7MBとすると、7 * 1024 * 1024 / 30_000 = 約244Byte となり、1スタックフレームあたり244Byteまでならスタックオーバーフローとならない。
    余裕そうなのでstep1a.rsで再帰による実装を練習する。
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let Some((initial_holding_price, remaining_prices)) = prices.split_first() else {
            return 0;
        };

        Self::make_max_profit(remaining_prices, 0, *initial_holding_price)
    }

    fn make_max_profit(prices: &[i32], day: usize, holding_price: i32) -> i32 {
        let Some(price) = prices.get(day) else {
            return 0;
        };

        if *price < holding_price {
            return Self::make_max_profit(prices, day + 1, *price);
        }

        let current_profit = *price - holding_price;
        let future_profits = Self::make_max_profit(prices, day + 1, *price);

        current_profit + future_profits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
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
