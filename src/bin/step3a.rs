// Step3a
// 目的: 再帰処理をループに書き換えて実装する。

/*
  n = n
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  所感
  - step3の空間計算量を見積もった時にループに書き換えるバージョンを書いていないことに気付いた。
  - 自力でループに書き換えられなかったのでGPT-5.2に聞いて写経した。

  解法の理解
  - 決定木の右の子は親の値を反転したものになる
    - 決定木の右の子はkが偶数になる性質がある
  - kが偶数のときは自身を反転した値が親の値(parent_value)になる。
*/

/*
n=3,k=4
      0
     / \
    0   1 <- parent_value=1
   / \ / \
  0  1 1  0 <- k=4
*/

pub struct Solution {}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            panic!("n and k must be greater than 0");
        }

        let mut current_n = n;
        let mut current_k = k;
        let mut parent_value = 0;

        while 1 < current_n {
            if current_k % 2 == 0 {
                parent_value = 1 - parent_value;
            }

            current_k = (current_k + 1) / 2;
            current_n -= 1;
        }

        parent_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3a_test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    #[should_panic]
    fn step3a_kth_grammar_n1_k0_panics() {
        Solution::kth_grammar(1, 0);
    }

    #[test]
    #[should_panic]
    fn step3a_kth_grammar_n0_k1_panics() {
        Solution::kth_grammar(0, 1);
    }

    #[test]
    #[should_panic]
    fn step3a_kth_grammar_n0_k0_panics() {
        Solution::kth_grammar(0, 0);
    }

    #[test]
    #[should_panic]
    fn step3a_kth_grammar_negative_n_panics() {
        Solution::kth_grammar(-1, 0);
    }

    #[test]
    #[should_panic]
    fn step3a_kth_grammar_negative_k_panics() {
        Solution::kth_grammar(0, -1);
    }
}
