// Step2a_dp
// 目的: step1c_dp.rs解法のリファクタリング

/*
  改善する時に考えたこと
  - step1c_dp.rsの実装を改善する。
    - coinsを昇順ソートにして小さいコインから見る。
      - 今見ているamountよりも大きいコインを見つけたらcoins loopをbreakする。
        - ただし、この方法はcoinsが昇順ソートされていて、今見ているよりamountよりも大きなコインを見つけたら探索を終了するということを理解している前提でないと読み手に負荷を与える。
        https://github.com/nanae772/leetcode-arai60/pull/39/files#r2442801687
        人が読むことを考えると、この最適化はしないほうがエンジニアリングの観点から良いと考えた。
    - coinsの内amountを超えるようなコインを取り除く

  所感
  - LeetCode採点システム上では変更前後(step1c_dp.rs -> step2a_dp.rs)で実行速度に有意な差は見られなかった。
  - coins.len()は1~12と値が小さいので取り除いたところで大きな差が出ないと理解した。
  - それぞれの実装でベンチマークを取ってみる。
*/

pub struct Solution {}
impl Solution {
    const COMBINATION_NOT_FOUND: i32 = -1;

    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return Self::COMBINATION_NOT_FOUND;
        }
        if amount == 0 {
            return 0;
        };
        if coins.iter().any(|coin| *coin < 0) {
            panic!("coins must be contains only positive values");
        }

        let mut coins = coins;
        coins = coins
            .into_iter()
            .filter(|coin| *coin <= amount)
            .collect::<Vec<_>>();

        let sentinel_coin_count = amount + 1;
        let amount = amount as usize;
        let mut amount_to_coin_count = vec![sentinel_coin_count; amount + 1];
        amount_to_coin_count[0] = 0;

        for current_amount in 1..amount + 1 {
            for coin in &coins {
                let Some(remaining_amount) = current_amount.checked_sub(*coin as usize) else {
                    continue;
                };

                amount_to_coin_count[current_amount] = amount_to_coin_count[current_amount]
                    .min(1 + amount_to_coin_count[remaining_amount]);
            }
        }

        if amount_to_coin_count[amount] == sentinel_coin_count {
            return Self::COMBINATION_NOT_FOUND;
        }

        amount_to_coin_count[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_dp_test() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(Solution::coin_change(coins, amount), 3);

        let coins = vec![2];
        let amount = 3;
        assert_eq!(Solution::coin_change(coins, amount), -1);

        let coins = vec![1];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);

        let coins = vec![1, 2, 3, 4, 5];
        let amount = 7;
        assert_eq!(Solution::coin_change(coins, amount), 2);

        let coins = vec![186, 419, 83, 408];
        let amount = 6249;
        assert_eq!(Solution::coin_change(coins, amount), 20);

        let coins = vec![186, 419, 83, 408];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);

        let coins = vec![1, 2, 3, 5];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), -1);

        let coins = vec![];
        let amount = 5;
        assert_eq!(Solution::coin_change(coins, amount), -1);

        let coins = vec![];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);
    }

    #[test]
    #[should_panic]
    fn step2a_dp_panic_test() {
        let coins = vec![1, 2, 3, -6];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }
}
