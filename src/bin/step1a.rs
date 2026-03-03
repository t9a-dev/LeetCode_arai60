// Step1a
// 目的: 答えを見て解法を理解する

/*
  https://www.youtube.com/watch?v=FdzJmTCVyJU
  - NeetCodeの解法
  - 動画を見ていたら、heapを使う解法で解けそうなことに気付いた。
    - 利用中の会議室の数を会議時間の開始で+1,終了で-1する。
    - 会議が新たに開始する時に、必要な会議室の最大値を更新する。

  所感
  - この考え方で実装してAcceptedになったが、[(1, 5), (2, 6), (3, 7), (4, 8), (5, 9)]をmin_heapにpush(),pop()したときの挙動を理解しきれていないことに気付いた。
  - 会議の開始、終了時間をそれぞれtupleの１つ目の要素としてmin_heapにpushしていくと、
    - [(1,1), (2,1), (3,1), (4,1), (5,1), (5,-1) ...]になると思っていた。tupleの１つ目の要素が同一の場合はpush順で並ぶと思い込んでいたため。実際には(5,-1)の方が先に来る。
    - [(1,1), (2,1), (3,1), (4,1), (5,-1), (5,1) ...]になっている。
      - この挙動のおかげで自身の書いたロジックが動いているようにみえる。
    - 公式ドキュメントにtupleのOrd,PartialOrdでの比較時には最初に等しくない値を見つけるまでタプルの値を順番に見ていくという記述があった。
      https://doc.rust-lang.org/std/primitive.tuple.html
      > The sequential nature of the tuple applies to its implementations of various traits. For example, in PartialOrd and Ord, the elements are compared sequentially until the first non-equal set is found.
  - この点を知らない状態で意識せずに実装していたので、たまたま正解した結果になってしまった。
  - この視点で見ると実装がパズルに見える。tuple比較時の挙動を正確に把握していないとロジックが動いている理由が正確に把握できないため。
*/

use std::{cmp::Reverse, collections::BinaryHeap};

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
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut time_to_use_room_count = BinaryHeap::new();
        for interval in intervals {
            time_to_use_room_count.push(Reverse((interval.start, 1)));
            time_to_use_room_count.push(Reverse((interval.end, -1)));
        }

        let mut min_meeting_rooms_count = 0;
        let mut using_meeting_rooms_count = 0;
        while let Some(Reverse((_, use_room_count))) = time_to_use_room_count.pop() {
            using_meeting_rooms_count += use_room_count;

            let use_meeting_room = use_room_count == 1;
            if use_meeting_room {
                min_meeting_rooms_count = min_meeting_rooms_count.max(using_meeting_rooms_count);
            }
        }

        min_meeting_rooms_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_interval(interval: (i32, i32)) -> Interval {
        Interval::new(interval.0, interval.1).unwrap()
    }

    #[test]
    fn play_ground() {
        let intervals = vec![(1, 5), (2, 6), (3, 7), (4, 8), (5, 9)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        let mut min_heap = BinaryHeap::new();
        for interval in intervals {
            min_heap.push(Reverse((interval.start, 1)));
            min_heap.push(Reverse((interval.end, -1)));
        }

        assert_eq!(Reverse((1, 1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((2, 1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((3, 1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((4, 1)), min_heap.pop().unwrap());
        // tupleは1つ目の要素T1が等しい時、次の要素T2で比較する。
        // なので、5が重複すると次の要素である-1,1の比較によって順序が決まる。
        // https://doc.rust-lang.org/std/primitive.tuple.html
        assert_eq!(Reverse((5, -1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((5, 1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((6, -1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((7, -1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((8, -1)), min_heap.pop().unwrap());
        assert_eq!(Reverse((9, -1)), min_heap.pop().unwrap());
    }

    #[test]
    fn step1a_conflict_intervals_test() {
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
    fn step1a_no_conflict_intervals_test() {
        let intervals = vec![(0, 40)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);

        let intervals = vec![].into_iter().map(to_interval).collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 0);
    }

    #[test]
    fn step1a_invalid_interval_range_test() {
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
    fn step1a_invalid_interval_value_test() {
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
