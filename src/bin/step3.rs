// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = intervals.len()
  時間計算量: O(n log n)
  空間計算量: O(1)
*/

/*
  can_attend_meetingsの実装のみの時間
  1回目: 1分43秒
  2回目: 1分03秒
  3回目: 0分56秒
*/

/*
  所感
  - 書籍「Effective Rust」で紹介されていたthiserrorクレートを試せたのが良かった。
    - NeetCodeに問題が存在していてRust言語でも採点システムを通せることを途中まで知らなかったので、採点システムの制限を受けずにある程度自由に書けたのが良かった。
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
    fn step3_test() {
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
