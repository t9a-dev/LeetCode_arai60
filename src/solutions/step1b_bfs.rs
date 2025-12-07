// Step1b_BFS
// 目的: 別の実装方針を練習する。幅優先探査(BFS)

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
  - 幅優先探査では深い位置に向かうにつれて幅が広くなるので、ナイーブな実装をすると計算量が爆発するので、キャッシュを利用する必要がある。
  入力制約は
    - 1 <= coins.len() <= 12
    - 1 <= coins[i] <= (2 ^ 31) - 1
    - 0 <= amount <= 10 ^ 4
  深さについては、amount - coins[i] < 0 になるまで続くが、この場合の計算量の表し方がよく分からない。
  O(深さ ^ coins.len()) となるので計算量が爆発することだけは分かる。
  ナイーブな実装では今見ている深さを更新しながらamountが0になるものを発見したら、このときの深さを返せば良いという考え方になる。
  しかし、キャッシュを利用することを考えると、amountに対して深さの情報を紐づけるのは正しく動作しない。
  なぜならamountは単純にcoins[i]との差であるので、深さは関係ない。浅い位置で出現したamountがより深い位置で出現することはあり得る。
  なのでキャッシュを利用するのであれば、深さの情報をamountに紐づけるような実装はできない。
  ここまで考えたが手が止まってので、GPT-5.1に聞いて写経する。

  解答の理解
  - この解法は手持ちのコインを使ってなるべく少ない枚数でamountを作ろうという考え方だと理解した。
    テーブルの上に厚みの異なるコインを平積みしてコインタワーを作り、ぴったりamountの高さになるか。
    ある高さになった時、その高さを構成する枚数はどうでも良くて、その高さから目標の高さにするために与えられたコインの種類で到達できるかだけわかれば良い。
    なので一度見た高さはキャッシュしているが、枚数はキャッシュしていない。
    コインタワーの高さが同じであっても、同じ枚数（同じコイン種類）とは限らないので、高さとコイン枚数を紐づけてキャッシュするとおかしなことになると理解した。
    e.g. 厚み1のコイン10枚と厚み5のコイン2枚どちらもコインタワーの高さは10になる。しかし、使っているコインの枚数は違う。

  正解してから気づいたこと
  - 再帰＋メモ化の時に何をキャッシュすればよいのか分からず、ある時点のamountと枚数をキャッシュしていた。
  これが間違いであることを理解できず混乱していた。
  個人的には amount - coins[i]で0になるかを見るよりも、0からamountにぴったりなるcoins[i]の組み合わせを考えるほうが理解しやすかった。
  - 答えを消して実装できたので覚えることはできている。

  - 計算量の見積もり方がわからない。コインの種類 * (0~amountに到達するまでの計算回数) といった感覚だが0からamountまでの~という部分は計算するまでわからないのでは？という感覚。
  GPT-5.1に聞いた。
   - 上記の考え方で方向は合っていた。
     - amount_cacheで0~amountの値は一度しか訪れない。つまりamount - 0回計算を実行することになるので、m = amountとなる。
  n = coins.len()
  m = amount
  時間計算量: O(n * m)
  空間計算量: O(n)

  step1c_dp.rsで模範解答写経してstep2へ進む
*/

use std::collections::{HashSet, VecDeque};

pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut frontiers = VecDeque::new();
        let mut amount_cache = HashSet::new();

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
                if !amount_cache.insert(current_amount) {
                    continue;
                }

                frontiers.push_back((current_amount, change_count + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1b_bfs_test() {
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
    fn step1b_bfs_minus_amount_test() {
        let coins = vec![1, 2, 3, 5];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }

    #[test]
    fn step1b_bfs_have_minus_coin_test() {
        let coins = vec![1, 2, 3, -6];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }
}
