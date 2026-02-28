// Step1a
// 目的: step1のコードから思いついた改善を試す。

/*
  改善するときに考えていたこと
  - intervalsを可変参照では受け取らないことにした。
    - ミーティング時間に重複が無いかを確認したいだけなのに、可変参照で受け取るとミーティング時間などの中身まで変更されるかもしれないという不安を呼び出し側に与えそうなのでやめた。

  n = intervals.len()
  時間計算量: O(n log n) ソートが支配的
  空間計算量: O(1)
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
        // intervalsは呼び出し側からmoveされており、呼び出し側に影響を与えない。
        // 呼び出し側はintervals.clone()するか、そのままmoveするかを選べる。
        intervals.sort_by(|a, b| a.start.cmp(&b.start));

        for i in 0..intervals.len() {
            let Some(next_interval) = intervals.get(i + 1) else {
                break;
            };
            let interval = &intervals[i];

            if interval.start <= next_interval.end && interval.end <= next_interval.start {
                continue;
            }

            return false;
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
    fn step1a_test() {
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
