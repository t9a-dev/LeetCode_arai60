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

  https://github.com/docto-rin/leetcode/pull/27/changes#diff-7621ca097a66d8850331853241ed62b900277d528a4f2ff8a3a9c56e63ea56a7R129
  - key関数を定義してbisect_leftで答えを求めている。key関数を定義する部分で抽象化が必要なので良い練習になると思った。
  Pythonのbisect_leftでrange(max_capacity)と引数に渡している部分は遅延評価なので空間計算量O(n)とはならずO(1)。
  Rustのbinary_search_byはスライスに対する操作（配列を実体化する必要がある）なので、空間計算量O(n)となるので注意が必要。
  Rustでは標準ライブラリでRangeに対する二分探索APIがない模様。
  bisectionという外部クレートを見つけたが、Rangeに対する二分探索は行えないようだった。
  IssuesやPull Requestを見ていたところ、二分探索アルゴリズムにおいて中間を計算するロジックがオーバーフローする可能性があるというIssueがあった。
  https://github.com/SteadBytes/bisection/issues/2
  コーディング練習会でもレビューで良く指摘があるのであるあるなんだなと思った。
  Issueの中でこの二分探索アルゴリズムに関するGoogle Researchの記事へのリンクが貼られており、過去にJDK用に実装された二分探索アルゴリズムでも同様のバグがあったのことで面白いと思った。
  https://research.google/blog/extra-extra-read-all-about-it-nearly-all-binary-searches-and-mergesorts-are-broken/
  ちなみに、bisectionクレートに依存しているライブラリにastral-sh/uv(Pythonパッケージ、プロジェクト管理ツール)があることに気付いた。
  uvは昨今のPythonパッケージ管理ツールとしてデファクトスタンダードになっている感じもあるので、依存しているクレートにバグが有ることを報告して実装を差し替えるなりのPull Requestチャンスかなと思って調べていたら、すでにPull Requestが行われていた。
  https://github.com/prefix-dev/async_http_range_reader/pull/18

  https://github.com/nanae772/leetcode-arai60/pull/43/changes#diff-7003a5baa3f89ae7ceafbb688dab132a20dfe083744966edff31be3fc901637bR41
  - 引数のweights.max() == 0,days = 0のケースをどうするかについて。問題の制約としてありえないことが分かっているが、コーディング練習の一環として考えてみること自体が良いことだと思った。

  https://github.com/h1rosaka/arai60/pull/46/changes
  - Pull Requestでのやり取りを理解できるかという視点で読むと良さそう。ソフトウェアエンジニアとして職場の同僚の議論を聞いていて理解できる必要があるよねという感じ。

  https://github.com/olsen-blue/Arai60/pull/44/changes#diff-4e146417f14c744a10f851601f26cd2cb17b420ff966720e568f6f5679aa475eR145
  - Pythonのbisect_leftの引数にrange(sum(weights) + 1)を渡しているが、なぜ+1しているのか少し考えた。
  rangeは引数が1つの場合にstopに対応していて、start <= i < stop な半開区間[start,stop)になるので+1することで調整していることが分かった。
  weights=[1, 2, 3]のときsum = 6となるが、range(6)とすると0 ~ 5になるので+1している。

  改善する時に考えたこと
  - 入力の制約上あり得ないがオーバーフローを考慮した実装にする。
    - weights,daysの引数チェックも行う。LeetCodeのテンプレートのシグネチャがi32なのでpanic!()しているが、自分で定義するならResultやOptionで表現すると思った。
    - Rustだとstd::num::NonZeroというものが有り、0を許容しないことを表現できるのでこれを使うのがベストそう。
    https://doc.rust-lang.org/std/num/struct.NonZero.html
    ```rust
    use std::num::NonZero;

    pub fn ship_within_days(weights: Vec<NonZero<u32>>, days: NonZero<u32>) -> Result<NonZero<u32>> {
      ...
    }
    ```
  - dayはdays_requiredの方がより分かりやすいと思った。

  所感
  - while loopの中のfor loopは関数に切り出したバージョンも書いてみる。step2a.rs
*/

pub struct Solution {}
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        if weights.is_empty() {
            return 0;
        }
        if weights.iter().any(|weight| *weight < 0) {
            panic!("weights must contain only positive values");
        }
        if days <= 0 {
            panic!("days must be greater than 0");
        }

        let mut min_capacity_of_day = *weights.iter().max().unwrap() as i64;
        let mut max_capacity_of_day = weights.iter().map(|x| *x as i64).sum::<i64>();

        while min_capacity_of_day < max_capacity_of_day {
            let capacity_of_day =
                min_capacity_of_day + (max_capacity_of_day - min_capacity_of_day) / 2;
            let mut days_required = 1;
            let mut load_of_day = 0;

            for weight in &weights {
                load_of_day += *weight as i64;

                if capacity_of_day < load_of_day {
                    days_required += 1;
                    load_of_day = *weight as i64;
                }
            }

            if days < days_required {
                min_capacity_of_day = capacity_of_day + 1;
            } else {
                max_capacity_of_day = capacity_of_day;
            }
        }

        max_capacity_of_day.try_into().expect(&format!(
            "max_capacity_of_day downcast failed. max_capacity_of_day: {}",
            max_capacity_of_day
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn step2_no_overflow_test() {
        assert_eq!(
            Solution::ship_within_days(vec![i32::MAX, i32::MAX, 5], 3),
            i32::MAX
        );
    }
}
