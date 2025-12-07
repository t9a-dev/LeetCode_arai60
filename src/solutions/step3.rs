// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = coins.len()
  m = amount
  時間計算量: O(n * m)
  空間計算量: O(n)
*/

/*
  1回目: 6分53秒
  2回目: 5分47秒
  3回目: 5分16秒
*/

/*
  所感
  - 決定木として考えてcoinsのコインを使ってコインタワー作るイメージが一番しっくり来た。高さが0（コインなし）からamountの高さになるコインの組み合わせを探すという感じ。
*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    const COMBINATION_NOT_FOUND: i32 = -1;
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        if amount < 0 {
            return -1;
        }
        if coins.iter().any(|coin| *coin < 0) {
            panic!("coins must be contains only positive values");
        }

        let mut coins = coins
            .into_iter()
            .filter(|coin| *coin <= amount)
            .collect::<Vec<_>>();
        coins.sort();
        coins.reverse();

        let mut visited_amount = vec![false; (amount + 1) as usize];
        let mut frontiers = VecDeque::new();
        frontiers.push_back((0, 0));

        while let Some((previous_amount, change_count)) = frontiers.pop_front() {
            for coin in &coins {
                let current_amount = previous_amount + *coin;

                if current_amount == amount {
                    return change_count + 1;
                }
                if amount < current_amount {
                    continue;
                }
                if visited_amount
                    .get(current_amount as usize)
                    .is_some_and(|is_visited| *is_visited)
                {
                    continue;
                }
                visited_amount[current_amount as usize] = true;

                frontiers.push_back((current_amount, change_count + 1));
            }
        }

        Self::COMBINATION_NOT_FOUND
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
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

        let coins = vec![];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);

        let coins = vec![];
        let amount = 5;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }

    #[test]
    fn step3_minus_amount_test() {
        let coins = vec![1, 2, 3, 5];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }

    #[test]
    #[should_panic]
    fn step3_have_minus_coin_test() {
        let coins = vec![1, 2, 3, -6];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }
}
