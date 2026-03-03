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
  空間計算量: O(n)
*/

/*
  enum MeetingEvent,min_meeting_roomsの実装のみ
  1回目: 3分11秒
  2回目: 2分49秒
  3回目: 3分10秒
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MeetingEvent {
    End,
    Start,
}

pub struct Solution {}
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut meeting_events = intervals
            .iter()
            .flat_map(|interval| {
                vec![
                    (interval.end, MeetingEvent::End),
                    (interval.start, MeetingEvent::Start),
                ]
            })
            .collect::<Vec<_>>();
        meeting_events.sort();

        let mut min_required_rooms_count = 0;
        let mut using_rooms_count = 0;
        for (_, meeting_event) in meeting_events {
            match meeting_event {
                MeetingEvent::End => using_rooms_count -= 1,
                MeetingEvent::Start => using_rooms_count += 1,
            }
            min_required_rooms_count = min_required_rooms_count.max(using_rooms_count);
        }

        min_required_rooms_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_interval(interval: (i32, i32)) -> Interval {
        Interval::new(interval.0, interval.1).unwrap()
    }

    #[test]
    fn step3_conflict_intervals_test() {
        let intervals = vec![(1, 5), (2, 6), (3, 7), (4, 8), (5, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 4);

        let intervals = vec![(0, 40), (5, 10), (15, 20)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    }

    #[test]
    fn step3_no_conflict_intervals_test() {
        let intervals = vec![(0, 40)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);

        let intervals = vec![].into_iter().map(to_interval).collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 0);
    }

    #[test]
    fn step3_invalid_interval_range_test() {
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
    fn step3_invalid_interval_value_test() {
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
