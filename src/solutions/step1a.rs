// Step1a
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

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
  NeetCodeの解説動画
  https://www.youtube.com/watch?v=H9bfqozjoqs
  - いきなりDPの解法は飛躍し過ぎだと感じてしまい理解できないので、決定木として問題を見て、再帰＋メモ化の実装方法を行う。
  - 決定木の考え方
    - amount に対して 全てのコイン種類 coins[i] との差 remaining_amount を求める。
      - 求めた差 remaining_amount に対して　全てのコイン種類 coins[i] とその差を求める。
      - ここで remaining_amount が 0未満になるまで再帰的に続く。
      - remaining_amount == 0 となったとき、それまでに使ったコインの数を解として知りたい。
    - 得られたコイン数の内、最小のもののみを取っておく。
  - 使ったコインの数え方をどうしよう。コインの使用枚数は単調増加する。
  - とりあえず再帰呼び出しするたびに使ったコインの枚数を+1する
    - amountが0になったときにそのパスで使ったコインの枚数を使いたい。
      - Some(0) を再帰の基本ケースで返す。
    - amountが0未満になるようなパスのコインの枚数は捨てたい。
      - None を返す。

  再帰の設計
  - 基本ケース
    - if amount == 0 return Some(0)
  - 再帰ケース
    - 再帰の戻り値がNoneであれば値を捨てる
    - 再帰の戻り値がSomeであれば値を足して、最小枚数を更新していく。

  ここまで考えて、実装の途中で手が止まったのでGPT-5.1のコードを写経した。
  分からなかったこと
  - amount == 0 となった時にSome(0)とするのが思いつかなかったのと、これを見た時にすぐに理解できなかった。つまり再帰の基本ケースが分からなかった。

  写経した後の解法の理解
  - 深さ優先探索(DFS)になっている。amount - coins[i] が 0 以下になるまで潜っていくイメージ
  - 0 < amount - coins[i] であれば、amountはまだcoins[i]と交換できる余地があるので再帰に入っている。
  - 部分問題で考えると、交換できたコインの枚数というよりは手持ちのamountでcoins[i]で交換できたかという考え方の方が自然だと思った。

  正解してから気づいたこと
  - 決定木の考え方で解くなら、幅優先探索でamount == 0を見つけた時点で探索を打ち切るような実装の方がしっくりくる感じがする。
  決定木から底に向かうに途中で答えを見つけたらそれ以上は探索しなくて良いので。 step1b_bfs.rsで実装してみる。

*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amount_to_change_count: HashMap<i32, Option<i32>> = HashMap::new();
        Self::calculate_change_count(&coins, amount, &mut amount_to_change_count).unwrap_or(-1)
    }

    fn calculate_change_count(
        coins: &[i32],
        amount: i32,
        amount_to_change_count: &mut HashMap<i32, Option<i32>>,
    ) -> Option<i32> {
        if amount == 0 {
            return Some(0);
        }
        if amount < 0 {
            return None;
        }
        if let Some(change_count_cache) = amount_to_change_count.get(&amount) {
            return *change_count_cache;
        }

        let mut min_change_count = None;
        for coin in coins {
            let Some(change_count) =
                Self::calculate_change_count(coins, amount - *coin, amount_to_change_count)
            else {
                continue;
            };

            let candidate = change_count + 1;

            min_change_count = match min_change_count {
                None => Some(candidate),
                Some(current) => Some(current.min(candidate)),
            }
        }
        amount_to_change_count.insert(amount, min_change_count);

        min_change_count
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1a_test() {
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
    }
}
