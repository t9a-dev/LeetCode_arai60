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
  - コメント集
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0#heading=h.ic8466had15a
    - 「配るDP」について。
    > メモ探索ではできない
    自分が詰まったところを表しているのかなと思った。具体的には、メモ化しようとしてamountとコイン枚数を紐づけてキャッシュしようとしたが、この方法は間違っており、メモ化できなかった。

  - ダイクストラ法というのもこの問題に適用できるらしい。
  https://discord.com/channels/1084280443945353267/1403227402984755271/1408287137559744624

  - コインの組み合わせが見つからなかったときの戻り値 -1 を定数にしたいというレビュー
  確かにしない方がいい理由は思い浮かばないので、定数として-1を入れて置くのが良さそう。
  https://github.com/h1rosaka/arai60/pull/42#discussion_r2384901626

  - BFSの方が早くなる実装があるらしい。なんとなく自分の中でより分かりにくいコードの方が実行速度は速いみたいな感覚があったので驚いた。良くない思考の癖だと思った。
  https://github.com/nanae772/leetcode-arai60/pull/39/files#diff-762c75d9f29079e87b6e0c8549c01ba137eb3b4a426da323f08df2a1d7f09ac1R82
    - とりあえず読んでみて速くなっている要因となっていそうな箇所
      - 訪問済みのamountをHashSetではなく、配列で管理している。ハッシュ化のコストを無くせる。
      - coinsの内amountより大きいものを取り除いてループの総回数を減らしている
      - coinsを降順ソートしている。なるべく少ないコインの組み合わせが知りたいので、より大きいコインから見ていった方が効率が良いくらいの理解。
        - coins=[1,50] amount=100 のようなケースを考えると分かりやすい。
      - コードのコメントに「最短経路問題としてみなして~」とある。
      https://github.com/nanae772/leetcode-arai60/pull/39/files#diff-3b6f683e2abf409ce92edb1fa882377eca64fff579d04113b07c4a039de2748dR28
        - 今回の問題ではコインの組み合わせの内「最小」となるコイン枚数を求めるので最短経路問題として考えられると理解した。
        https://ja.wikipedia.org/wiki/%E6%9C%80%E7%9F%AD%E7%B5%8C%E8%B7%AF%E5%95%8F%E9%A1%8C

  改善する時に考えたこと
  - step1b_bfs.rsの実装を改善する。
    - visitedをHashSetではなく、配列で行う。
    - coinsを降順ソートにして大きいコインから見る。
    - coinsの内amountを超えるようなコインを取り除く

  所感
  - step1b_bfs.rsと同じ実装方針で定数倍に影響する処理を調整したところ、有意に処理速度が向上した。
  具体的にはLeetCode採点システムで 60ms前後 -> 6ms前後 になったので10倍程度改善したことになる。
  - DP解法(step1c_dp.rs)でもcoinsのフィルターとソートを行ってみる。step2a_dp.rs
  改善による効果有無の予想としては
    - coinsのフィルターは効果があると考える。ループ回数を減らせるので。
    - coinsのソートは効果が無い気がする。DP[amouont]まで全て計算したうえで最小を探す必要があるため。
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
            return Self::COMBINATION_NOT_FOUND;
        }
        if coins.iter().any(|v| *v < 0) {
            panic!("coins must be contain only positive values");
        }

        let mut visited_amount = vec![false; (amount + 1) as usize];
        let mut frontiers = VecDeque::new();
        let mut coins = coins
            .into_iter()
            .filter(|coin| *coin <= amount)
            .collect::<Vec<_>>();

        coins.sort();
        coins.reverse();
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
    fn step2_test() {
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
    fn step2_minus_amount_test() {
        let coins = vec![1, 2, 3, 5];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }

    #[test]
    #[should_panic]
    fn step2_have_minus_coin_test() {
        let coins = vec![1, 2, 3, -6];
        let amount = -5;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }
}
