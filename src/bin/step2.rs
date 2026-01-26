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
  コメント集、他の人のコードを読んで考えたこと
  https://github.com/TORUS0818/leetcode/pull/47/changes#r2031685187
  - 変数を破壊しない。呼び出し側に影響を与えないようにではなく、可読性をあげるという観点

  https://github.com/hroc135/leetcode/pull/43#discussion_r2002294824
    > この再帰をループに直すのは、自然に見えて(あまり違いがないように見えて)欲しいです。
  - 自分もループに書き換えるのに少し手間取ったし、あまり自然に見えなかったので変数をimmutableにする方針で書いてみてどう感じるかを試す。

  https://github.com/TORUS0818/leetcode/pull/47/changes#diff-91647df59bb2863e62120bcb064c143cab9cb1c4e78ed9feab30fc009844b5d0R153
  https://github.com/Yoshiki-Iwasa/Arai60/pull/38/changes#diff-b99da78c4652531fb1dfdd24ee4608d9c18613a21307675674e3be8cddc3f746R30
  - ビットシフトを利用している。実装の幅はあまり無いと思っていたが意外とあった。ビットシフトを利用した実装も折角なので書いておきたい。

  https://github.com/hroc135/leetcode/pull/43#discussion_r2002298814
  - IEEE-754の内部ビットの数について。何を言っているのかわからないので調べてみる。
  Rustのf64型はIEEE 754-2008で定義されているbinary64を実装していると明記されている。
  https://doc.rust-lang.org/std/primitive.f64.html
    > A 64-bit floating-point type (specifically, the “binary64” type defined in IEEE 754-2008).
  浮動小数点方式の仕様。ビット列として表現する時に、bitをどのように使って表現するかを規定している。
  十進法を単精度で表現する例が分かりやすい。
  https://ja.wikipedia.org/wiki/IEEE_754#:~:text=%E4%BE%8B,-%5B%E7%B7%A8%E9%9B%86
  exponentビットがbinary32単精度のとき8bitとなり、binary64倍精度のとき11bitになる点を覚えておくと、符号は1ビットで表されることから残りのビットであるfractionが求められると理解した。
  sign: 1bit
  exponent: 8bit(binary32), 11bit(binary64)
  fraction: 23bit(binary32), 52bit(binary64)

  https://github.com/h1rosaka/arai60/pull/47#discussion_r2658664769
  - 問題の制約に違反するような入力値のチェックを行う。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/38#issuecomment-2272528081
  - RustにおけるPowの実装について。コンパイラ基盤であるLLVMの関数を使っているらしい。
  https://ja.wikipedia.org/wiki/LLVM

  改善する時に考えたこと
  - 変数を変更しながら計算するのではなく、別の変数として扱う。
  - 問題の制約ではxが0でないか、n>0であると定義されているので、この条件を満たさないケースをpanicさせる。
    - 前提となる仕様を満たさない入力値を処理してしまうよりはpanic!した方が良いだろうという感覚。(実務ではResult型でErrを返すなどする。)

  所感
  - x *= x, n = n / 2 よりは変数名でどのような性質、変更を加えられたものなのかが分かるので可読性は向上したと感じた。
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

        let mut powered_x = x;
        let mut upcasted_n = n as i64;
        let mut result = 1.0;

        if n < 0 {
            powered_x = 1.0 / x;
            upcasted_n = upcasted_n.abs();
        }

        while 0 < upcasted_n {
            if upcasted_n % 2 == 1 {
                result *= powered_x;
            }

            powered_x *= powered_x;
            upcasted_n = upcasted_n / 2;
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
    fn step2_test() {
        assert_eq!(round_5(Solution::my_pow(2.00000, 10)), 1024.00000);
        assert_eq!(round_5(Solution::my_pow(2.10000, 3)), 9.26100);
        assert_eq!(round_5(Solution::my_pow(2.00000, -2)), 0.25000);
    }

    #[test]
    fn step2_not_overflow_test() {
        assert!(Solution::my_pow(1.00000, i32::MAX).is_sign_positive());
        assert!(Solution::my_pow(1.00000, i32::MIN).is_sign_positive());
    }

    #[test]
    #[should_panic]
    fn step2_invalid_params_panic_test() {
        assert!(Solution::my_pow(0.0, -1).is_sign_positive());
    }
}
