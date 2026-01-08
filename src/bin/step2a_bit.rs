// Step2a
// 目的: 別の実装方法を練習する

/*
  https://github.com/Yoshiki-Iwasa/Arai60/pull/38/changes#diff-b99da78c4652531fb1dfdd24ee4608d9c18613a21307675674e3be8cddc3f746R30
  - ビット演算の実装を写経しておく。

  解法の理解
  - 注目すべき点は以下
    - if upcasted_n & 1 != 0
      - 2進数において最下位ビット(LSB)が1かどうかで偶数、奇数を判定できるという性質を利用している。
        - 0101 -> 5(10進)となり奇数なのが分かる。最下位ビットが1
        - 0100 -> 4(10進)となり偶数なのが分かる。最下位ビットが0
      - 1(2進)とのAND演算により最下位ビットが1かどうかを見ている。
    - upcasted_n >>= 1
    https://doc.rust-lang.org/std/ops/trait.Shr.html#required-methods
      - bit列を右に1つシフトしている。
        - 5(10進) -> 0000_0101(2進)
          - 右に1つシフトすると、0000_0010(2進)となる。つまり、2（10進）になることが分かる。
  所感
  - 算術シフトとGPT-5.2と壁打ちしていて知ったが、単純に高速になりそうだから算術シフト(>>)するのは危険だということが分かった。(具体的にはbit_shift_test参照)
    - 算術シフトの対象が負数の場合、signビットごとシフトするため。
  - 今回の実装では、たまたまnが負数にならないように調整しているが、この点を意識せずに n / 2 を高速化するために n >> 1 と書き換えるような一般化をすると危険。
  - 知識として算術シフトにより高速に計算できるという覚え方は良いが、かなり注意して使う必要があると理解した。
    - それなりに読み書きされる部分で使うのはかなり危ういと感じた。低いレイヤーに押し込められないコードならあまり使いたくないなという感覚。
*/

pub struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 && n <= 0 {
            panic!("x is not zero or n > 0")
        }
        if n == 0 {
            return 1.0;
        }

        let mut result = 1.0;
        let mut powered_x = x;
        let mut upcasted_n = n as i64;

        if n < 0 {
            powered_x = 1.0 / x;
            upcasted_n = upcasted_n.abs();
        }

        while 0 < upcasted_n {
            if upcasted_n & 1 == 1 {
                result *= powered_x;
            }

            powered_x *= powered_x;
            upcasted_n >>= 1;
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
    fn bit_shift_test() {
        let n = -5;
        // 符号付きの値を算術シフトすると符号ビットも移動するので注意。
        // nが負数の時、n >> 1 は n / 2 と等しくならない。
        assert_ne!(n >> 1, n / 2);
    }

    #[test]
    fn step2a_test() {
        assert_eq!(round_5(Solution::my_pow(2.00000, 10)), 1024.00000);
        assert_eq!(round_5(Solution::my_pow(2.10000, 3)), 9.26100);
        assert_eq!(round_5(Solution::my_pow(2.00000, -2)), 0.25000);
    }

    #[test]
    fn step2a_not_overflow_test() {
        assert!(Solution::my_pow(1.00000, i32::MAX).is_sign_positive());
        assert!(Solution::my_pow(1.00000, i32::MIN).is_sign_positive());
    }

    #[test]
    #[should_panic]
    fn step2a_invalid_params_panic_test() {
        assert!(Solution::my_pow(0.0, -1).is_sign_positive());
    }
}
