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
  https://github.com/skypenguins/coding-practice/pull/25
  - NeetCodeに問題があることを知った。step1.rs,step1a.rsともにAcceptedになることが確認できた。
    https://neetcode.io/problems/meeting-schedule/question

  https://github.com/tokuhirat/LeetCode/pull/55/changes#diff-b5d45bde6718c036bbfcde726bed8503a608ff8f9d0836c44647b97fbb5d9fb0R4
    > 終了時刻が早い順に見ていき、それより早く開始する会議があれば参加できない。
  - 問題を自分とは異なる始点から捉えているのが面白いと思った。アルゴリズムを考える時に、自然言語で別の言い方にすると(問題設定をより洗練する？)コードがよりシンプルになるという例に見える。

  https://github.com/tokuhirat/LeetCode/pull/55/changes#r2286546596
  - 自分は変数の主役を考えるのが面倒なので、数直線上の並びになるように　< を使うのが好きなので好みの部分だなと思った。ただし、この方針で否定形を使わないと行けない場合は否定形を使わなくて済む記述を優先するなと思った。

  https://github.com/olsen-blue/Arai60/pull/56/changes#diff-ab6cfa3e835a34ed5eb3a6822101cce7a548f12d24be2a329849b326fdc792b6R7
  - 累積和は思いつかなかったので、他の人の書いたコードを読む練習に良さそう。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/60/changes#diff-7e5d8cbda3fd172d8f65764fe82e6ce4fe0ddef03db2dd8e8569cc620967d8b0R30
  - sort_by_keyメソッドで必要十分でシンプルなのでこちらのほうが良いと思った。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/60/changes#diff-62432b9fe4023e71399f571d3ac42125fd73cfbeded458f8b97d12fc5215016fR23
  - 自分のstep1a.rsのコードはこのコードとやっていることは同じだなと思った。all()メソッドについてもfalseを見つけたら早期リターンしてくれる。
    https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
      > all() is short-circuiting; in other words, it will stop processing as soon as it finds a false, given that no matter what else happens, the result will also be false.

  https://github.com/hayashi-ay/leetcode/pull/59/changes#diff-ab6cfa3e835a34ed5eb3a6822101cce7a548f12d24be2a329849b326fdc792b6R21
  - Pythonのドキュメントには結構ラフなことも書いてあって面白いと思った。

  https://github.com/hayashi-ay/leetcode/pull/59/changes#diff-ab6cfa3e835a34ed5eb3a6822101cce7a548f12d24be2a329849b326fdc792b6R24
  - heapを使った解法は思いつかなかった。ソートしたいという目的には一致していると思った。ある時点のスケジュールから重複が存在するかを確認する用途ではソートの方が良いと思った。
  ScheduleManagerのような構造体でschedulesを状態として扱うような場合はBinaryHeapが適していそうだと思った。

  改善する時に考えたこと
  - ソート済みであることを考えると、if interval.start <= next_interval.end && interval.end <= next_interval.start は素直だが冗長でもっと短く書ける。
  肯定形であれば、if interval.end <= next_interval.start then true
  - sort_by_key(),window(),all()メソッドを活用してスッキリと書ける。

  所感
  - 最初 w.first().unwrap(),w.last().unwrap()と書いていたが、どっちにしろunwrap()でエラー処理しないなら結果は変わらないので&w[0],&w[1]に書き換えた。
  windowsメソッドの引数から範囲外アクセスにならないのは自明であるため。
*/

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IntervalError {
    #[error("invalid interval. end must be greater start. start: {0}, end: {1}")]
    InvalidInterval(u32, u32),
}

#[derive(Debug)]
pub struct Interval {
    start: u32,
    end: u32,
}
impl Interval {
    pub fn new(start: u32, end: u32) -> Result<Self, IntervalError> {
        if end <= start {
            return Err(IntervalError::InvalidInterval(start, end));
        }

        Ok(Self { start, end })
    }
}

pub struct Solution {}
impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|v| v.start);
        intervals.windows(2).all(|w| {
            let (interval, next_interval) = (&w[0], &w[1]);
            interval.end <= next_interval.start
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_interval(interval: (u32, u32)) -> Interval {
        Interval::new(interval.0, interval.1).unwrap()
    }

    #[test]
    fn step2_test() {
        let intervals = vec![(0, 30), (5, 10), (15, 20)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        let intervals = vec![(5, 8), (9, 15)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        let intervals = vec![(5, 8), (8, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        let intervals = vec![(1, 2), (2, 4), (2, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);
    }

    #[test]
    fn invalid_interval_test() {
        let (start, end) = (1, 0);
        let error = Interval::new(start, end).unwrap_err();
        assert_eq!(error, IntervalError::InvalidInterval(start, end));

        let (start, end) = (1, 1);
        let error = Interval::new(start, end).unwrap_err();
        assert_eq!(error, IntervalError::InvalidInterval(start, end));
    }

    // GPT-5.2によって生成
    #[test]
    fn additional_cases_test() {
        // empty: meetings無しなら参加可能（仕様として自然）
        let intervals = vec![].into_iter().map(to_interval).collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        // single: 1件なら参加可能
        let intervals = vec![(1, 2)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        // unsorted but non-overlapping: ソートして判定できるか
        let intervals = vec![(10, 12), (1, 3), (4, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        // unsorted with overlap
        let intervals = vec![(10, 12), (1, 5), (4, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        // fully contained: 内包（(2,3) が (1,10) に含まれる）
        let intervals = vec![(1, 10), (2, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        // same start: 開始時刻が同一なら必ず重なる（endが異なる）
        let intervals = vec![(5, 7), (5, 6)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        // same interval duplicated
        let intervals = vec![(1, 2), (1, 2)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        // touching chain: 端点で接するのはOK（半開区間想定）
        let intervals = vec![(0, 1), (1, 2), (2, 3), (3, 10)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        // overlap by 1: 1だけ重なる
        let intervals = vec![(0, 2), (1, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);

        // large numbers
        let intervals = vec![
            (0, 1),
            (1_000_000_000, 1_000_000_100),
            (1_000_000_100, 2_000_000_000),
        ]
        .into_iter()
        .map(to_interval)
        .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), true);

        // long range conflicts with later small range
        let intervals = vec![(0, 100), (50, 60), (100, 120)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(intervals), false);
    }
}
