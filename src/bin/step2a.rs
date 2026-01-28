// Step2a
// 目的: 別の解法を練習する

/*
  https://github.com/hayashi-ay/leetcode/pull/46/changes#diff-da439603310f08640b8dab0ec6cfc15251b5669e04e4effc5795dbe1f506a8daR66
  - 決定木として考える方針で何をしているのか理解できた。
  n-1,(k+1)/2に対応する数値parent_valueを見たときに、
  parent_valueが0のとき
    kが偶数であれば1
    kが奇数であれば0
  parent_valueが1のとき
    kが偶数であれば0
    kが奇数であれば1
  といった規則があることを利用している。
  手元で決定木を書いてみたところ規則性が読み取れた。
  n=3,k=4
      0
     / \
    0   1 <- parent_value=1
   / \ / \
  0  1 1  0 <- k=4

  所感
  - 決定木として考えると分かりやすいと感じた。bit演算が登場しないからだと思った。
  - かなりすっきりと書ける。情報を圧縮しすぎて何をしているのかわかりにくいかもとも思ったが別の解法でも、解法を理解していないと一見何をしているのかわからないのは変わらないと思った。
*/

pub struct Solution {}
impl Solution {
    const BINARY_TREE_ROUTES: [[i32; 2]; 2] = [[0, 1], [1, 0]];

    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            panic!("n and k must be greater than 0")
        }

        if n == 1 || k == 1 {
            return 0;
        }

        let parent_value = Self::kth_grammar(n - 1, k - (k / 2));
        Self::BINARY_TREE_ROUTES[parent_value as usize][((k + 1) % 2) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    #[should_panic]
    fn step2a_kth_grammar_n1_k0_panics() {
        Solution::kth_grammar(1, 0);
    }

    #[test]
    #[should_panic]
    fn step2a_kth_grammar_n0_k1_panics() {
        Solution::kth_grammar(0, 1);
    }

    #[test]
    #[should_panic]
    fn step2a_kth_grammar_n0_k0_panics() {
        Solution::kth_grammar(0, 0);
    }

    #[test]
    #[should_panic]
    fn step2a_kth_grammar_negative_n_panics() {
        Solution::kth_grammar(-1, 0);
    }

    #[test]
    #[should_panic]
    fn step2a_kth_grammar_negative_k_panics() {
        Solution::kth_grammar(0, -1);
    }
}
