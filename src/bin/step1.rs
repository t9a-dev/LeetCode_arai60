// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - (start,end)の情報を持つ配列intervalsが与えられる。intervalsの要素で重複する区間の有無をbooleanで返す。
  start=ミーティング開始時間、end=ミーティング終了時間を表している。intervalsはミーティング時間を表す区間を複数持っており、全てのミーティング時間で重複の有無を確認したいと理解した。
  [(0,8),(8,10)]は重複していない。

  何を考えて解いていたか
  - 採点システムがRustに対応していないのでテストケース例の入出力からテストコードを実装する。実装できたと思ったらGPT-5.2にテストケースを追加してもらってテストが通るかでAcceptedかを判定する。
  - (start,end)はleft-close,right-openな半開区間として見るのが自然そうだと思った。[(0,8), (8,10)]は重複していないので。
  - すぐに解法が思いつかないので、図に書いて考えてみる。
    - 任意の区間を基準(intervals[0])として考えて、bs(基準スタート), be(基準エンド)としたときに、bs <= e && be <= s を満たすかどうか確認していき、falseを見つけたら早期リターンで良さそう。
    - intervals[0], intervals[1]の区間が重複していなくても、intervals[1], intervals[2]が重複しているケースも見ないといけないので全ての組み合わせを確認する必要がある。
      - [(1,2), (2,4), (2,3)] では(2,4), (2,3)が重複している
    - 2重ループで外側 0..intervals.len() 内側 i+1..intervals.len()として全ての組み合わせを見ながら重複区間を見つけたら早期リターンで解けそう。
    入力の制約から時間計算量は(10 ^ 4) ^ 2 / 10 ^ 8 = 1秒 となり、遅いが現実的ではあると考える。
  自分で考えたテストケースは通ったので、GPT-5.2に生成させたテストケースを通るか確認する。
  unsorted but non-overlappingのテストケースが通らなかった。
  base_interval.start < interval.startであることが前提になっていることに気付いていなかった。この条件を満たすようにswapする処理を追加して全てのテストケースをパスした。
  改善できるかコードを確認していたところ、intervals.startで一度ソートしてしまえば、時間計算量がO(n log n)で済むことに気付いた。
  ソートしてしまえば、intervals[i] , intervals[i+1]の区間が重複していないかを確認するだけで良いため。
  step1a.rsで試してみる。

  何がわからなかったか
  - ナイーブではない実装が思いつかなかった。step2で他の人のコードを読む。

  正解してから気づいたこと
  - 最初に一度ソートすることで、時間計算量を改善できると思った。step1a.rsで試す。
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
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        for i in 0..intervals.len() {
            for j in i + 1..intervals.len() {
                let mut base_interval = &intervals[i];
                let mut interval = &intervals[j];

                if interval.start < base_interval.start {
                    std::mem::swap(&mut base_interval, &mut interval);
                }

                if base_interval.start <= interval.end && base_interval.end <= interval.start {
                    continue;
                }

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
    fn step1_test() {
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
