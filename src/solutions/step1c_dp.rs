// Step1b_BFS
// 目的: 別の実装方針を練習する。動的計画法(Dynamic Programming)

/*
  問題の理解
  - 異なる金額のコインを表す整数からなる配列coninsと合計金額を表すamountが与えられる。
  合計金額amountになる最小のコインの枚数の組み合わせをcoins配列から探してコインの数を解として返す。
  合計金額になるようなコインの組み合わせを見つけられなければ-1を返す。
  coinsに含まれるコインの種類は種類ごとに何回使っても良い。
  coins=[1, 2, 5] amount=11 output=3 5+5+1
  coins=[2] amount=3 output = -1
  coins=[1] amount=0 output = 0

  何を考えて解いていたか
  - 時間切れなのでDP解法を写経する。
  https://www.youtube.com/watch?v=H9bfqozjoqs

  解答の理解
  - 322-dp-memo.pngにまとめた。

  正解してから気づいたこと
  - DP解法においてcoinsに負のコインが含まれるケースに対応できないことに気付くのに時間がかかった。GPT-5.1に聞いて対応できないことは分かったが理由は難しくてよく分からなかった。
  動的計画法は過去の状態に依存して現在の状態を決定する。負のコインが含まれると過去の状態を変更してしまうので、前提が崩れて計算がおかしくなるという理解をした。
  - LeetCode採点システム上での相対的な実行速度がstep1b_bfs.rsよりも10倍ほど速い。
  個人的にはBFS実装の方が何をしているのか分かりやすいので、大きなプログラムの内この部分の処理が占める割合がそこまで大きくなければBFS実装をしたくなる気持ちはある。
  このコードを読むであろうチームの人によって変えるかなと思った。
  自分は今まで ms 単位でパフォーマンスを追求するようなコードを要求されたことが無い。このような環境ではライブラリの背後に隠蔽されないコードではBFSで実装するななどと思った。
  実行時間が65ms -> 6ms になることを説明して、このコードを読ませるのは憚られるという感覚。

  n = coins.len()
  m = amount
  時間計算量: O(n * m)
  空間計算量: O(n)
    - step1b_bfs.rsの実装とBig-O記法による時間計算量は全く同じだが、実際の実行速度には相対的に見て10倍前後の有意な差がある。
    Big-O記法での時間計算量には現れない定数倍の差が明確に出る良い例だと思った。
      step1b_bfs.rs実装
        - HashSetによるハッシュ化、挿入、探索をO(n * m)回行っている。
        - 可変長配列(VecDeque)への挿入、取り出しをO(n * m)回行っている。
      step1c_dp.rs実装
        - 配列への添字アクセスや加算、減算、比較、代入といった比較的軽量な操作のみ行っている。
*/

pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }
        if coins.iter().any(|v| *v < 0) {
            panic!("coins must be contain only positive values")
        }

        let amount = amount as usize;
        let mut coin_counts = vec![amount + 1; amount + 1];
        coin_counts[0] = 0;

        for current_amount in 1..amount + 1 {
            for coin in &coins {
                let Some(remaining_amount) = current_amount.checked_sub(*coin as usize) else {
                    continue;
                };

                coin_counts[current_amount] =
                    coin_counts[current_amount].min(1 + coin_counts[remaining_amount as usize]);
            }
        }

        if coin_counts[amount] == amount + 1 {
            return -1;
        }

        coin_counts[amount] as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1c_dp_test() {
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
    fn step1c_dp_panic_test() {
        let coins = vec![1, 2, 3, -6];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }
}
