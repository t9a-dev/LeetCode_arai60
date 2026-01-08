// Step1a
// 目的: 別の実装方法を練習

/*
  所感
  - ループにすると直感的でないと感じる。
    - 再帰処理の方では half = my_pow(x, n / 2)としているところが式に対応しているように見えるので分かりやすく感じる。
*/

pub struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let (mut x, mut n) = (x, n as i64);
        if n < 0 {
            x = 1.0 / x;
            n = n.abs();
        }

        let mut result = 1.0;
        while 0 < n {
            // x^4 = (x^2)^2
            // x^5 = (x^2)^2 * x
            if n % 2 == 1 {
                result *= x;
            }
            x *= x;
            n = n / 2;
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
    fn step1a_test() {
        assert_eq!(round_5(Solution::my_pow(2.00000, 10)), 1024.00000);
        assert_eq!(round_5(Solution::my_pow(2.10000, 3)), 9.26100);
        assert_eq!(round_5(Solution::my_pow(2.00000, -2)), 0.25000);
    }

    #[test]
    fn step1a_not_overflow_test() {
        assert!(Solution::my_pow(1.00000, i32::MAX).is_sign_positive());
        assert!(Solution::my_pow(1.00000, i32::MIN).is_sign_positive());
    }
}
