// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n は weights.sum() - weights.max() だが、Big-O記法において定数項は無視する。
  n = weights.sum()
  m = weights.len()
  時間計算量: O(m log n)
  空間計算量: O(1)
*/

/*
  1回目: 8分28秒
  2回目: 8分3秒
  3回目: 分秒 例外処理を書いている時間が長いのでスキップ
*/

/*
  所感:
  - step3を書いた時に変数名days_requiredを無意識にrequired_daysにしていた。GPT-5.2に聞いてみたところ、required_daysの方が自然だと言うことだったのでそのままにした。
  - lower_capacity,upper_capacityの初期化をbefore->afterに変更した。
    - 問題の文脈からして船の最小積載重量ロジックは頻繁に呼び出されると判断。
    - 頻繁に仕様変更が発生して読み書きされる部分では無いと判断。
    - 以上の理由から人によって頻繁に読み書きされず、頻繁に呼び出されそうなメソッドなので可読性を少し犠牲にして一度の走査で計算する方向にした。

  ```rust
  // before
  let weights_iter = weights.iter().copied().map(i64::from);
  let mut lower_capacity = weights_iter.clone().max().unwrap() as i64;
  let mut upper_capacity = weights_iter.sum::<i64>();

  //after
  let (mut lower_capacity, mut upper_capacity) = weights
      .iter()
      .map(|x| i64::from(*x))
      .fold((i64::MIN, 0i64), |(lower_capacity, upper_capacity), x| {
          (lower_capacity.max(x), upper_capacity.add(x))
      });
  ```
*/

use std::ops::Add;

pub struct Solution {}
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        if weights.is_empty() {
            return 0;
        }
        if weights.iter().any(|x| *x < 0) {
            panic!("weights must contain only positive values");
        }
        if days <= 0 {
            panic!("days must be greater than 0");
        }

        let (mut lower_capacity, mut upper_capacity) = weights
            .iter()
            .map(|x| i64::from(*x))
            .fold((i64::MIN, 0i64), |(lower_capacity, upper_capacity), x| {
                (lower_capacity.max(x), upper_capacity.add(x))
            });

        while lower_capacity < upper_capacity {
            let middle_capacity = lower_capacity + (upper_capacity - lower_capacity) / 2;
            let required_days =
                Self::required_days_for_shipping(&weights, middle_capacity).unwrap();

            if days < required_days {
                lower_capacity = middle_capacity + 1;
            } else {
                upper_capacity = middle_capacity;
            }
        }

        lower_capacity.try_into().expect(&format!(
            "lower_capacity downcast failed. lower_capacity: {lower_capacity}"
        ))
    }

    fn required_days_for_shipping(weights: &[i32], daily_capacity: i64) -> Result<i32, String> {
        let mut required_days = 1;
        let mut load_of_day = 0;

        for weight in weights {
            let weight = i64::from(*weight);

            if daily_capacity < weight {
                return Err(format!("weight over daily_capacity. weight: {weight}, daily_capacity: {daily_capacity}"));
            }

            if daily_capacity < load_of_day + weight {
                required_days += 1;
                load_of_day = 0;
            }
            load_of_day += weight;
        }

        Ok(required_days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn step3_no_overflow_test() {
        assert_eq!(
            Solution::ship_within_days(vec![i32::MAX, i32::MAX, 5], 3),
            i32::MAX
        );
    }

    #[test]
    fn step3_required_days_for_shipping_test() {
        assert_eq!(
            Solution::required_days_for_shipping(&vec![1, 2, 3], 4).unwrap(),
            2
        );
        assert_eq!(
            Solution::required_days_for_shipping(&vec![1, 2, 3], 10).unwrap(),
            1
        );
        assert_eq!(
            Solution::required_days_for_shipping(&vec![5], 5).unwrap(),
            1
        );
        assert_eq!(
            Solution::required_days_for_shipping(&vec![3, 3, 3], 3).unwrap(),
            3
        );
    }

    #[test]
    fn step3_required_days_for_shipping_over_daily_capacity_test() {
        // 1日あたりの積載重量上限を超える荷物があり運べないのでエラー。
        let daily_capacity = 1;
        assert!(Solution::required_days_for_shipping(&vec![1, 2, 3], daily_capacity).is_err());
    }
}
