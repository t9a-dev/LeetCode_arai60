// Step1
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

  何がわからなかったか
  - いわゆる動的計画法での解き方が分からず、思いついた貪欲法でコードを書いたが貪欲法では解けない入力ケースまで思いつかなかった。

  何を考えて解いていたか
  - amount=0 のとき output=0 となるのがDP問題における初期状態のを表すような感じがする。合計金額が0となるコインの枚数は0なので納得できる。
    - amount=0のとき0を返す。coins.isempty()で-1を返す。
  - amoount - coins[i] の補数を求める操作が必要そう。
    - 最小のコインを使うので、coinsの大きい金額を使って補数を求める。
    - coinsをソートしたい。coinsの種類は制約上 coins.len() <= 12 なので計算量は問題なさそう。
    - 補数を求めるたびにcoin_change_countをインクリメントする。
      - coins[i] < amount　である場合にのみ補数を求める whileでこの条件で減算し続ける
      - coins[i] == amount となったら、その時点のcoin_change_countで早期リターン
    - メソッド終了地点まで到達したら -1
  - この考え方でナイーブな実装をしてWrong Answerとなった。
  coins=[186, 419, 83, 408] amount=6249 expect=20 output=-1
  よく分からないので、自分の実装方法でこのテストケースだけ手作業で解いてみる。
  coinsを降順ソートする。coins=[419, 408, 186, 83]
  6249 - (419 * 14) = 383
  383 - (186 * 2) = 11 <- ここで11より小さいコインを持たないので-1となる。

  5分だけDP的な解き方の方向で考えてみる。
  - 前提としてなるべく交換するコインを少なくしたいので、より大きい金額のコインで優先的に交換したいという気持ちがある。
  これ以上何も思い浮かばないので答えを見る。
  NeetCode解説動画:
  https://www.youtube.com/watch?v=H9bfqozjoqs
  自分がナイーブな実装と読んでいたのは貪欲法というアプローチであり、この問題は解けないという説明がされていた。
  とりあえず再帰＋メモ化のアプローチを考える。
  知りたいのはコインの組み合わせ枚数のうち最小のコイン数
  再帰ケースに入るたびに使ったコインの数を+1する必要がある。
  補数が値が使えるコイン金額よりも小さくなったらNoneを返す。
  補数が0になったら、Some(使ったコインの数)とする。
  コインの組み合わせの中から最小の組み合わせのコイン数をiter().flatten()でSome(x)だけを取り出して最小を求める。
  .unwrap_or(-1)としておいて全部None（コインの組み合わせが存在しない）のときは-1を返す。
  入力制約は
    - 1 <= coins.len() <= 12
    - 1 <= coins[i] <= (2 ^ 31) - 1
    - 0 <= amount <= 10 ^ 4
  amount - coins[i]をするので、i32だと桁溢れするかも？と思ったが、amountが0以下になったらそれ以上減算しなければ大丈夫そう。
  再帰深さはどうなるだろうか。
  coins[1] amount=100 のようなケースで考えると、100回減算する。最悪ケースで(10 ^ 4)になる。キャッシュが効かない最大深さは10_000なので問題なさそう。
  coins種類は最大で12となり、メモ化すればO(coins.len())になるという感覚。
  メモ化は何をキャッシュすればよいのか正直ピンときていない。コードを書けば分かるかも知れない。
  メモ化を仕様として動かないコードが出来上がった。解法を理解できていないので、step1a.rsで書き直す。

  再帰の設計
    基本ケース
    - if complement == coins[i] return Some(coin_change_count)
    再帰ケース
    - complement -= coins[i] 、coin_change_count+1して再帰に入る。
      - complementが0未満であれば、　return Noneとして再帰に入らないようにする。

  想定ユースケース
  - 使えるコインの枚数を表現して全てcoins配列に入れる。使ったコインを消費すれば自販機のお釣り計算とかに使えそうだと思った。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn coin_change(conins: Vec<i32>, amount: i32) -> i32 {
        /*
          このコードは動きません。
        */
        let mut amount_to_change_count: HashMap<i32, Option<i32>> = HashMap::new();
        Self::change_coin(&conins, amount, 0, &mut amount_to_change_count).unwrap_or(-1)
    }

    fn change_coin(
        coins: &[i32],
        amount: i32,
        change_count: i32,
        amount_to_change_count: &mut HashMap<i32, Option<i32>>,
    ) -> Option<i32> {
        /*
          このコードは動きません。
        */
        if amount == 0 {
            return Some(change_count);
        }

        if let Some(change_count_cache) = amount_to_change_count.get(&amount) {
            return *change_count_cache;
        }

        let change_count = coins
            .iter()
            .map(|coin| {
                let Some(remaining_amount) = amount.checked_sub(*coin) else {
                    return None;
                };

                if remaining_amount < 0 {
                    return None;
                }

                if remaining_amount == 0 {
                    return Some(change_count + 1);
                }

                Self::change_coin(
                    coins,
                    remaining_amount,
                    change_count + 1,
                    amount_to_change_count,
                )
            })
            .into_iter()
            .flatten()
            .min();
        amount_to_change_count.insert(amount, change_count);

        change_count
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn playground() {
        assert!(i32::MIN.checked_sub(i32::MAX).is_none());

        assert_eq!(
            vec![None, Some(3)]
                .into_iter()
                .flatten()
                .min()
                .unwrap_or(-1),
            3
        );

        assert_eq!(
            vec![None, None].into_iter().flatten().min().unwrap_or(-1),
            -1
        );
    }
    #[test]
    fn step1_test() {
        assert_eq!(Solution::coin_change(vec![], 0), 0)

        // let coins = vec![1, 2, 5];
        // let amount = 11;
        // assert_eq!(Solution::coin_change(coins, amount), 3);

        // let coins = vec![2];
        // let amount = 3;
        // assert_eq!(Solution::coin_change(coins, amount), -1);

        // let coins = vec![1];
        // let amount = 0;
        // assert_eq!(Solution::coin_change(coins, amount), 0);

        // let coins = vec![1, 2, 3, 4, 5];
        // let amount = 7;
        // assert_eq!(Solution::coin_change(coins, amount), 2);

        // coins=[186, 419, 83, 408] amount=6249
        // let coins = vec![186, 419, 83, 408];
        // let amount = 6249;
        // assert_eq!(Solution::coin_change(coins, amount), 20);
    }
}
