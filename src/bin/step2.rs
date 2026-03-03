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
  https://github.com/olsen-blue/Arai60/pull/57#discussion_r2030075157
  - 座標圧縮という知らない用語が出てきた。
    https://drken1215.hatenablog.com/entry/2021/08/09/235400
    数列のそれぞれの要素が数列の中で何番目に小さいかということを求める作業のことを「座標圧縮」という。
    step1a.rsの解法では、(会議の開始時間,1), (会議の終了時間,-1) とすることで二次元座標圧縮しているように見える。
    こうやって見ると、Rustのtupleが１つ目の要素が等しい時に次の要素で比較するのは自然に見える。(5,1), (5,-1)のとき、5は等しいので次の値である1, -1で比較する。

  https://github.com/olsen-blue/Arai60/pull/57/changes#diff-a0ae933995d3a32d66b233c1e96d7f1bbe7ff33e80eb0997d04a4806ba5d2be5R122
  - step1a.rsで自分が感じたtupleの暗黙的な並び替えの動きについては、heapを使わずにsort系のメソッドでpredicateを記述すれば明示的になるので分かりやすくなるなと思った。

  https://github.com/nittoco/leetcode/pull/45#discussion_r1996403980
    > https://github.com/nittoco/leetcode/pull/45/changes/BASE..b5bdf0b884b8a18e951161ff3a59fe94d73b6c6a#diff-f58a8d93c7990227607003cbdcf9ac9bc8c009c853befff8a9f204036e1ce249R3
  - step1a.rsの自分と同じようなことを考えている人がいた。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/61/changes#diff-dc5acf3ef0f2cf3ed60df746eb0a49f46bd5e823ef000d992cd0c1ba7d9f34c0R8
  - (start,1), (end,-1)の1と-1はマジックナンバーなので、Enumで定義するとより分かりやすくなっていて良いと思った。

  https://github.com/ryosuketc/leetcode_arai60/pull/56/changes#diff-ef32d0d747c317926cc64728bede34c3076b19b0207cb57561ab021fed93cc3fR45
  - このコードを読んでいて気付いたが、step1a.rsの自分のコードでif use_meeting_room は無くても動く。endの方が先に来ることを理解していなかったためだと思った。

  https://github.com/Satorien/LeetCode/pull/55#discussion_r2658899716
  - RustのBinaryHeapのドキュメントを確認したところ、push()は最悪ケース(capacityの伸長)で時間計算量O(n)となるので、今回の問題のようにsort()で事足りるのであれば、闇雲にheapを使うべきではないと思った。
  sort()を利用しておけば時間計算量O(n log n)になることが分かりやすくシンプルなので。

  step1a.rsから改善する時に考えたこと
  - min_meeting_rooms_count -> min_required_rooms_countの方が良いと思った。
    - requiredを含めたほうが「必要な最小の」という意味が分かりやすいと思った。
    - meetingはなくても良いかなと思って削った。部屋の中で何をするのかは関係なくて必要な部屋の数がわかればよいので。
  - BinaryHeapを使うのを止めて、sort()とfor-loopで書く。
  - 会議の開始、終了に対応する1,-1がマジックナンバーなのでEnumで表す。
  - 不要な条件分岐をなくす。

  所感
  - Enumを活用することで利用中の会議室の増減とイベントがひと目見て分かるようになり読みやすくなったと思う。
  - MeetingEventでEndの方が小さくなることをEnumの定義順で決めているところが分かりづらい気がするものの代替案は特に思い浮かばない。
    - MeetingEvent::Startを先頭に書いてPartialOrdトレイトを手動で実装してもボイラープレートなコードが増えるだけでかえって読みづらくなりそう。
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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
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
            };
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
    fn step2_conflict_intervals_test() {
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
    fn step2_no_conflict_intervals_test() {
        let intervals = vec![(0, 40)]
            .into_iter()
            .map(to_interval)
            .collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);

        let intervals = vec![].into_iter().map(to_interval).collect::<Vec<_>>();
        assert_eq!(Solution::min_meeting_rooms(intervals), 0);
    }

    #[test]
    fn step2_invalid_interval_range_test() {
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
    fn step2_invalid_interval_value_test() {
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
