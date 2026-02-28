// Step2a
// 目的: 他の人が書いたコードを写経して理解する練習

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  https://github.com/olsen-blue/Arai60/pull/56/changes#diff-ab6cfa3e835a34ed5eb3a6822101cce7a548f12d24be2a329849b326fdc792b6R7
  - 累積和の解法。

  解法の理解
  - ミーティングが始まったら+1してミーティングが終了したら-1する
  - ミーティングが始まって(+1)次のミーティングが始まる(+1)とprefix_sumが2になるので重複していることが分かる。
  - ミーティングが始まる(+1),ミーティングが終わる(-1)のように重複がないとprefix_sumは常に1以下となり、重複していないことが分かる。

  考えていたこと
  - この解法であればintervalsをソートしないので、引数は参照で受け取れる。累積和の配列のメモリを追加で確保する必要があるというトレードオフはある。
  - 参考にしたコードでは累積和の配列サイズを初期化する部分が問題の制約に依存している（問題の制約に基づいて配列上限サイズを決め打ちしている）ので、ヒープを使ったほうがより良い気がする。

  所感
  - 最初見た時に理解するのに苦労した。
  - この問題文を見た時に最初に累積和が思いつくという点に驚いた。優劣ではなくて自分の視点の外側にあるという感想。
*/

use std::{cmp::Reverse, collections::BinaryHeap};

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
    /*
      関数のシグネチャがNeetCode採点システムに適合しません。
    */
    pub fn can_attend_meetings(intervals: &[Interval]) -> bool {
        let mut time_to_use_room_key_count = BinaryHeap::new();
        for interval in intervals {
            time_to_use_room_key_count.push(Reverse((interval.start, 1)));
            time_to_use_room_key_count.push(Reverse((interval.end, -1)));
        }

        let mut using_room_key_count = 0;
        while let Some(Reverse((_, room_key_count))) = time_to_use_room_key_count.pop() {
            using_room_key_count += room_key_count;

            let is_conflict_use_key = 2 <= using_room_key_count;
            if is_conflict_use_key {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_interval(interval: (u32, u32)) -> Interval {
        Interval::new(interval.0, interval.1).unwrap()
    }

    #[test]
    fn step2a_test() {
        let intervals = vec![(0, 30), (5, 10), (15, 20)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        let intervals = vec![(5, 8), (9, 15)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        let intervals = vec![(5, 8), (8, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        let intervals = vec![(1, 2), (2, 4), (2, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);
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
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        // single: 1件なら参加可能
        let intervals = vec![(1, 2)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        // unsorted but non-overlapping: ソートして判定できるか
        let intervals = vec![(10, 12), (1, 3), (4, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        // unsorted with overlap
        let intervals = vec![(10, 12), (1, 5), (4, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        // fully contained: 内包（(2,3) が (1,10) に含まれる）
        let intervals = vec![(1, 10), (2, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        // same start: 開始時刻が同一なら必ず重なる（endが異なる）
        let intervals = vec![(5, 7), (5, 6)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        // same interval duplicated
        let intervals = vec![(1, 2), (1, 2)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        // touching chain: 端点で接するのはOK（半開区間想定）
        let intervals = vec![(0, 1), (1, 2), (2, 3), (3, 10)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        // overlap by 1: 1だけ重なる
        let intervals = vec![(0, 2), (1, 3)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);

        // large numbers
        let intervals = vec![
            (0, 1),
            (1_000_000_000, 1_000_000_100),
            (1_000_000_100, 2_000_000_000),
        ]
        .into_iter()
        .map(to_interval)
        .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), true);

        // long range conflicts with later small range
        let intervals = vec![(0, 100), (50, 60), (100, 120)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::can_attend_meetings(&intervals), false);
    }
}
