// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = n
  時間計算量: O(log n)
  空間計算量: O(1)
*/

/*
  1回目: 3分30秒
  2回目: 3分24秒
  3回目: 3分25秒
*/

pub struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // x is not zero or n > 0
        if x == 0.0 && n <= 0 {
            panic!("x is not zero or n > 0");
        }
        if n == 0 {
            return 1.0;
        }

        let mut result = 1.0;
        let mut powered_x = x;
        let mut upcasted_n = n as i64;

        if n < 0 {
            powered_x = 1.0 / powered_x;
            upcasted_n = upcasted_n.abs();
        }

        while 0 < upcasted_n {
            if upcasted_n % 2 == 1 {
                result *= powered_x;
            }

            powered_x *= powered_x;
            upcasted_n /= 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_5(v: f64) -> f64 {
        (v * 1e5).round() / 1e5
    }

    #[test]
    fn step3_test() {
        assert_eq!(round_5(Solution::my_pow(2.00000, 10)), 1024.00000);
        assert_eq!(round_5(Solution::my_pow(2.10000, 3)), 9.26100);
        assert_eq!(round_5(Solution::my_pow(2.00000, -2)), 0.25000);
    }

    #[test]
    fn step3_not_overflow_test() {
        assert!(Solution::my_pow(1.00000, i32::MAX).is_sign_positive());
        assert!(Solution::my_pow(1.00000, i32::MIN).is_sign_positive());
    }

    #[test]
    #[should_panic]
    fn step3_invalid_params_panic_test() {
        assert!(Solution::my_pow(0.0, -1).is_sign_positive());
    }
}
