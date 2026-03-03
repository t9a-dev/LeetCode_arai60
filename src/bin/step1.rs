// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 会議の開始時間、終了時間を表すInterval構造体からなる配列intervalsが入力として与えられるので、全ての会議が重複しないために必要な最小の会議室の数を返す。
  [(start1,end1), (start2,end2))]　のとして、[(0,3), (0,2)]　のとき必要な会議室の数は２になる。

  何を考えて解いていたか
  - 全問と同じ考え方で、会議の開始時間でソートして重複を確認する。会議時間の重複を見つけるたびに必要な会議室の数をインクリメントしていけば良さそう。
  - intervalsが空であれば0を返すエッジケースの処理が必要そう。intervalsが空でなければ会議室の数は1からスタートするので。
  n = intervals.len()
  時間計算量: O(n log n)
  空間計算量: O(1)
  重複した時に、数え上げる方向で本当に良いのか不安を感じるので、コードを書く前にできる範囲でテストケースを考えて問題なさそうか確認する。
  [(0,5)] out=1
  [(0,5), (0,2)] out=2
  [(0,5), (5,7)] out=1
  [(0,5), (0,2), (5,7)] out=2
  [(0,5), (0,2), (1,5), (5,7)] out=3
  この考え方で大丈夫そうに見える。

  提出したところ、次のテストケースでWrong Answerとなった。
  intervals=[(1,5), (2,6), (3,7), (4,8), (5,9)] actual=5 expect=4
  (1,5), (5,9)をうまく処理できていないように見える。(1,5)の会議終わりで、(5,9)が使えるのにも関わらずインクリメントしてしまっている。
  つまり、会議の開始時間でソートして隣接する会議時間を見るだけでは正しい答えが得られないようにみえる。
  他の解法が思いつかないので、step1a.rsで他の人のコードを見て解法を理解する。

  何がわからなかったか
  - 単にソートして数え上げるだけでは対応できないケースがあることを見抜けなかった。
*/

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IntervalError {
    #[error("invalid interval. end must be greater start. start: {0},  end: {1}")]
    InvalidIntervalRange(i32, i32),
    #[error("invalid interval. start and end must be greater than 0. start: {0},  end: {1}")]
    InvalidIntervalValue(i32, i32),
}

#[derive(Debug)]
pub struct Interval {
    start: i32,
    end: i32,
}
impl Interval {
    pub fn new(start: i32, end: i32) -> Result<Self, IntervalError> {
        if start < 0 || end < 0 {
            return Err(IntervalError::InvalidIntervalValue(start, end));
        }
        if end <= start {
            return Err(IntervalError::InvalidIntervalRange(start, end));
        }

        Ok(Self { start, end })
    }
}

pub struct Solution {}
impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
        /*
          このコードはWrong Answerとなるコードです。
          intervals=[(1,5), (2,6), (3,7), (4,8), (5,9)] actual=5 expect=4
        */
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_by_key(|v| v.start);

        let mut min_required_meeting_rooms_count = 1;
        for w in intervals.windows(2) {
            let (interval, next_interval) = (&w[0], &w[1]);
            if next_interval.start < interval.end {
                min_required_meeting_rooms_count += 1;
            }
        }

        min_required_meeting_rooms_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_interval(interval: (i32, i32)) -> Interval {
        Interval::new(interval.0, interval.1).unwrap()
    }

    #[test]
    fn step1_conflict_intervals_test() {
        let intervals = vec![(0, 40), (5, 10), (15, 20)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    }

    #[test]
    fn step1_no_conflict_intervals_test() {
        let intervals = vec![(0, 40)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);

        let intervals = vec![].into_iter().map(to_interval).collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 0);
    }

    #[test]
    fn step1_invalid_interval_range_test() {
        let (start, end) = (1, 0);
        assert_eq!(
            Interval::new(start, end).unwrap_err(),
            IntervalError::InvalidIntervalRange(start, end)
        );

        let (start, end) = (10, 10);
        assert_eq!(
            Interval::new(start, end).unwrap_err(),
            IntervalError::InvalidIntervalRange(start, end)
        );
    }

    #[test]
    fn step1_invalid_interval_value_test() {
        let (start, end) = (-1, 1);
        assert_eq!(
            Interval::new(start, end).unwrap_err(),
            IntervalError::InvalidIntervalValue(start, end)
        );

        let (start, end) = (-2, 0);
        assert_eq!(
            Interval::new(start, end).unwrap_err(),
            IntervalError::InvalidIntervalValue(start, end)
        );
    }
}
