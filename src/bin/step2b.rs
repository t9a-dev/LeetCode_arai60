// Step2b
// 目的: 別の解法を練習する

/*
  https://github.com/hayashi-ay/leetcode/pull/46#issuecomment-1986824146
  https://github.com/olsen-blue/Arai60/pull/47/changes#r2002307405
  - nに依存せずkから答えを求められる。数学パズルに近いとのことで写経だけする。

  所感
  - 上の2つの例をみて、なぜそうなるのかという数学的な証明は考えずにRustで書き換えることだけ考えながら書いたら動いた。
    - 例が何をしているのかの理解
      - k-1をbinaryで表した時に1がいくつあるかをカウントしている
      - カウントした結果の偶奇性を結果として返している
        - binary（2進数）において、LSB(一番右の最下位ビット)が1であれば奇数、0であれば偶数になる性質を利用して、0001とAND演算することによって偶奇性を確認している
      - カウントした結果の偶奇性がなぜ答えと一致するのかが数学パズルな部分だと理解
  - 偶奇性という言葉を初めて使ったので、調べたところ英語でparityということが分かった。
    - 関連してparity bit という英語は聞いたことがあったものの意味は知らなかったので調べたところ、誤り検出の文脈で利用されている英語だった。
    https://ja.wikipedia.org/wiki/%E3%83%91%E3%83%AA%E3%83%86%E3%82%A3%E3%83%93%E3%83%83%E3%83%88
  - bit演算で何かしたいという時に、こういった数学的な特性を利用してショートカットする手法が存在することを知っておくことで、必要になったときに調べながら利用できる程度で良いと思った。（今自分が関与しているレイヤーでは。）
*/

pub struct Solution {}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            panic!("n and k must be greater than 0")
        }

        ((k - 1).count_ones() & 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    #[should_panic]
    fn step2b_kth_grammar_n1_k0_panics() {
        Solution::kth_grammar(1, 0);
    }

    #[test]
    #[should_panic]
    fn step2b_kth_grammar_n0_k1_panics() {
        Solution::kth_grammar(0, 1);
    }

    #[test]
    #[should_panic]
    fn step2b_kth_grammar_n0_k0_panics() {
        Solution::kth_grammar(0, 0);
    }

    #[test]
    #[should_panic]
    fn step2b_kth_grammar_negative_n_panics() {
        Solution::kth_grammar(-1, 0);
    }

    #[test]
    #[should_panic]
    fn step2b_kth_grammar_negative_k_panics() {
        Solution::kth_grammar(0, -1);
    }
}
