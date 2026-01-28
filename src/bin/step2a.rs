// Step2a
// 目的:　別の実装方法で練習する

/*
  改善する時に考えたこと
  - 全体的に変数名がしっくり来なかったのでGPT-5.2に相談しつつ変更した。
  - オーバーフロー対策でi64にアップキャストしている部分はGPT-5.2に提案されたコードの中身を精査して問題なさそうだったのでcopiedアダプタを用いた書き方にした。

  所感
  - copiedアダプタは見たことがあったものの使い所を知らなかったが、GPT-5.2の提案するコードに含まれていたので調べたところ新しいことが学べたので良かった。
*/

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
            panic!("days must be greater than 0")
        }

        // iter.copied()はcopiedアダプタと呼ばれているもの。iterの要素が全てCopy型である場合に利用可能。
        // 意図として iter().map(|x| i64::from(*x)) を iter().copied().map(i64::from)とするためにcopiedアダプタを間に入れて参照ではなくコピーした値をmapに渡している。
        // weight.into_iter()とするとweightsの所有権がムーブし、以降weightsが使えなくなるので、参照 -> copiedアダプタ としている。
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied
        // 「プログラミングRust 第2版」 P.84, P.342 あたりを参照。
        let mut lower_capacity = weights.iter().copied().max().unwrap() as i64;
        let mut upper_capacity = weights.iter().copied().map(i64::from).sum::<i64>();

        while lower_capacity < upper_capacity {
            let middle_capacity = lower_capacity + (upper_capacity - lower_capacity) / 2;
            let required_days = Self::required_days_for_shipping(&weights, middle_capacity)
                .expect("calculate required_days_for_shipping failed");

            if days < required_days {
                lower_capacity = middle_capacity + 1;
            } else {
                upper_capacity = middle_capacity;
            }
        }

        lower_capacity.try_into().expect(&format!(
            "lower_capacity downcast failed. lower_capacity: {}",
            lower_capacity
        ))
    }

    fn required_days_for_shipping(weights: &[i32], daily_capacity: i64) -> Result<i32, String> {
        let mut days_required = 1;
        let mut load_of_day = 0;

        for weight in weights {
            let weight = i64::from(*weight);
            if daily_capacity < weight {
                return Err(format!("weight over daily_capacity. weight: {weight}, daily_capacity: {daily_capacity}"));
            }

            // 積載量上限を超えるなら次の日に積む。
            if daily_capacity < load_of_day + weight {
                days_required += 1;
                load_of_day = 0;
            }
            load_of_day += weight;
        }

        Ok(days_required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn step2a_no_overflow_test() {
        assert_eq!(
            Solution::ship_within_days(vec![i32::MAX, i32::MAX, 5], 3),
            i32::MAX
        );
    }

    #[test]
    fn step2a_required_days_for_shipping_test() {
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
    fn step2a_required_days_for_shipping_over_daily_capacity_test() {
        // 1日あたりの積載重量上限を超える荷物があり運べないのでエラー。
        let daily_capacity = 1;
        assert!(Solution::required_days_for_shipping(&vec![1, 2, 3], daily_capacity).is_err());
    }
}
