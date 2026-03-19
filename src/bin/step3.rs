// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = s.chars().count()
  時間計算量: O(n)
  空間計算量: O(n)
*/

/*
  1回目: 5分18秒
  2回目: 3分15秒
  3回目: 2分54秒
*/

/*
  所感
  - 行の移動方向制御はフラグ管理の実装にした。少し冗長になるものの、iが単調増加・減少することが一目で分かるので読みやすいという感覚。
*/

pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows
            .try_into()
            .expect("num_rows must be positive value");

        if num_rows == 0 {
            return "".to_string();
        }
        if num_rows == 1 || s.chars().count() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut is_down_direction = true;
        let mut i = 0;
        for c in s.chars() {
            rows[i].push(c);

            if is_down_direction {
                i += 1;
            } else {
                i -= 1;
            }

            if i == 0 || i == num_rows - 1 {
                is_down_direction = !is_down_direction;
            }
        }

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn step3_num_rows_negative_value_test() {
        Solution::convert("PAYPALISHIRING".to_string(), -3);
    }

    #[test]
    fn step3_test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 1),
            "PAYPALISHIRING"
        );

        assert_eq!(Solution::convert("P".to_string(), 3), "P");
    }
}
