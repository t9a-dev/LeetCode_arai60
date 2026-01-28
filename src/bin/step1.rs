// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - ベルトコンベアに乗っている荷物の重さを表す自然数からなるweights配列と自然数のdaysが与えられる。
  daysで表される日数内で全ての荷物をベルトコンベアから船に積む時に必要な船の最小積載重量を求めて返す。
  制約としてベルトコンベアの荷物の順番は変えることができない。つまり、与えられた時点のweights[i]は変更不可。

  何を考えて解いていたか
  - 手作業でやることを考えてどこから始めるかを考える。
    - days内に全ての荷物を送るための一日あたりの荷物量は weights.len() / days で求められる。（最小積載量は無視した場合）
  手が止まったので答えを見る。

  何がわからなかったか
  - 手作業でやる場合ですらどうやれば良いかが分からなかった。

  解法の理解
  https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/solutions/3217242/javascript-rust-easy-to-understand-solut-y5lw/
  - 荷物の総重量をmax_capacityで表している。
    - 1日で全部船に積むとしたときの最小積載重量という考え方に見える。
  - 最も重い荷物をmin_capacityで表している。
    - 1日1個ずつ船に積むとしたときの最小積載重量という考え方に見える。
  - 上記の両極端なケースから区間の開始と終了を作って、区間を二分探索するという考え方に見える。
    - capacity = (min_capacity + max_capacity) / 2 があまり腑に落ちない。
      - 1日1個ずつ荷物を船に積む時の最小積載重量 ~ 1日で全ての荷物を船に積むときの最小積載重量　という区間の中間地点なのは分かる。
      - 荷物の数と日数が関連している。
        - 1日1個ずつ荷物を船に積むとき、荷物の数=日数になる。
        - 1日に全ての荷物を船に積むとき、日数=1になる。
  - 両極端なケースを考えてこの区間の間に答えが存在するように問題を再定義して二分探索をしているように見える。
  - 二分探索の条件(days < day)では何をしているか。
    - 指定された日数内で荷物を捌ききれないとき、一度に船に詰める載重量を+1して最小積載重量を増やしている。
      - 左の区間(より小さい積載重量)を捨てている。
    - 指定された日数内で荷物を捌き切れるとき、より小さい最小積載重量を探す。
      - 右の区間(より大きい積載重量)を捨てている。

  所感
  - 問題を二分探索で処理できる問題に再定義しているように見えた。
    - 問題を見て二分探索で解けることに気付けるレベルまで至っていないが、練習を続けていけば慣れるタイプのものだと思った。
  - 入力の制約としてはあり得ないが、引数のシグネチャとしてはi32の合計を求める部分はオーバーフローしうるので、i64で扱うようにした方が良いと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut max_capacity_of_day = 0;
        let mut min_capacity_of_day = 0;

        for weight in &weights {
            max_capacity_of_day += weight;
            min_capacity_of_day = min_capacity_of_day.max(*weight);
        }

        while min_capacity_of_day < max_capacity_of_day {
            let capacity = (min_capacity_of_day + max_capacity_of_day) / 2;
            let mut day = 1;
            let mut load_of_day = 0;

            for weight in &weights {
                load_of_day += weight;

                // 想定していた1日あたりの積載量を超えるので、次の日に持ち越し。
                if capacity < load_of_day {
                    day += 1;
                    load_of_day = *weight;
                }
            }

            if days < day {
                min_capacity_of_day = capacity + 1;
            } else {
                max_capacity_of_day = capacity;
            }
        }

        min_capacity_of_day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    #[should_panic]
    fn step1_overflow_test() {
        assert_eq!(
            Solution::ship_within_days(vec![i32::MAX, i32::MAX, 5], 3),
            i32::MAX
        );
    }
}
